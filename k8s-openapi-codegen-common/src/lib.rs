#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::doc_markdown,
	clippy::let_and_return,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::similar_names,
	clippy::struct_excessive_bools,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unseparated_literal_suffix,
	clippy::use_self,
)]

//! This crate contains common code for the [`k8s-openapi` code generator](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-codegen)
//! and the [`k8s-openapi-derive`](https://crates.io/crates/k8s-openapi-derive) custom derive crate.
//!
//! It can be used by code generators that want to generate crates like `k8s-openapi` and `k8s-openapi-derive` for Kubernetes-like software
//! such as OpenShift.
//!
//! 1. Create a [`swagger20::Spec`] value, either by deserializing it from an OpenAPI spec JSON file or by creating it manually.
//! 1. Invoke the [`run`] function for each definition in the spec.
//! 1. For each left-over API operations, ie those operations that weren't associated with any definition, invoke the [`write_operation`] function.

pub mod swagger20;

mod templates;

/// Statistics from a successful invocation of [`run`]
#[derive(Clone, Copy, Debug)]
pub struct RunResult {
	pub num_generated_structs: usize,
	pub num_generated_type_aliases: usize,
	pub num_generated_apis: usize,
}

/// Error type reported by [`run`] and [`write_operation`]
#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl From<&'_ str> for Error {
	fn from(err: &'_ str) -> Self {
		Error(err.into())
	}
}

impl From<String> for Error {
	fn from(err: String) -> Self {
		Error(err.into())
	}
}

impl From<std::fmt::Error> for Error {
	fn from(err: std::fmt::Error) -> Self {
		Error(err.into())
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error(err.into())
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::error::Error for Error {
}

/// A mechanism for converting (the components of) an openapi path to (the components of) a Rust namespace.
///
/// The k8s-openapi code generator uses this trait to emit the paths of all types relative to the `k8s_openapi` crate.
/// For example, it maps the components of the openapi path `io.k8s.api.core.v1` to
/// the components of the Rust namespace `crate::core::v1`. The `io.k8s.` prefix is stripped and the path starts with `crate::`.
///
/// Other code generators can have more complicated implementations. For example, an OpenShift code generator that has its own types but also wants to
/// reuse types from the k8s-openapi crate would map `com.github.openshift.` to `crate::` and `io.k8s.` to `k8s_openapi::` instead.
///
/// The implementation should return `None` for paths that it does not recognize.
pub trait MapNamespace {
	fn map_namespace<'a>(&self, path_parts: &[&'a str]) -> Option<Vec<&'a str>>;
}

/// Used to create an impl of `std::io::Write` for each type that the type's generated code will be written to.
pub trait RunState {
	/// The impl of `std::io::Write` for each type that the type's generated code will be written to.
	type Writer: std::io::Write;

	/// Returns an impl of `std::io::Write` for each type that the type's generated code will be written to.
	///
	/// # Parameters
	///
	/// - `parts`: A list of strings making up the components of the path of the generated type. Code generators that are emitting crates
	///   can use this parameter to make module subdirectories for each component, and to emit `use` statements in the final module's `mod.rs`.
	///
	/// - `type_feature`: The name of the Rust feature that should be used to `cfg`-gate this type as a whole, if any.
	///   Code generators that are emitting modules can use this flag to emit a `#[cfg(feature = "<this value>")]` on the `use` statement
	///   for the generated type in the module's `mod.rs`.
	fn make_writer(
		&mut self,
		parts: &[&str],
		type_feature: Option<&str>,
	) -> std::io::Result<Self::Writer>;

	/// This function is invoked with the names of the optional parameters type and result type for each operation generated for a particular type.
	///
	/// Code generators that are emitting modules can write out `use` lines in the module's `mod.rs` for each of these types.
	fn handle_operation_types(
		&mut self,
		operation_optional_parameters_name: Option<&str>,
		operation_result_name: Option<&str>,
	) -> std::io::Result<()>;

	/// This function is invoked when `k8s_openapi_codegen_common::run` is done with the writer and completes successfully.
	/// The implementation can do any cleanup that it wants here.
	fn finish(&mut self, writer: Self::Writer);
}

impl<T> RunState for &'_ mut T where T: RunState {
	type Writer = <T as RunState>::Writer;

	fn make_writer(
		&mut self,
		parts: &[&str],
		type_feature: Option<&str>,
	) -> std::io::Result<Self::Writer> {
		(*self).make_writer(parts, type_feature)
	}

	fn handle_operation_types(
		&mut self,
		operation_optional_parameters_name: Option<&str>,
		operation_result_name: Option<&str>,
	) -> std::io::Result<()> {
		(*self).handle_operation_types(operation_optional_parameters_name, operation_result_name)
	}

	fn finish(&mut self, writer: Self::Writer) {
		(*self).finish(writer);
	}
}

/// Whether [`run`] should generate an impl of [`schemars::JsonSchema`] for the type or not.
#[derive(Clone, Copy, Debug)]
pub enum GenerateSchema<'a> {
	Yes {
		/// An optional feature that the impl of [`schemars::JsonSchema`] will be `cfg`-gated by.
		feature: Option<&'a str>,
	},

	No,
}

/// Each invocation of this function generates a single type specified by the `definition_path` parameter along with its associated API operation functions.
///
/// # Parameters
///
/// - `definitions`: The definitions parsed from the OpenAPI spec that should be emitted as model types.
///
/// - `operations`: The list of operations parsed from the OpenAPI spec that should be emitted as API functions.
///   Note that this value will be mutated to remove the operations that are determined to be associated with the type currently being generated.
///
/// - `definition_path`: The specific definition path out of the `definitions` collection that should be emitted.
///
/// - `map_namespace`: An instance of the [`MapNamespace`] trait that controls how OpenAPI namespaces of the definitions are mapped to rust namespaces.
///
/// - `vis`: The visibility modifier that should be emitted on the generated code.
///
/// - `operation_feature`: If specified, all API functions will be emitted with a `#[cfg(feature = "<this value>")]` attribute.
///    The attribute will also be applied to their optional parameters and response types, if any, and to common types for
///    optional parameters and response types that are shared by multiple operations.
///
/// - `state`: See the documentation of the [`RunState`] trait.
pub fn run(
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	operations: &mut Vec<swagger20::Operation>,
	definition_path: &swagger20::DefinitionPath,
	map_namespace: &impl MapNamespace,
	vis: &str,
	generate_schema: GenerateSchema<'_>,
	operation_feature: Option<&str>,
	mut state: impl RunState,
) -> Result<RunResult, Error> {
	use std::io::Write;

	let definition = definitions.get(definition_path).ok_or_else(|| format!("definition for {} does not exist in spec", definition_path))?;

	let local = map_namespace_local_to_string(map_namespace)?;

	let mut run_result = RunResult {
		num_generated_structs: 0,
		num_generated_type_aliases: 0,
		num_generated_apis: 0,
	};

	let path_parts: Vec<_> = definition_path.split('.').collect();
	let namespace_parts: Vec<_> =
		map_namespace.map_namespace(&path_parts).ok_or_else(|| format!("unexpected path {:?}", definition_path))?
		.into_iter()
		.collect();

	let type_feature =
		if let swagger20::SchemaKind::Ty(
			swagger20::Type::CreateOptional(_) |
			swagger20::Type::DeleteOptional(_) |
			swagger20::Type::ListOptional(_) |
			swagger20::Type::PatchOptional(_) |
			swagger20::Type::ReplaceOptional(_) |
			swagger20::Type::WatchOptional(_) |
			swagger20::Type::CreateResponse |
			swagger20::Type::DeleteResponse |
			swagger20::Type::ListResponse |
			swagger20::Type::PatchResponse |
			swagger20::Type::ReplaceResponse |
			swagger20::Type::WatchResponse
		) = &definition.kind {
			operation_feature
		}
		else {
			None
		};

	let mut out = state.make_writer(&namespace_parts, type_feature)?;

	let type_name = path_parts.last().ok_or_else(|| format!("path for {} has no parts", definition_path))?;

	let derives = get_derives(&definition.kind, definitions, map_namespace)?;

	templates::type_header::generate(
		&mut out,
		definition_path,
		definition.description.as_deref(),
		type_feature,
		derives,
		vis,
	)?;

	match &definition.kind {
		swagger20::SchemaKind::Properties(properties) => {
			let (template_properties, resource_metadata, metadata_ty) = {
				let mut result = Vec::with_capacity(properties.len());

				let mut single_group_version_kind = match &definition.kubernetes_group_kind_versions[..] {
					[group_version_kind] => Some((group_version_kind, false, false)),
					_ => None,
				};

				let mut metadata_ty = None;

				for (name, (schema, required)) in properties {
					if name.0 == "apiVersion" {
						if let Some((_, has_api_version, _)) = &mut single_group_version_kind {
							*has_api_version = true;
							continue;
						}
					}

					if name.0 == "kind" {
						if let Some((_, _, has_kind)) = &mut single_group_version_kind {
							*has_kind = true;
							continue;
						}
					}

					let field_name = get_rust_ident(name);

					let mut field_type_name = String::new();

					let required = match required {
						true => templates::PropertyRequired::Required {
							is_default: is_default(&schema.kind, definitions, map_namespace)?,
						},
						false => templates::PropertyRequired::Optional,
					};

					if let templates::PropertyRequired::Optional = required {
						field_type_name.push_str("Option<");
					}

					let type_name = get_rust_type(&schema.kind, map_namespace)?;

					if name.0 == "metadata" {
						metadata_ty = Some((type_name.clone(), required));
					}

					// Fix cases of infinite recursion
					if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &schema.kind {
						match (&**definition_path, &**name, &**path) {
							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
								"not",
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps",
							) |
							(
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps",
								"not",
								"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps",
							) => {
								field_type_name.push_str("Box<");
								field_type_name.push_str(&type_name);
								field_type_name.push('>');
							},

							_ => field_type_name.push_str(&type_name),
						}
					}
					else {
						field_type_name.push_str(&type_name);
					};

					if let templates::PropertyRequired::Optional = required {
						field_type_name.push('>');
					}

					let is_flattened = matches!(&schema.kind, swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(_)));

					result.push(templates::Property {
						name,
						comment: schema.description.as_deref(),
						field_name,
						field_type_name,
						required,
						is_flattened,
					});
				}

				let resource_metadata = match single_group_version_kind {
					Some((single_group_version_kind, true, true)) =>
						Some(if single_group_version_kind.group.is_empty() {
							(
								format!("{:?}", single_group_version_kind.version),
								format!("{:?}", ""),
								format!("{:?}", single_group_version_kind.kind),
								format!("{:?}", single_group_version_kind.version),
								definition.list_kind.as_ref().map(|kind| format!("{:?}", kind)),
							)
						}
						else {
							(
								format!("{:?}", format!("{}/{}", single_group_version_kind.group, single_group_version_kind.version)),
								format!("{:?}", single_group_version_kind.group),
								format!("{:?}", single_group_version_kind.kind),
								format!("{:?}", single_group_version_kind.version),
								definition.list_kind.as_ref().map(|kind| format!("{:?}", kind)),
							)
						}),
					Some((_, true, false)) => return Err(format!("{} has an apiVersion property but not a kind property", definition_path).into()),
					Some((_, false, true)) => return Err(format!("{} has a kind property but not an apiVersion property", definition_path).into()),
					Some((_, false, false)) | None => None,
				};

				(result, resource_metadata, metadata_ty)
			};

			templates::r#struct::generate(
				&mut out,
				vis,
				type_name,
				Default::default(),
				&template_properties,
			)?;

			let mut namespace_or_cluster_scoped_url_path_segment_and_scope = vec![];
			let mut subresource_url_path_segment_and_scope = vec![];

			if !definition.kubernetes_group_kind_versions.is_empty() {
				let mut kubernetes_group_kind_versions: Vec<_> = definition.kubernetes_group_kind_versions.iter().collect();
				kubernetes_group_kind_versions.sort();

				let mut operations_by_gkv: std::collections::BTreeMap<_, Vec<_>> = Default::default();
				for operation in std::mem::take(operations) {
					operations_by_gkv
						.entry(operation.kubernetes_group_kind_version.clone())
						.or_default()
						.push(operation);
				}

				for kubernetes_group_kind_version in kubernetes_group_kind_versions {
					if let Some(mut operations) = operations_by_gkv.remove(&Some(kubernetes_group_kind_version.clone())) {
						operations.sort_by(|o1, o2| o1.id.cmp(&o2.id));

						writeln!(out)?;
						writeln!(out, "// Begin {}/{}/{}",
							kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;

						for operation in operations {
							let (operation_optional_parameters_name, operation_result_name) =
								write_operation(
									&mut out,
									&operation,
									map_namespace,
									vis,
									Some(type_name),
									operation_feature)?;
							state.handle_operation_types(operation_optional_parameters_name.as_deref(), operation_result_name.as_deref())?;
							run_result.num_generated_apis += 1;

							// If this is a CRUD operation, use it to determine the resource's URL path segment and scope.
							match operation.kubernetes_action {
								Some(
									swagger20::KubernetesAction::Delete |
									swagger20::KubernetesAction::Get |
									swagger20::KubernetesAction::Post |
									swagger20::KubernetesAction::Put
								) => (),
								_ => continue,
							}
							let mut components = operation.path.rsplit('/');
							let components = (
								components.next().expect("str::rsplit returns at least one component"),
								components.next(),
								components.next(),
								components.next(),
							);

							// TODO:
							// The clippy lint doesn't take into account that the order of the arms is significant.
							// Ref: https://github.com/rust-lang/rust-clippy/issues/860
							#[allow(clippy::match_same_arms)]
							let (url_path_segment_, scope_, url_path_segment_and_scope) = match components {
								("{name}", Some(url_path_segment), Some("{namespace}"), Some("namespaces")) =>
									(format!("{:?}", url_path_segment), format!("{}NamespaceResourceScope", local), &mut namespace_or_cluster_scoped_url_path_segment_and_scope),

								("{name}", Some(url_path_segment), _, _) =>
									(format!("{:?}", url_path_segment), format!("{}ClusterResourceScope", local), &mut namespace_or_cluster_scoped_url_path_segment_and_scope),

								(url_path_segment, Some("{name}"), _, _) =>
									(format!("{:?}", url_path_segment), format!("{}SubResourceScope", local), &mut subresource_url_path_segment_and_scope),

								(url_path_segment, Some("{namespace}"), Some("namespaces"), _) =>
									(format!("{:?}", url_path_segment), format!("{}NamespaceResourceScope", local), &mut namespace_or_cluster_scoped_url_path_segment_and_scope),

								(url_path_segment, _, _, _) =>
									(format!("{:?}", url_path_segment), format!("{}ClusterResourceScope", local), &mut namespace_or_cluster_scoped_url_path_segment_and_scope),
							};

							url_path_segment_and_scope.push((url_path_segment_, scope_));
						}

						writeln!(out)?;
						writeln!(out, "// End {}/{}/{}",
							kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;
					}
				}

				*operations = operations_by_gkv.into_values().flatten().collect();
			}

			match &**definition_path {
				"io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup" |
				"io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupList" |
				"io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList" |
				"io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions" =>
					namespace_or_cluster_scoped_url_path_segment_and_scope.push((r#""""#.to_owned(), format!("{}ClusterResourceScope", local))),
				"io.k8s.apimachinery.pkg.apis.meta.v1.Status" =>
					subresource_url_path_segment_and_scope.push((r#""status""#.to_owned(), format!("{}SubResourceScope", local))),
				_ => (),
			}

			namespace_or_cluster_scoped_url_path_segment_and_scope.dedup();
			subresource_url_path_segment_and_scope.dedup();

			let template_resource_metadata = match (&resource_metadata, &metadata_ty) {
				(
					Some((api_version, group, kind, version, list_kind)),
					Some((metadata_ty, templates::PropertyRequired::Required { is_default: _ })),
				) => Some(templates::ResourceMetadata {
					api_version,
					group,
					kind,
					version,
					list_kind: list_kind.as_deref(),
					metadata_ty: Some(metadata_ty),
					url_path_segment_and_scope: match (&*namespace_or_cluster_scoped_url_path_segment_and_scope, &*subresource_url_path_segment_and_scope) {
						([(url_path_segment, scope)], _) |
						([], [(url_path_segment, scope)]) => (&**url_path_segment, &**scope),

						([], []) => return Err(format!(
							"definition {} is a Resource but its URL path segment and scope could not be inferred",
							definition_path).into()),
						([_, ..], _) => return Err(format!(
							"definition {} is a Resource but was inferred to have multiple scopes {:?}",
							definition_path, namespace_or_cluster_scoped_url_path_segment_and_scope).into()),
						([], [_, ..]) => return Err(format!(
							"definition {} is a Resource but was inferred to have multiple scopes {:?}",
							definition_path, subresource_url_path_segment_and_scope).into()),
					},
				}),

				(Some(_), Some((_, templates::PropertyRequired::Optional | templates::PropertyRequired::OptionalDefault))) =>
					return Err(format!("definition {} has optional metadata", definition_path).into()),

				(
					Some((api_version, group, kind, version, list_kind)),
					None,
				) => Some(templates::ResourceMetadata {
					api_version,
					group,
					kind,
					version,
					list_kind: list_kind.as_deref(),
					metadata_ty: None,
					url_path_segment_and_scope: match (&*namespace_or_cluster_scoped_url_path_segment_and_scope, &*subresource_url_path_segment_and_scope) {
						([(url_path_segment, scope)], _) |
						([], [(url_path_segment, scope)]) => (&**url_path_segment, &**scope),

						([], []) => return Err(format!(
							"definition {} is a Resource but its URL path segment and scope could not be inferred",
							definition_path).into()),
						([_, _, ..], _) => return Err(format!(
							"definition {} is a Resource but was inferred to have multiple scopes {:?}",
							definition_path, namespace_or_cluster_scoped_url_path_segment_and_scope).into()),
						([], [_, _, ..]) => return Err(format!(
							"definition {} is a Resource but was inferred to have multiple scopes {:?}",
							definition_path, subresource_url_path_segment_and_scope).into()),
					},
				}),

				(None, _) => None,
			};

			if let Some(template_resource_metadata) = &template_resource_metadata {
				templates::impl_resource::generate(
					&mut out,
					type_name,
					Default::default(),
					map_namespace,
					template_resource_metadata,
				)?;

				templates::impl_listable_resource::generate(
					&mut out,
					type_name,
					Default::default(),
					map_namespace,
					template_resource_metadata,
				)?;

				templates::impl_metadata::generate(
					&mut out,
					type_name,
					Default::default(),
					map_namespace,
					template_resource_metadata,
				)?;
			}

			templates::impl_deserialize::generate(
				&mut out,
				type_name,
				Default::default(),
				&template_properties,
				map_namespace,
				template_resource_metadata.as_ref(),
			)?;

			templates::impl_serialize::generate(
				&mut out,
				type_name,
				Default::default(),
				&template_properties,
				map_namespace,
				template_resource_metadata.as_ref(),
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ref(_) => return Err(format!("{} is a Ref", definition_path).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => {
			templates::int_or_string::generate(
				&mut out,
				type_name,
				map_namespace,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ (
			swagger20::Type::JsonSchemaPropsOrArray(_) |
			swagger20::Type::JsonSchemaPropsOrBool(_) |
			swagger20::Type::JsonSchemaPropsOrStringArray(_)
		)) => {
			let (namespace, json_schema_props_or) = match ty {
				swagger20::Type::JsonSchemaPropsOrArray(namespace) => (namespace, templates::json_schema_props_or::Or::Array),
				swagger20::Type::JsonSchemaPropsOrBool(namespace) => (namespace, templates::json_schema_props_or::Or::Bool),
				swagger20::Type::JsonSchemaPropsOrStringArray(namespace) => (namespace, templates::json_schema_props_or::Or::StringArray),
				_ => unreachable!("unexpected JSONSchemaPropsOr* variant"),
			};

			let json_schema_props_type_name =
				get_fully_qualified_type_name(
					&swagger20::RefPath {
						path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", namespace),
						can_be_default: None,
					},
					map_namespace);

			templates::json_schema_props_or::generate(
				&mut out,
				type_name,
				json_schema_props_or,
				&json_schema_props_type_name,
				map_namespace,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => {
			templates::patch::generate(
				&mut out,
				type_name,
				map_namespace,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(raw_extension_ref_path)) => {
			let has_bookmark_event_type = {
				let watch_optional_schema =
					definitions.get(&swagger20::DefinitionPath("io.k8s.WatchOptional".to_owned()))
					.ok_or("could not find io.k8s.WatchOptional")?;
				let watch_optional_properties =
					if let swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(properties)) = &watch_optional_schema.kind {
						properties
					}
					else {
						return Err("io.k8s.WatchOptional has unexpected schema kind".into());
					};
				watch_optional_properties.contains_key(&swagger20::PropertyName("allowWatchBookmarks".to_owned()))
			};

			let error_status_rust_type = get_rust_type(
				&swagger20::SchemaKind::Ref(swagger20::RefPath {
					path: "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned(),
					can_be_default: None,
				}),
				map_namespace,
			)?;

			let error_other_rust_type = get_rust_type(
				&swagger20::SchemaKind::Ref(raw_extension_ref_path.clone()),
				map_namespace,
			)?;

			templates::watch_event::generate(
				&mut out,
				type_name,
				has_bookmark_event_type,
				&error_status_rust_type,
				&error_other_rust_type,
				map_namespace,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { metadata }) => {
			let metadata_rust_type = get_rust_type(metadata, map_namespace)?;

			let template_generics_where_part = format!("T: {}ListableResource", local);
			let template_generics = templates::Generics {
				type_part: Some("T"),
				where_part: Some(&template_generics_where_part),
			};

			let template_properties = vec![
				templates::Property {
					name: "items",
					comment: Some("List of objects."),
					field_name: "items".into(),
					field_type_name: "Vec<T>".to_owned(),
					required: templates::PropertyRequired::Required { is_default: true },
					is_flattened: false,
				},

				templates::Property {
					name: "metadata",
					comment: Some("Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"),
					field_name: "metadata".into(),
					field_type_name: (&*metadata_rust_type).to_owned(),
					required: templates::PropertyRequired::Required { is_default: true },
					is_flattened: false,
				},
			];

			let template_resource_metadata = templates::ResourceMetadata {
				api_version: "<T as crate::Resource>::API_VERSION",
				group: "<T as crate::Resource>::GROUP",
				kind: "<T as crate::ListableResource>::LIST_KIND",
				version: "<T as crate::Resource>::VERSION",
				list_kind: None,
				metadata_ty: Some(&metadata_rust_type),
				url_path_segment_and_scope: (r#""""#, "<T as crate::Resource>::Scope"),
			};

			templates::r#struct::generate(
				&mut out,
				vis,
				type_name,
				template_generics,
				&template_properties,
			)?;

			templates::impl_resource::generate(
				&mut out,
				type_name,
				template_generics,
				map_namespace,
				&template_resource_metadata,
			)?;

			templates::impl_listable_resource::generate(
				&mut out,
				type_name,
				template_generics,
				map_namespace,
				&template_resource_metadata,
			)?;

			templates::impl_metadata::generate(
				&mut out,
				type_name,
				template_generics,
				map_namespace,
				&template_resource_metadata,
			)?;

			{
				let template_generics_where_part = format!("T: {local}serde::Deserialize<'de> + {local}ListableResource", local = local);
				let template_generics = templates::Generics {
					where_part: Some(&template_generics_where_part),
					..template_generics
				};

				templates::impl_deserialize::generate(
					&mut out,
					type_name,
					template_generics,
					&template_properties,
					map_namespace,
					Some(&template_resource_metadata),
				)?;
			}

			{
				let template_generics_where_part = format!("T: {local}serde::Serialize + {local}ListableResource", local = local);
				let template_generics = templates::Generics {
					where_part: Some(&template_generics_where_part),
					..template_generics
				};

				templates::impl_serialize::generate(
					&mut out,
					type_name,
					template_generics,
					&template_properties,
					map_namespace,
					Some(&template_resource_metadata),
				)?;
			}

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) => {
			let item_type_name = get_rust_type(items, map_namespace)?;
			let alias_type_name = format!("{}List<{}>", local, item_type_name);

			templates::type_alias::generate(
				&mut out,
				type_name,
				&alias_type_name,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ (
			swagger20::Type::CreateOptional(_) |
			swagger20::Type::DeleteOptional(_) |
			swagger20::Type::ListOptional(_) |
			swagger20::Type::PatchOptional(_) |
			swagger20::Type::ReplaceOptional(_) |
			swagger20::Type::WatchOptional(_)
		)) => {
			let properties = match ty {
				swagger20::Type::CreateOptional(properties) |
				swagger20::Type::DeleteOptional(properties) |
				swagger20::Type::ListOptional(properties) |
				swagger20::Type::PatchOptional(properties) |
				swagger20::Type::ReplaceOptional(properties) |
				swagger20::Type::WatchOptional(properties) => properties,
				_ => unreachable!("unexpected optional params type"),
			};

			let template_properties = {
				let mut result = Vec::with_capacity(properties.len());

				for (name, schema) in properties {
					let field_name = get_rust_ident(name);

					let type_name = get_rust_borrow_type(&schema.kind, map_namespace)?;

					let field_type_name =
						if let Some(borrowed_type_name) = type_name.strip_prefix('&') {
							format!("Option<&'a {}>", borrowed_type_name)
						}
						else {
							format!("Option<{}>", type_name)
						};

					result.push(templates::Property {
						name,
						comment: schema.description.as_deref(),
						field_name,
						field_type_name,
						required: templates::PropertyRequired::Optional,
						is_flattened: false,
					});
				}

				result
			};

			let template_generics = templates::Generics {
				type_part: Some("'a"),
				where_part: None,
			};

			templates::r#struct::generate(
				&mut out,
				vis,
				type_name,
				template_generics,
				&template_properties,
			)?;

			match ty {
				swagger20::Type::CreateOptional(_) |
				swagger20::Type::ListOptional(_) |
				swagger20::Type::PatchOptional(_) |
				swagger20::Type::ReplaceOptional(_) =>
					templates::query_string_optional::generate(
						&mut out,
						type_name,
						template_generics,
						vis,
						&template_properties,
						false,
						operation_feature,
						map_namespace,
					)?,

				swagger20::Type::DeleteOptional(_) =>
					templates::impl_serialize::generate(
						&mut out,
						type_name,
						template_generics,
						&template_properties,
						map_namespace,
						None,
					)?,

				swagger20::Type::WatchOptional(_) =>
					templates::query_string_optional::generate(
						&mut out,
						type_name,
						template_generics,
						vis,
						&template_properties,
						true,
						operation_feature,
						map_namespace,
					)?,

				_ => unreachable!("unexpected optional params type"),
			}

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::Create,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::Delete,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::List,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::Patch,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::Replace,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				type_name,
				map_namespace,
				templates::operation_response_common::OperationAction::Watch,
				operation_feature,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(_) => {
			let inner_type_name = get_rust_type(&definition.kind, map_namespace)?;

			// Kubernetes requires MicroTime to be serialized with exactly six decimal digits, instead of the default serde serialization of `chrono::DateTime`
			// that uses a variable number up to nine.
			//
			// Furthermore, while Kubernetes does deserialize a Time from a string with one or more decimal digits,
			// the format string it uses to *serialize* datetimes does not contain any decimal digits. So match that behavior just to be safe, and to have
			// the same behavior as the golang client.
			//
			// Refs:
			// - https://github.com/Arnavion/k8s-openapi/issues/63
			// - https://github.com/deislabs/krustlet/issues/5
			// - https://github.com/kubernetes/apimachinery/issues/88
			let datetime_serialization_format = match (&**definition_path, &definition.kind) {
				(
					"io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime",
					swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }),
				) => templates::DateTimeSerializationFormat::SixDecimalDigits,

				(
					"io.k8s.apimachinery.pkg.apis.meta.v1.Time",
					swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }),
				) => templates::DateTimeSerializationFormat::ZeroDecimalDigits,

				_ => templates::DateTimeSerializationFormat::Default,
			};

			templates::newtype::generate(
				&mut out,
				vis,
				type_name,
				&inner_type_name,
				datetime_serialization_format,
				map_namespace,
			)?;

			run_result.num_generated_type_aliases += 1;
		},
	}

	if let GenerateSchema::Yes { feature: schema_feature } = generate_schema {
		match &definition.kind {
			swagger20::SchemaKind::Properties(_) |
			swagger20::SchemaKind::Ty(
				swagger20::Type::Any |
				swagger20::Type::Array { .. } |
				swagger20::Type::Boolean |
				swagger20::Type::Integer { .. } |
				swagger20::Type::IntOrString |
				swagger20::Type::Number { .. } |
				swagger20::Type::Object { .. } |
				swagger20::Type::String { .. } |
				swagger20::Type::JsonSchemaPropsOrArray(_) |
				swagger20::Type::JsonSchemaPropsOrBool(_) |
				swagger20::Type::JsonSchemaPropsOrStringArray(_) |
				swagger20::Type::Patch
			) => {
				templates::impl_schema::generate(
					&mut out,
					type_name,
					Default::default(),
					definition_path,
					definition,
					schema_feature,
					map_namespace,
				)?;
			} 

			swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => {
				templates::impl_schema::generate(
					&mut out,
					type_name,
					templates::Generics {
						type_part: Some("T"),
						where_part: None,
					},
					definition_path,
					definition,
					schema_feature,
					map_namespace,
				)?;
			}

			_ => (),
		}
	}

	state.finish(out);

	Ok(run_result)
}

fn map_namespace_local_to_string(map_namespace: &impl MapNamespace) -> Result<String, Error> {
	let namespace_parts = map_namespace.map_namespace(&["io", "k8s"]).ok_or(r#"unexpected path "io.k8s""#)?;

	let mut result = String::new();
	for namespace_part in namespace_parts {
		result.push_str(&*get_rust_ident(namespace_part));
		result.push_str("::");
	}
	Ok(result)
}

fn get_derives(
	kind: &swagger20::SchemaKind,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	map_namespace: &impl MapNamespace,
) -> Result<Option<templates::type_header::Derives>, Error> {
	if matches!(kind, swagger20::SchemaKind::Ty(swagger20::Type::ListRef { .. })) {
		// ListRef is emitted as a type alias.
		return Ok(None);
	}

	let derive_clone =
		evaluate_trait_bound(
			kind,
			true,
			definitions,
			map_namespace,
			|kind, _| Ok(!matches!(kind, swagger20::SchemaKind::Ty(
				swagger20::Type::CreateResponse |
				swagger20::Type::DeleteResponse |
				swagger20::Type::ListResponse |
				swagger20::Type::PatchResponse |
				swagger20::Type::ReplaceResponse |
				swagger20::Type::WatchResponse
			))))?;

	let derive_copy =
		derive_clone &&
		evaluate_trait_bound(
			kind,
			false,
			definitions,
			map_namespace,
			|kind, _| Ok(matches!(kind, swagger20::SchemaKind::Ty(
				swagger20::Type::CreateOptional(_) |
				swagger20::Type::DeleteOptional(_) |
				swagger20::Type::ListOptional(_) |
				swagger20::Type::PatchOptional(_) |
				swagger20::Type::ReplaceOptional(_) |
				swagger20::Type::WatchOptional(_)
			))))?;

	#[allow(clippy::match_same_arms)]
	let is_default = evaluate_trait_bound(kind, false, definitions, map_namespace, |kind, required| match kind {
		// Option<T>::default is None regardless of T
		_ if !required => Ok(true),

		swagger20::SchemaKind::Ref(swagger20::RefPath { can_be_default: Some(can_be_default), .. }) => Ok(*can_be_default),

		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. }) if ref_path.references_scope(map_namespace) => Ok(false),

		// metadata field in resource type created by #[derive(CustomResourceDefinition)]
		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. })
			if !ref_path.references_scope(map_namespace) && ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta" => Ok(true),

		// Handled by evaluate_trait_bound
		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. }) if !ref_path.references_scope(map_namespace) => unreachable!(),

		// chrono::DateTime<chrono::Utc> is not Default
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok(false),

		// Enums without a default value
		swagger20::SchemaKind::Ty(
			swagger20::Type::JsonSchemaPropsOrArray(_) |
			swagger20::Type::JsonSchemaPropsOrBool(_) |
			swagger20::Type::JsonSchemaPropsOrStringArray(_) |
			swagger20::Type::Patch |
			swagger20::Type::WatchEvent(_) |
			swagger20::Type::CreateResponse |
			swagger20::Type::DeleteResponse |
			swagger20::Type::ListResponse |
			swagger20::Type::PatchResponse |
			swagger20::Type::ReplaceResponse |
			swagger20::Type::WatchResponse
		) => Ok(false),

		_ => Ok(true),
	})?;
	let derive_default =
		is_default &&
		// IntOrString has a manual Default impl, so don't #[derive] it.
		!matches!(kind, swagger20::SchemaKind::Ty(swagger20::Type::IntOrString));

	let derive_partial_eq =
		evaluate_trait_bound(
			kind,
			true,
			definitions,
			map_namespace,
			|kind, _| Ok(!matches!(kind, swagger20::SchemaKind::Ty(
				swagger20::Type::CreateResponse |
				swagger20::Type::DeleteResponse |
				swagger20::Type::ListResponse |
				swagger20::Type::PatchResponse |
				swagger20::Type::ReplaceResponse |
				swagger20::Type::WatchResponse
			))))?;

	// The choice of deriving Eq, Ord and PartialOrd is deliberately more conservative than the choice of deriving PartialEq,
	// so as to not change dramatically between Kubernetes versions. For example, ObjectMeta is Ord in v1.15 but not in v1.16 because
	// it indirectly gained a serde_json::Value field (`managed_fields.fields_v1.0`).
	//
	// Also, being conservative means the types generated by #[derive(k8s_openapi_derive::CustomResource)] don't have to require them either.

	let derive_eq =
		derive_partial_eq &&
		matches!(kind, swagger20::SchemaKind::Ty(
			swagger20::Type::IntOrString |
			swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }
		));

	let derive_partial_ord =
		derive_partial_eq &&
		matches!(kind, swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }));

	let derive_ord = derive_partial_ord && derive_eq;

	Ok(Some(templates::type_header::Derives {
		clone: derive_clone,
		copy: derive_copy,
		default: derive_default,
		eq: derive_eq,
		ord: derive_ord,
		partial_eq: derive_partial_eq,
		partial_ord: derive_partial_ord,
	}))
}

fn is_default(
	kind: &swagger20::SchemaKind,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	map_namespace: &impl MapNamespace,
) -> Result<bool, Error> {
	#[allow(clippy::match_same_arms)]
	evaluate_trait_bound(kind, false, definitions, map_namespace, |kind, required| match kind {
		// Option<T>::default is None regardless of T
		_ if !required => Ok(true),

		swagger20::SchemaKind::Ref(swagger20::RefPath { can_be_default: Some(can_be_default), .. }) => Ok(*can_be_default),

		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. }) if ref_path.references_scope(map_namespace) => Ok(false),

		// metadata field in resource type created by #[derive(CustomResourceDefinition)]
		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. })
			if !ref_path.references_scope(map_namespace) && ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta" => Ok(true),

		// Handled by evaluate_trait_bound
		swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. }) if !ref_path.references_scope(map_namespace) => unreachable!(),

		// chrono::DateTime<chrono::Utc> is not Default
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok(false),

		// Enums without a default value
		swagger20::SchemaKind::Ty(
			swagger20::Type::JsonSchemaPropsOrArray(_) |
			swagger20::Type::JsonSchemaPropsOrBool(_) |
			swagger20::Type::JsonSchemaPropsOrStringArray(_) |
			swagger20::Type::Patch |
			swagger20::Type::WatchEvent(_) |
			swagger20::Type::CreateResponse |
			swagger20::Type::DeleteResponse |
			swagger20::Type::ListResponse |
			swagger20::Type::PatchResponse |
			swagger20::Type::ReplaceResponse |
			swagger20::Type::WatchResponse
		) => Ok(false),

		_ => Ok(true),
	})
}

fn evaluate_trait_bound(
	kind: &swagger20::SchemaKind,
	array_follows_elements: bool,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	map_namespace: &impl MapNamespace,
	mut f: impl FnMut(&swagger20::SchemaKind, bool) -> Result<bool, Error>,
) -> Result<bool, Error> {
	fn evaluate_trait_bound_inner<'a>(
		kind: &std::borrow::Cow<'a, swagger20::SchemaKind>,
		required: bool,
		array_follows_elements: bool,
		definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
		map_namespace: &impl MapNamespace,
		visited: &mut std::collections::BTreeSet<std::borrow::Cow<'a, swagger20::SchemaKind>>,
		f: &mut impl FnMut(&swagger20::SchemaKind, bool) -> Result<bool, Error>,
	) -> Result<bool, Error> {
		if !visited.insert(kind.clone()) {
			// In case of recursive types, assume the bound holds.
			return Ok(true);
		}

		match &**kind {
			swagger20::SchemaKind::Properties(properties) => {
				for (property_schema, property_required) in properties.values() {
					let mut visited = visited.clone();
					let field_bound =
						evaluate_trait_bound_inner(
							&std::borrow::Cow::Borrowed(&property_schema.kind),
							required && *property_required,
							array_follows_elements,
							definitions,
							map_namespace,
							&mut visited,
							f,
						)?;
					if !field_bound {
						return Ok(false);
					}
				}

				Ok(true)
			},

			swagger20::SchemaKind::Ref(ref_path @ swagger20::RefPath { .. }) if !ref_path.references_scope(map_namespace) => {
				#[allow(clippy::option_if_let_else)]
				let trait_bound =
					if let Some(target) = definitions.get(&swagger20::DefinitionPath(ref_path.path.clone())) {
						let mut visited = visited.clone();
						evaluate_trait_bound_inner(
							&std::borrow::Cow::Borrowed(&target.kind),
							required,
							array_follows_elements,
							definitions,
							map_namespace,
							&mut visited,
							f,
						)
					}
					else {
						f(kind, required)
					};
				trait_bound
			},

			swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) if array_follows_elements =>
				evaluate_trait_bound_inner(
					&std::borrow::Cow::Owned(items.kind.clone()),
					required,
					array_follows_elements,
					definitions,
					map_namespace,
					visited,
					f,
				),

			swagger20::SchemaKind::Ty(
				swagger20::Type::JsonSchemaPropsOrArray(namespace) |
				swagger20::Type::JsonSchemaPropsOrBool(namespace) |
				swagger20::Type::JsonSchemaPropsOrStringArray(namespace)
			) => {
				let json_schema_props_ref_path = swagger20::RefPath {
					path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", namespace),
					can_be_default: None,
				};
				let json_schema_props_bound =
					evaluate_trait_bound_inner(
						&std::borrow::Cow::Owned(swagger20::SchemaKind::Ref(json_schema_props_ref_path)),
						required,
						array_follows_elements,
						definitions,
						map_namespace,
						visited,
						f,
					)?;
				if !json_schema_props_bound {
					return Ok(false);
				}

				f(kind, required)
			},

			swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(raw_extension_ref_path)) => {
				let raw_extension_bound =
					evaluate_trait_bound_inner(
						&std::borrow::Cow::Owned(swagger20::SchemaKind::Ref(raw_extension_ref_path.clone())),
						required,
						array_follows_elements,
						definitions,
						map_namespace,
						visited,
						f,
					)?;
				if !raw_extension_bound {
					return Ok(false);
				}

				f(kind, required)
			},

			kind => f(kind, required),
		}
	}

	let mut visited = Default::default();
	evaluate_trait_bound_inner(
		&std::borrow::Cow::Borrowed(kind),
		true,
		array_follows_elements,
		definitions,
		map_namespace,
		&mut visited,
		&mut f,
	)
}

fn get_comment_text<'a>(s: &'a str, indent: &'a str) -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'a {
	s.lines().scan(true, move |previous_line_was_empty, line|
		if line.is_empty() {
			*previous_line_was_empty = true;
			Some("".into())
		}
		else {
			let line =
				line
				.replace('\\', r"\\")
				.replace('[', r"\[")
				.replace(']', r"\]")
				.replace('<', r"\<")
				.replace('>', r"\>")
				.replace('\t', "    ");

			let line =
				if *previous_line_was_empty && line.starts_with("    ") {
					// Collapse this line's spaces into two. Otherwise rustdoc will think this is the start of a code block containing a Rust test.
					format!("  {}", line.trim_start())
				}
				else {
					line
				};

			*previous_line_was_empty = false;

			Some(format!("{} {}", indent, line).into())
		})
}

fn get_fully_qualified_type_name(
	ref_path: &swagger20::RefPath,
	map_namespace: &impl MapNamespace,
) -> String {
	let path_parts: Vec<_> = ref_path.path.split('.').collect();
	let namespace_parts = map_namespace.map_namespace(&path_parts[..(path_parts.len() - 1)]);
	if let Some(namespace_parts) = namespace_parts {
		let mut result = String::new();
		for namespace_part in namespace_parts {
			result.push_str(&*get_rust_ident(namespace_part));
			result.push_str("::");
		}
		result.push_str(path_parts[path_parts.len() - 1]);
		result
	}
	else {
		let last_part = *path_parts.last().expect("str::split yields at least one item");
		last_part.to_owned()
	}
}

/// Converts the given string into a string that can be used as a Rust ident.
pub fn get_rust_ident(name: &str) -> std::borrow::Cow<'static, str> {
	// Fix cases of invalid rust idents
	match name {
		"$ref" => return "ref_path".into(),
		"$schema" => return "schema".into(),
		"as" => return "as_".into(),
		"continue" => return "continue_".into(),
		"enum" => return "enum_".into(),
		"ref" => return "ref_".into(),
		"type" => return "type_".into(),
		_ => (),
	}

	// Some cases of "ABc" should be converted to "abc" instead of "a_bc".
	// Eg "JSONSchemas" => "json_schemas", but "externalIPs" => "external_ips" instead of "external_i_ps".
	// Mostly happens with plurals of abbreviations.
	match name {
		"clusterIPs" => return "cluster_ips".into(),
		"externalIPs" => return "external_ips".into(),
		"nonResourceURLs" => return "non_resource_urls".into(),
		"podCIDRs" => return "pod_cidrs".into(),
		"podIPs" => return "pod_ips".into(),
		"serverAddressByClientCIDRs" => return "server_address_by_client_cidrs".into(),
		"targetWWNs" => return "target_wwns".into(),
		_ => (),
	}

	let mut result = String::new();

	let chars =
		name.chars()
		.zip(std::iter::once(None).chain(name.chars().map(|c| Some(c.is_uppercase()))))
		.zip(name.chars().skip(1).map(|c| Some(c.is_uppercase())).chain(std::iter::once(None)));

	for ((c, previous), next) in chars {
		if c.is_uppercase() {
			match (previous, next) {
				(Some(false), _) |
				(Some(true), Some(false)) => result.push('_'),
				_ => (),
			}

			result.extend(c.to_lowercase());
		}
		else {
			result.push(match c {
				'.' | '-' => '_',
				c => c,
			});
		}
	}

	result.into()
}

fn get_rust_borrow_type(
	schema_kind: &swagger20::SchemaKind,
	map_namespace: &impl MapNamespace,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	let local = map_namespace_local_to_string(map_namespace)?;

	#[allow(clippy::match_same_arms)]
	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.CreateOptional" =>
			Ok(format!("{}CreateOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.DeleteOptional" =>
			Ok(format!("{}DeleteOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.ListOptional" =>
			Ok(format!("{}ListOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.PatchOptional" =>
			Ok(format!("{}PatchOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.ReplaceOptional" =>
			Ok(format!("{}ReplaceOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.WatchOptional" =>
			Ok(format!("{}WatchOptional<'_>", local).into()),

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(format!("&{}", get_fully_qualified_type_name(ref_path, map_namespace)).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok(format!("&{}serde_json::Value", local).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("&[{}]", get_rust_type(&items.kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("&std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) =>
			Ok(format!("&{}", get_rust_type(schema_kind, map_namespace)?).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) =>
			Ok(format!("&{}", get_rust_type(schema_kind, map_namespace)?).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("&str".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(_)) =>
			Ok(format!("&{}", get_rust_type(schema_kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(
			swagger20::Type::JsonSchemaPropsOrArray(_) |
			swagger20::Type::JsonSchemaPropsOrBool(_) |
			swagger20::Type::JsonSchemaPropsOrStringArray(_)
		) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { .. }) => Err("ListDef type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { .. }) => Ok(format!("&{}", get_rust_type(schema_kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CreateOptional(_)) => Err("CreateOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) => Err("DeleteOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) => Err("ListOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) => Err("PatchOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceOptional(_)) => Err("ReplaceOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) => Err("WatchOptional type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) => Err("CreateResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) => Err("DeleteResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) => Err("ListResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) => Err("PatchResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) => Err("ReplaceResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => Err("WatchResponse type not supported".into()),
	}
}

fn get_rust_type(
	schema_kind: &swagger20::SchemaKind,
	map_namespace: &impl MapNamespace,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	let local = map_namespace_local_to_string(map_namespace)?;

	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(get_fully_qualified_type_name(ref_path, map_namespace).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok(format!("{}serde_json::Value", local).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("Vec<{}>", get_rust_type(&items.kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) =>
			Ok(format!("{}ByteString", local).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) =>
			Ok(format!("{local}chrono::DateTime<{local}chrono::Utc>", local = local).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("String".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(namespace)) => {
			let namespace_parts =
				&["io", "k8s", "apiextensions_apiserver", "pkg", "apis", "apiextensions", namespace];
			let namespace_parts =
				map_namespace.map_namespace(namespace_parts)
				.ok_or_else(|| format!("unexpected path {:?}", namespace_parts.join(".")))?;

			let mut result = String::new();
			for namespace_part in namespace_parts {
				result.push_str(&*get_rust_ident(namespace_part));
				result.push_str("::");
			}
			result.push_str("CustomResourceSubresources");
			Ok(result.into())
		},

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(
			swagger20::Type::JsonSchemaPropsOrArray(_) |
			swagger20::Type::JsonSchemaPropsOrBool(_) |
			swagger20::Type::JsonSchemaPropsOrStringArray(_)
		) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { .. }) => Err("ListDef type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) =>
			Ok(format!("{}List<{}>", local, get_rust_type(items, map_namespace)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CreateOptional(_)) => Err("CreateOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) => Err("DeleteOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) => Err("ListOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) => Err("PatchOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceOptional(_)) => Err("ReplaceOptional type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) => Err("WatchOptional type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) => Err("CreateResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) => Err("DeleteResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) => Err("ListResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) => Err("PatchResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) => Err("ReplaceResponse type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => Err("WatchResponse type not supported".into()),
	}
}

/// Each invocation of this function generates a single API operation function specified by the `operation` parameter.
///
/// # Parameters
///
/// - `out`: An impl of `std::io::Write` that the operation will be emitted to.
///
/// - `operation`: The operation to be emitted.
///
/// - `map_namespace`: An instance of the [`MapNamespace`] trait that controls how OpenAPI namespaces of the definitions are mapped to rust namespaces.
///
/// - `vis`: The visibility modifier that should be emitted on the generated code.
///
/// - `type_name`: The name of the type that this operation is associated with, if any.
///
/// - `operation_feature`: Specifies whether the API function will be emitted with a `#[cfg(feature = "<this value>")]` attribute or not.
///
/// # Returns
///
/// A tuple of two optional strings:
///
/// 1. The name of the optional parameters type associated with the operation, if any.
/// 1. The name of the response type associated with the operation, if any.
pub fn write_operation(
	mut out: impl std::io::Write,
	operation: &swagger20::Operation,
	map_namespace: &impl MapNamespace,
	vis: &str,
	type_name: Option<&str>,
	operation_feature: Option<&str>,
) -> Result<(Option<String>, Option<String>), Error> {
	let local = map_namespace_local_to_string(map_namespace)?;

	writeln!(out)?;

	writeln!(out, "// Generated from operation {}", operation.id)?;

	let (operation_fn_name, operation_result_name, operation_optional_parameters_name) =
		get_operation_names(operation, type_name.is_some())?;

	let indent = if type_name.is_some() { "    " } else { "" };

	writeln!(out)?;

	if let Some(type_name) = type_name {
		writeln!(out, "impl {} {{", type_name)?;
	}

	let mut previous_parameters: std::collections::HashSet<_> = Default::default();
	let parameters: Result<Vec<_>, Error> =
		operation.parameters.iter().map(std::ops::Deref::deref)
		.map(|parameter| {
			let mut parameter_name = get_rust_ident(&parameter.name);
			while previous_parameters.contains(&parameter_name) {
				parameter_name = format!("{}_", parameter_name).into();
			}
			previous_parameters.insert(parameter_name.clone());

			let parameter_type = match get_rust_borrow_type(&parameter.schema.kind, map_namespace) {
				Ok(parameter_type) => parameter_type,
				Err(err) => return Err(err),
			};

			Ok((parameter_name, parameter_type, parameter))
		})
		.collect();
	let mut parameters = parameters?;

	let delete_optional_parameter =
		parameters.iter()
		.position(|(_, _, parameter)|
			if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &parameter.schema.kind {
				path == "io.k8s.DeleteOptional"
			}
			else {
				false
			})
		.map(|index| parameters.swap_remove(index));

	let query_string_optional_parameter =
			parameters.iter()
			.position(|(_, _, parameter)|
				if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &parameter.schema.kind {
					path == "io.k8s.CreateOptional" ||
					path == "io.k8s.ListOptional" ||
					path == "io.k8s.PatchOptional" ||
					path == "io.k8s.ReplaceOptional" ||
					path == "io.k8s.WatchOptional" 
				}
				else {
					false
				})
			.map(|index| parameters.swap_remove(index));

	parameters.sort_by(|(_, _, parameter1), (_, _, parameter2)| {
		(match (parameter1.location, parameter2.location) {
			(location1, location2) if location1 == location2 => std::cmp::Ordering::Equal,
			(swagger20::ParameterLocation::Path, _) |
			(swagger20::ParameterLocation::Body, swagger20::ParameterLocation::Query) => std::cmp::Ordering::Less,
			_ => std::cmp::Ordering::Greater,
		})
		.then_with(|| parameter1.name.cmp(&parameter2.name))
	});
	let parameters = parameters;
	let (required_parameters, optional_parameters): (Vec<_>, Vec<_>) = parameters.iter().partition(|(_, _, parameter)| parameter.required);
	let any_optional_fields_have_lifetimes = optional_parameters.iter().any(|(_, parameter_type, _)| parameter_type.starts_with('&'));

	let mut need_empty_line = false;

	if let Some(description) = operation.description.as_ref() {
		for line in get_comment_text(description, "") {
			writeln!(out, "{}///{}", indent, line)?;
			need_empty_line = true;
		}
	}
	if let Some(operation_result_name) = &operation_result_name {
		if need_empty_line {
			writeln!(out, "{}///", indent)?;
		}

		writeln!(out,
			"{}/// Use the returned [`{}ResponseBody`]`<`[`{}`]`>` constructor, or [`{}`] directly, to parse the HTTP response.",
			indent, local, operation_result_name, operation_result_name)?;
		need_empty_line = true;
	}
	else {
		let common_response_type_link = match operation.responses {
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::CreateResponse) =>
				Some(format!("[`{}CreateResponse`]`<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse) => match operation.kubernetes_action {
				Some(swagger20::KubernetesAction::Delete) =>
					Some(format!("[`{}DeleteResponse`]`<Self>", local)),

				Some(swagger20::KubernetesAction::DeleteCollection) =>
					Some(format!("[`{local}DeleteResponse`]`<`[`{local}List`]`<Self>>", local = local)),

				kubernetes_action => return Err(format!(
					"operation {} has a DeleteResponse response but its action {:?} is neither a Delete nor DeleteCollection action",
					operation.id, kubernetes_action).into()),
			},

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ListResponse) =>
				Some(format!("[`{}ListResponse`]`<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::PatchResponse) =>
				Some(format!("[`{}PatchResponse`]`<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ReplaceResponse) =>
				Some(format!("[`{}ReplaceResponse`]`<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::WatchResponse) =>
				Some(format!("[`{}WatchResponse`]`<Self>", local)),

			_ => None,
		};

		if let Some(common_response_type_link) = common_response_type_link {
			if need_empty_line {
				writeln!(out, "{}///", indent)?;
			}

			writeln!(out,
				"{}/// Use the returned [`{}ResponseBody`]`<`{}>` constructor, or {}` directly, to parse the HTTP response.",
				indent,
				local,
				common_response_type_link,
				common_response_type_link)?;
			need_empty_line = true;
		}
	}

	if !parameters.is_empty() || delete_optional_parameter.is_some() || query_string_optional_parameter.is_some() {
		if need_empty_line {
			writeln!(out, "{}///", indent)?;
		}

		writeln!(out, "{}/// # Arguments", indent)?;
		for (parameter_name, _, parameter) in &required_parameters {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if let Some((parameter_name, _, parameter)) = &delete_optional_parameter {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if let Some((parameter_name, _, parameter)) = &query_string_optional_parameter {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `{}`", indent, parameter_name)?;
			if let Some(description) = parameter.schema.description.as_ref() {
				writeln!(out, "{}///", indent)?;
				for line in get_comment_text(description, "    ") {
					writeln!(out, "{}///{}", indent, line)?;
				}
			}
		}
		if !optional_parameters.is_empty() {
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}/// * `optional`", indent)?;
			writeln!(out, "{}///", indent)?;
			writeln!(out, "{}///     Optional parameters. Use `Default::default()` to not pass any.", indent)?;
		}
	}

	if let Some(operation_feature) = operation_feature {
		writeln!(out, r#"{}#[cfg(feature = "{}")]"#, indent, operation_feature)?;
	}

	writeln!(out, "{}{}fn {}(", indent, vis, operation_fn_name)?;
	for (parameter_name, parameter_type, _) in &required_parameters {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if let Some((parameter_name, parameter_type, _)) = &delete_optional_parameter {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if let Some((parameter_name, parameter_type, _)) = &query_string_optional_parameter {
		writeln!(out, "{}    {}: {},", indent, parameter_name, parameter_type)?;
	}
	if !optional_parameters.is_empty() {
		write!(out, "{}    optional: {}", indent, operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(out, "<'_>")?;
		}
		writeln!(out, ",")?;
	}
	if let Some(operation_result_name) = &operation_result_name {
		writeln!(out,
			"{}) -> Result<({local}http::Request<Vec<u8>>, fn({local}http::StatusCode) -> {local}ResponseBody<{}>), {local}RequestError> {{",
			indent, operation_result_name, local = local)?;
	}
	else {
		let common_response_type = match operation.responses {
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::CreateResponse) =>
				Some(format!("{}CreateResponse<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse) => match operation.kubernetes_action {
				Some(swagger20::KubernetesAction::Delete) =>
					Some(format!("{}DeleteResponse<Self>", local)),

				Some(swagger20::KubernetesAction::DeleteCollection) =>
					Some(format!("{local}DeleteResponse<{local}List<Self>>", local = local)),

				kubernetes_action => return Err(format!(
					"operation {} has a DeleteResponse response but its action {:?} is neither a Delete nor DeleteCollection action",
					operation.id, kubernetes_action).into()),
			},

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ListResponse) =>
				Some(format!("{}ListResponse<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::PatchResponse) =>
				Some(format!("{}PatchResponse<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ReplaceResponse) =>
				Some(format!("{}ReplaceResponse<Self>", local)),

			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::WatchResponse) =>
				Some(format!("{}WatchResponse<Self>", local)),

			_ => None,
		};

		if let Some(common_response_type) = common_response_type {
			writeln!(out,
				"{}) -> Result<({local}http::Request<Vec<u8>>, fn({local}http::StatusCode) -> {local}ResponseBody<{}>), {local}RequestError> {{",
				indent, common_response_type, local = local)?;
		}
		else {
			writeln!(out, "{}) -> Result<{local}http::Request<Vec<u8>>, {local}RequestError> {{", indent, local = local)?;
		}
	}

	let have_path_parameters = parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Path);
	let have_query_parameters =
		parameters.iter().any(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Query) ||
		query_string_optional_parameter.is_some();

	if !optional_parameters.is_empty() {
		writeln!(out, "{}    let {} {{", indent, operation_optional_parameters_name)?;
		for (parameter_name, _, _) in &optional_parameters {
			writeln!(out, "{}        {},", indent, parameter_name)?;
		}

		writeln!(out, "{}    }} = optional;", indent)?;
	}

	if have_path_parameters {
		write!(out, r#"{}    let __url = format!("{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(out, "?")?;
		}
		write!(out, r#"""#)?;
		if !parameters.is_empty() {
			writeln!(out, ",")?;
			for (parameter_name, parameter_type, parameter) in &parameters {
				if parameter.location == swagger20::ParameterLocation::Path {
					match parameter.schema.kind {
						swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => (),
						_ => return Err(format!("parameter {} is in the path but is a {:?}", parameter_name, parameter_type).into()),
					}

					writeln!(
						out,
						"{}        {} = {local}percent_encoding::percent_encode({}.as_bytes(), {local}percent_encoding2::PATH_SEGMENT_ENCODE_SET),",
						indent, parameter_name, parameter_name, local = local)?;
				}
			}
			write!(out, "{}    ", indent)?;
		}
		writeln!(out, ");")?;
	}
	else {
		write!(out, r#"{}    let __url = "{}"#, indent, operation.path)?;
		if have_query_parameters {
			write!(out, "?")?;
		}
		writeln!(out, r#"".to_owned();"#)?;
	}

	if have_query_parameters {
		writeln!(out, "{}    let mut __query_pairs = {}url::form_urlencoded::Serializer::new(__url);", indent, local)?;
		if let Some((parameter_name, _, _)) = &query_string_optional_parameter {
			writeln!(out, "{}    {}.__serialize(&mut __query_pairs);", indent, parameter_name)?;
		}
		else {
			for (parameter_name, parameter_type, parameter) in &parameters {
				if parameter.location == swagger20::ParameterLocation::Query {
					if parameter.required {
						match &parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) => match &items.kind {
								swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
									writeln!(out, "{}    for {} in {} {{", indent, parameter_name, parameter_name)?;
									writeln!(out, r#"{}        __query_pairs.append_pair({:?}, {});"#, indent, parameter.name, parameter_name)?;
									writeln!(out, "{}    }}", indent)?;
								},

								_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
							},

							swagger20::SchemaKind::Ty(
								swagger20::Type::Boolean |
								swagger20::Type::Integer { .. } |
								swagger20::Type::Number { .. }
							) => writeln!(out, r#"{}    __query_pairs.append_pair({:?}, &{}.to_string());"#, indent, parameter.name, parameter_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"{}    __query_pairs.append_pair({:?}, &{});"#, indent, parameter.name, parameter_name)?,

							_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
						}
					}
					else {
						writeln!(out, "{}    if let Some({}) = {} {{", indent, parameter_name, parameter_name)?;
						match &parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) => match &items.kind {
								swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
									writeln!(out, "{}        for {} in {} {{", indent, parameter_name, parameter_name)?;
									writeln!(out, r#"{}            __query_pairs.append_pair({:?}, {});"#, indent, parameter.name, parameter_name)?;
									writeln!(out, "{}        }}", indent)?;
								},

								_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
							},

							swagger20::SchemaKind::Ty(
								swagger20::Type::Boolean |
								swagger20::Type::Integer { .. } |
								swagger20::Type::Number { .. }
							) => writeln!(out, r#"{}        __query_pairs.append_pair({:?}, &{}.to_string());"#, indent, parameter.name, parameter_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"{}        __query_pairs.append_pair({:?}, {});"#, indent, parameter.name, parameter_name)?,

							_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
						}
						writeln!(out, "{}    }}", indent)?;
					}
				}
			}
		}
		writeln!(out, "{}    let __url = __query_pairs.finish();", indent)?;
	}
	writeln!(out)?;

	let method = match operation.method {
		swagger20::Method::Delete => "delete",
		swagger20::Method::Get => "get",
		swagger20::Method::Patch => "patch",
		swagger20::Method::Post => "post",
		swagger20::Method::Put => "put",
	};

	writeln!(out, "{}    let __request = {}http::Request::{}(__url);", indent, local, method)?;

	let body_parameter =
		delete_optional_parameter.as_ref()
		.or_else(|| parameters.iter().find(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Body));

	write!(out, "{}    let __body = ", indent)?;
	if let Some((parameter_name, parameter_type, parameter)) = body_parameter {
		if parameter.required {
			if parameter_type.starts_with('&') {
				writeln!(out, "{local}serde_json::to_vec({}).map_err({local}RequestError::Json)?;", parameter_name, local = local)?;
			}
			else {
				writeln!(out, "{local}serde_json::to_vec(&{}).map_err({local}RequestError::Json)?;", parameter_name, local = local)?;
			}
		}
		else {
			writeln!(out)?;
			writeln!(out, "{}.unwrap_or(Ok(vec![]), |value| {local}serde_json::to_vec(value).map_err({local}RequestError::Json))?;", parameter_name, local = local)?;
		}

		let is_patch =
			if let swagger20::SchemaKind::Ref(ref_path) = &parameter.schema.kind {
				ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.Patch"
			}
			else {
				false
			};
		if is_patch {
			let patch_type = get_rust_type(&parameter.schema.kind, map_namespace)?;
			writeln!(out,
				"{}    let __request = __request.header({local}http::header::CONTENT_TYPE, {local}http::header::HeaderValue::from_static(match {} {{",
				indent, parameter_name, local = local)?;
			writeln!(out, r#"{}        {}::Json(_) => "application/json-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::Merge(_) => "application/merge-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::StrategicMerge(_) => "application/strategic-merge-patch+json","#, indent, patch_type)?;
			writeln!(out, "{}    }}));", indent)?;
		}
		else {
			writeln!(out,
				r#"{}    let __request = __request.header({local}http::header::CONTENT_TYPE, {local}http::header::HeaderValue::from_static("application/json"));"#,
				indent, local = local)?;
		}
	}
	else {
		writeln!(out, "vec![];")?;
	}

	if operation_result_name.is_some() {
		writeln!(out, "{}    match __request.body(__body) {{", indent)?;
		writeln!(out, "{}        Ok(request) => Ok((request, {}ResponseBody::new)),", indent, local)?;
		writeln!(out, "{}        Err(err) => Err({}RequestError::Http(err)),", indent, local)?;
		writeln!(out, "{}    }}", indent)?;
	}
	else {
		let is_common_response_type = match operation.responses {
			crate::swagger20::OperationResponses::Common(_) => true,
			crate::swagger20::OperationResponses::Map(_) => false,
		};

		if is_common_response_type {
			writeln!(out, "{}    match __request.body(__body) {{", indent)?;
			writeln!(out, "{}        Ok(request) => Ok((request, {}ResponseBody::new)),", indent, local)?;
			writeln!(out, "{}        Err(err) => Err({}RequestError::Http(err)),", indent, local)?;
			writeln!(out, "{}    }}", indent)?;
		}
		else {
			writeln!(out, "{}    __request.body(__body).map_err({}RequestError::Http)", indent, local)?;
		}
	}
	writeln!(out, "{}}}", indent)?;

	if type_name.is_some() {
		writeln!(out, "}}")?;
	}

	if !optional_parameters.is_empty() {
		writeln!(out)?;

		if let Some(type_name) = type_name {
			writeln!(out, "/// Optional parameters of [`{}::{}`]", type_name, operation_fn_name)?;
		}
		else {
			writeln!(out, "/// Optional parameters of [`{}`]", operation_fn_name)?;
		}

		if let Some(operation_feature) = operation_feature {
			writeln!(out, r#"#[cfg(feature = "{}")]"#, operation_feature)?;
		}
		writeln!(out, "#[derive(Clone, Copy, Debug, Default)]")?;
		write!(out, "{}struct {}", vis, operation_optional_parameters_name)?;
		if any_optional_fields_have_lifetimes {
			write!(out, "<'a>")?;
		}
		writeln!(out, " {{")?;

		for (parameter_name, parameter_type, parameter) in &optional_parameters {
			if let Some(description) = parameter.schema.description.as_ref() {
				for line in get_comment_text(description, "") {
					writeln!(out, "    ///{}", line)?;
				}
			}
			if let Some(borrowed_parameter_type) = parameter_type.strip_prefix('&') {
				writeln!(out, "    {}{}: Option<&'a {}>,", vis, parameter_name, borrowed_parameter_type)?;
			}
			else {
				writeln!(out, "    {}{}: Option<{}>,", vis, parameter_name, parameter_type)?;
			}
		}

		writeln!(out, "}}")?;
	}

	if let swagger20::OperationResponses::Map(responses) = &operation.responses {
		if let Some(operation_result_name) = &operation_result_name {
			writeln!(out)?;

			if let Some(type_name) = type_name {
				writeln!(out,
					"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}::{}`]",
					operation_result_name, type_name, operation_fn_name)?;
			}
			else {
				writeln!(out,
					"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}`]",
					operation_result_name, operation_fn_name)?;
			}

			if let Some(operation_feature) = operation_feature {
				writeln!(out, r#"#[cfg(feature = "{}")]"#, operation_feature)?;
			}
			writeln!(out, "#[derive(Debug)]")?;
			writeln!(out, "{}enum {} {{", vis, operation_result_name)?;

			let operation_responses: Result<Vec<_>, _> =
				responses.iter()
				.map(|(&status_code, schema)| {
					let http_status_code = match status_code {
						http::StatusCode::ACCEPTED => "ACCEPTED",
						http::StatusCode::CREATED => "CREATED",
						http::StatusCode::OK => "OK",
						http::StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
						http::StatusCode::UNPROCESSABLE_ENTITY => "UNPROCESSABLE_ENTITY",
						_ => return Err(format!("unrecognized status code {}", status_code)),
					};

					let variant_name = match status_code {
						http::StatusCode::ACCEPTED => "Accepted",
						http::StatusCode::CREATED => "Created",
						http::StatusCode::OK => "Ok",
						http::StatusCode::UNAUTHORIZED => "Unauthorized",
						http::StatusCode::UNPROCESSABLE_ENTITY => "UnprocessableEntity",
						_ => return Err(format!("unrecognized status code {}", status_code)),
					};

					Ok((http_status_code, variant_name, schema))
				})
				.collect();
			let operation_responses = operation_responses?;

			for &(_, variant_name, schema) in &operation_responses {
				writeln!(out, "    {}({}),", variant_name, get_rust_type(&schema.kind, map_namespace)?)?;
			}

			writeln!(out, "    Other(Result<Option<{local}serde_json::Value>, {local}serde_json::Error>),", local = local)?;
			writeln!(out, "}}")?;
			writeln!(out)?;

			if let Some(operation_feature) = operation_feature {
				writeln!(out, r#"#[cfg(feature = "{}")]"#, operation_feature)?;
			}
			writeln!(out, "impl {}Response for {} {{", local, operation_result_name)?;
			writeln!(out, "    fn try_from_parts(status_code: {local}http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {local}ResponseError> {{", local = local)?;

			writeln!(out, "        match status_code {{")?;
			for &(http_status_code, variant_name, schema) in &operation_responses {
				writeln!(out, "            {}http::StatusCode::{} => {{", local, http_status_code)?;

				match &schema.kind {
					swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
						writeln!(out, "                if buf.is_empty() {{")?;
						writeln!(out, "                    return Err({}ResponseError::NeedMoreData);", local)?;
						writeln!(out, "                }}")?;
						writeln!(out)?;
						writeln!(out, "                let (result, len) = match std::str::from_utf8(buf) {{")?;
						writeln!(out, "                    Ok(s) => (s, buf.len()),")?;
						writeln!(out, "                    Err(err) => match (err.valid_up_to(), err.error_len()) {{")?;
						writeln!(out, "                        (0, Some(_)) => return Err({}ResponseError::Utf8(err)),", local)?;
						writeln!(out, "                        (0, None) => return Err({}ResponseError::NeedMoreData),", local)?;
						writeln!(out, "                        (valid_up_to, _) => (")?;
						writeln!(out, "                            unsafe {{ std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) }},")?;
						writeln!(out, "                            valid_up_to,")?;
						writeln!(out, "                        ),")?;
						writeln!(out, "                    }},")?;
						writeln!(out, "                }};")?;
						writeln!(out, "                Ok(({}::{}(result.to_owned()), len))", operation_result_name, variant_name)?;
					},

					swagger20::SchemaKind::Ref(_) => {
						writeln!(out, "                let result = match {}serde_json::from_slice(buf) {{", local)?;
						writeln!(out, "                    Ok(value) => value,")?;
						writeln!(out, "                    Err(err) if err.is_eof() => return Err({}ResponseError::NeedMoreData),", local)?;
						writeln!(out, "                    Err(err) => return Err({}ResponseError::Json(err)),", local)?;
						writeln!(out, "                }};")?;
						writeln!(out, "                Ok(({}::{}(result), buf.len()))", operation_result_name, variant_name)?;
					},

					other => return Err(format!("operation {} has unrecognized type for response of variant {}: {:?}", operation.id, variant_name, other).into()),
				}

				writeln!(out, "            }},")?;
			}
			writeln!(out, "            _ => {{")?;
			writeln!(out, "                let (result, read) =")?;
			writeln!(out, "                    if buf.is_empty() {{")?;
			writeln!(out, "                        (Ok(None), 0)")?;
			writeln!(out, "                    }}")?;
			writeln!(out, "                    else {{")?;
			writeln!(out, "                        match {}serde_json::from_slice(buf) {{", local)?;
			writeln!(out, "                            Ok(value) => (Ok(Some(value)), buf.len()),")?;
			writeln!(out, "                            Err(err) if err.is_eof() => return Err({}ResponseError::NeedMoreData),", local)?;
			writeln!(out, "                            Err(err) => (Err(err), 0),")?;
			writeln!(out, "                        }}")?;
			writeln!(out, "                    }};")?;
			writeln!(out, "                Ok(({}::Other(result), read))", operation_result_name)?;
			writeln!(out, "            }},")?;
			writeln!(out, "        }}")?;
			writeln!(out, "    }}")?;
			writeln!(out, "}}")?;
		}
	}

	let mut result = (None, operation_result_name);
	if type_name.is_some() && !optional_parameters.is_empty() {
		result.0 = Some(operation_optional_parameters_name);
	}
	Ok(result)
}

fn get_operation_names(
	operation: &swagger20::Operation,
	strip_tag: bool,
) -> Result<(std::borrow::Cow<'static, str>, Option<String>, String), Error> {
	let operation_id =
		if strip_tag {
			operation.tag.as_ref()
			.map_or(std::borrow::Cow::Borrowed(&*operation.id), |tag| {
				// For functions associated with types (eg `Pod::list_core_v1_namespaced_pod`), the API version contained in the operation name
				// is already obvious from the type's path (`core::v1::Pod`), so it can be stripped (`list_namespaced_pod`).
				let tag: String =
					tag.split('_')
					.map(|part| {
						let mut chars = part.chars();
						if let Some(first_char) = chars.next() {
							let rest_chars = chars.as_str();
							std::borrow::Cow::Owned(format!("{}{}", first_char.to_uppercase(), rest_chars))
						}
						else {
							std::borrow::Cow::Borrowed("")
						}
					})
					.collect();

				std::borrow::Cow::Owned(operation.id.replace(&tag, ""))
			})
		}
		else {
			// Functions not associated with types (eg `::get_core_v1_api_resources`) get emitted at the mod root,
			// so their ID should be used as-is.
			std::borrow::Cow::Borrowed(&*operation.id)
		};

	let operation_fn_name = get_rust_ident(&operation_id);

	let mut chars = operation_id.chars();
	let first_char = chars.next().ok_or_else(|| format!("operation has empty ID: {:?}", operation))?.to_uppercase();
	let rest_chars = chars.as_str();
	let operation_result_name = match (&operation.responses, operation.kubernetes_action) {
		(swagger20::OperationResponses::Common(_), _) |
		(_, Some(swagger20::KubernetesAction::Connect)) => None,
		_ => Some(format!("{}{}Response", first_char, rest_chars)),
	};
	let operation_optional_parameters_name = format!("{}{}Optional", first_char, rest_chars);

	Ok((operation_fn_name, operation_result_name, operation_optional_parameters_name))
}

#[cfg(test)]
mod test {
	#[test]
	fn test_get_rust_ident() {
		assert_eq!(super::get_rust_ident("as"), "as_");
		assert_eq!(super::get_rust_ident("foo.bar"), "foo_bar");
	}
}
