#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::must_use_candidate,
	clippy::similar_names,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unseparated_literal_suffix,
	clippy::use_self,
)]

//! This crate contains common code for the [`k8s-openapi` code generator](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-codegen)
//! and the [`k8s-openapi-derive`](https://crates.io/crates/k8s-openapi-derive) custom derive crate.
//!
//! WARNING: This crate is not meant to be used directly by end users and does not have a stable API.

#[doc(hidden)]
pub mod swagger20;

mod templates;

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct RunResult {
	pub num_generated_structs: usize,
	pub num_generated_type_aliases: usize,
	pub num_generated_apis: usize,
}

#[doc(hidden)]
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

#[doc(hidden)]
pub fn run<W>(
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	operations: &mut Vec<swagger20::Operation>,
	definition_path: &swagger20::DefinitionPath,
	ref_path_relative_to: swagger20::RefPathRelativeTo,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	vis: &str,
	use_api_feature_for_operations: bool,
	out: impl FnOnce(&[std::borrow::Cow<'_, str>], bool) -> std::io::Result<W>,
	mut imports: impl FnMut(Option<String>, Option<String>) -> std::io::Result<()>,
) -> Result<RunResult, Error> where W: std::io::Write {
	let definition = definitions.get(definition_path).ok_or_else(|| format!("definition for {} does not exist in spec", definition_path))?;

	let mut run_result = RunResult {
		num_generated_structs: 0,
		num_generated_type_aliases: 0,
		num_generated_apis: 0,
	};

	let parts = replace_namespace(definition_path.split('.'), replace_namespaces);

	let is_under_api_feature = match &definition.kind {
		swagger20::SchemaKind::Ty(swagger20::Type::CreateOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => true,

		_ => false,
	};

	let mut out = out(&parts, is_under_api_feature)?;

	let type_name = parts.last().ok_or_else(|| format!("path for {} has no parts", definition_path))?.to_string();

	let type_ref_path = swagger20::RefPath {
		path: definition_path.0.clone(),
		relative_to: ref_path_relative_to,
		can_be_default: None,
	};

	let derives = get_derives(&definition.kind, definitions)?;

	templates::type_header::generate(
		&mut out,
		definition_path,
		definition.description.as_ref().map(AsRef::as_ref),
		if is_under_api_feature { "#[cfg(feature = \"api\")]\n" } else { "" },
		derives,
		vis,
	)?;

	match &definition.kind {
		swagger20::SchemaKind::Properties(properties) => {
			let (template_properties, resource_metadata, metadata_property_ty) = {
				use std::fmt::Write;

				let mut result = Vec::with_capacity(properties.len());

				let single_group_version_kind =
					definition.kubernetes_group_kind_versions.as_ref()
					.and_then(|group_version_kinds| {
						if group_version_kinds.len() == 1 {
							Some(&group_version_kinds[0])
						}
						else {
							None
						}
					});
				let mut has_api_version = false;
				let mut has_kind = false;
				let mut metadata_property_ty = None;

				for (name, (schema, required)) in properties {
					if name.0 == "apiVersion" && single_group_version_kind.is_some() {
						has_api_version = true;
						continue;
					}

					if name.0 == "kind" && single_group_version_kind.is_some() {
						has_kind = true;
						continue;
					}

					let field_name = get_rust_ident(&name);

					let mut field_type_name = String::new();

					if !required {
						write!(field_type_name, "Option<")?;
					}

					let type_name = get_rust_type(&schema.kind, &replace_namespaces, crate_root)?;

					if name.0 == "metadata" {
						metadata_property_ty = Some((*required, type_name.clone()));
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
							) => write!(field_type_name, "Box<{}>", type_name)?,

							_ => write!(field_type_name, "{}", type_name)?,
						}
					}
					else {
						write!(field_type_name, "{}", type_name)?;
					};

					if !required {
						write!(field_type_name, ">")?;
					}

					let is_flattened =
						if let swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(_)) = &schema.kind {
							true
						}
						else {
							false
						};

					result.push(templates::Property {
						name,
						comment: schema.description.as_ref().map(AsRef::as_ref),
						field_name,
						field_type_name,
						required: *required,
						is_flattened,
					});
				}

				let resource_metadata = match (has_api_version, has_kind) {
					(true, true) => {
						let single_group_version_kind = single_group_version_kind.unwrap();
						if single_group_version_kind.group == "" {
							Some((
								format!("{:?}", single_group_version_kind.version),
								format!("{:?}", ""),
								format!("{:?}", single_group_version_kind.kind),
								format!("{:?}", single_group_version_kind.version),
							))
						}
						else {
							Some((
								format!("{:?}", format!("{}/{}", single_group_version_kind.group, single_group_version_kind.version)),
								format!("{:?}", single_group_version_kind.group),
								format!("{:?}", single_group_version_kind.kind),
								format!("{:?}", single_group_version_kind.version),
							))
						}
					},
					(false, false) => None,
					(true, false) => return Err(format!("{} has an apiVersion property but not a kind property", definition_path).into()),
					(false, true) => return Err(format!("{} has a kind property but not an apiVersion property", definition_path).into()),
				};

				(result, resource_metadata, metadata_property_ty)
			};

			let template_resource_metadata = match (&resource_metadata, &metadata_property_ty) {
				(Some((api_version, group, kind, version)), Some((metadata_required, metadata_type_name))) => Some(templates::ResourceMetadata {
					api_version,
					group,
					kind,
					version,
					is_listable: definition.has_corresponding_list_type,
					metadata_ty: Some((metadata_type_name, *metadata_required)),
				}),

				(Some((api_version, group, kind, version)), None) => Some(templates::ResourceMetadata {
					api_version,
					group,
					kind,
					version,
					is_listable: definition.has_corresponding_list_type,
					metadata_ty: None,
				}),

				(None, _) => None,
			};

			templates::r#struct::generate(
				&mut out,
				vis,
				&type_name,
				Default::default(),
				&template_properties,
			)?;

			if let Some(kubernetes_group_kind_versions) = &definition.kubernetes_group_kind_versions {
				let mut kubernetes_group_kind_versions: Vec<_> = kubernetes_group_kind_versions.iter().collect();
				kubernetes_group_kind_versions.sort();

				let mut operations_by_gkv: std::collections::BTreeMap<_, Vec<_>> = Default::default();
				for operation in std::mem::replace(operations, vec![]) {
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
									&replace_namespaces,
									crate_root,
									vis,
									&mut Some((&type_name, &type_ref_path)),
									is_under_api_feature || use_api_feature_for_operations)?;
							imports(operation_optional_parameters_name, operation_result_name)?;
							run_result.num_generated_apis += 1;
						}

						writeln!(out)?;
						writeln!(out, "// End {}/{}/{}",
							kubernetes_group_kind_version.group, kubernetes_group_kind_version.version, kubernetes_group_kind_version.kind)?;
					}
				}

				*operations = operations_by_gkv.into_iter().flat_map(|(_, operations)| operations).collect();
			}

			if let Some(template_resource_metadata) = &template_resource_metadata {
				templates::impl_resource::generate(
					&mut out,
					&type_name,
					Default::default(),
					crate_root,
					template_resource_metadata,
				)?;

				templates::impl_listable_resource::generate(
					&mut out,
					&type_name,
					Default::default(),
					crate_root,
					template_resource_metadata,
				)?;

				templates::impl_metadata::generate(
					&mut out,
					&type_name,
					Default::default(),
					crate_root,
					template_resource_metadata,
				)?;
			}

			templates::impl_deserialize::generate(
				&mut out,
				&type_name,
				Default::default(),
				&template_properties,
				crate_root,
				template_resource_metadata.as_ref(),
			)?;

			templates::impl_serialize::generate(
				&mut out,
				&type_name,
				Default::default(),
				&template_properties,
				crate_root,
				template_resource_metadata.as_ref(),
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ref(_) => return Err(format!("{} is a Ref", definition_path).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => {
			templates::int_or_string::generate(
				&mut out,
				&type_name,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrArray(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrBool(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::JSONSchemaPropsOrStringArray(_)) => {
			let (namespace, json_schema_props_or) = match ty {
				swagger20::Type::JSONSchemaPropsOrArray(namespace) => (namespace, templates::json_schema_props_or::Or::Array),
				swagger20::Type::JSONSchemaPropsOrBool(namespace) => (namespace, templates::json_schema_props_or::Or::Bool),
				swagger20::Type::JSONSchemaPropsOrStringArray(namespace) => (namespace, templates::json_schema_props_or::Or::StringArray),
				_ => unreachable!(),
			};

			let json_schema_props_type_name =
				get_fully_qualified_type_name(
					&swagger20::RefPath {
						path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", namespace),
						relative_to: swagger20::RefPathRelativeTo::Crate,
						can_be_default: None,
					},
					&replace_namespaces,
					crate_root)?;

			templates::json_schema_props_or::generate(
				&mut out,
				&type_name,
				json_schema_props_or,
				&json_schema_props_type_name,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => {
			templates::patch::generate(
				&mut out,
				&type_name,
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
					relative_to: swagger20::RefPathRelativeTo::Crate,
					can_be_default: None,
				}),
				&replace_namespaces,
				crate_root,
			)?;

			let error_other_rust_type = get_rust_type(
				&swagger20::SchemaKind::Ref(raw_extension_ref_path.clone()),
				&replace_namespaces,
				crate_root,
			)?;

			templates::watch_event::generate(
				&mut out,
				&type_name,
				has_bookmark_event_type,
				&error_status_rust_type,
				&error_other_rust_type,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { metadata }) => {
			let metadata_rust_type = get_rust_type(metadata, &replace_namespaces, crate_root)?;

			let template_generics_where_part = format!("T: {}::ListableResource", crate_root);
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
					required: true,
					is_flattened: false,
				},

				templates::Property {
					name: "metadata",
					comment: Some("Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"),
					field_name: "metadata".into(),
					field_type_name: format!("Option<{}>", metadata_rust_type),
					required: false,
					is_flattened: false,
				},
			];

			let template_resource_metadata = templates::ResourceMetadata {
				api_version: "<T as crate::Resource>::API_VERSION",
				group: "<T as crate::Resource>::GROUP",
				kind: "<T as crate::ListableResource>::LIST_KIND",
				version: "<T as crate::Resource>::VERSION",
				is_listable: false,
				metadata_ty: Some((&metadata_rust_type, false)),
			};

			templates::r#struct::generate(
				&mut out,
				vis,
				&type_name,
				template_generics,
				&template_properties,
			)?;

			templates::impl_resource::generate(
				&mut out,
				&type_name,
				template_generics,
				crate_root,
				&template_resource_metadata,
			)?;

			templates::impl_listable_resource::generate(
				&mut out,
				&type_name,
				template_generics,
				crate_root,
				&template_resource_metadata,
			)?;

			templates::impl_metadata::generate(
				&mut out,
				&type_name,
				template_generics,
				crate_root,
				&template_resource_metadata,
			)?;

			{
				let template_generics_where_part = format!("T: serde::Deserialize<'de> + {}::ListableResource", crate_root);
				let template_generics = templates::Generics {
					where_part: Some(&template_generics_where_part),
					..template_generics
				};

				templates::impl_deserialize::generate(
					&mut out,
					&type_name,
					template_generics,
					&template_properties,
					crate_root,
					Some(&template_resource_metadata),
				)?;
			}

			{
				let template_generics_where_part = format!("T: serde::Serialize + {}::ListableResource", crate_root);
				let template_generics = templates::Generics {
					where_part: Some(&template_generics_where_part),
					..template_generics
				};

				templates::impl_serialize::generate(
					&mut out,
					&type_name,
					template_generics,
					&template_properties,
					crate_root,
					Some(&template_resource_metadata),
				)?;
			}

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) => {
			let item_type_name = get_rust_type(items, &replace_namespaces, crate_root)?;
			let alias_type_name = format!("{}::List<{}>", crate_root, item_type_name);

			templates::type_alias::generate(
				&mut out,
				&type_name,
				&alias_type_name,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(ty @ swagger20::Type::CreateOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::DeleteOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::ListOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::PatchOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::ReplaceOptional(_)) |
		swagger20::SchemaKind::Ty(ty @ swagger20::Type::WatchOptional(_)) => {
			let properties = match ty {
				swagger20::Type::CreateOptional(properties) |
				swagger20::Type::DeleteOptional(properties) |
				swagger20::Type::ListOptional(properties) |
				swagger20::Type::PatchOptional(properties) |
				swagger20::Type::ReplaceOptional(properties) |
				swagger20::Type::WatchOptional(properties) => properties,
				_ => unreachable!(),
			};

			let template_properties = {
				let mut result = Vec::with_capacity(properties.len());

				for (name, schema) in properties {
					let field_name = get_rust_ident(name);

					let type_name = get_rust_borrow_type(&schema.kind, &replace_namespaces, crate_root)?;

					let field_type_name =
						if type_name.starts_with('&') {
							format!("Option<&'a {}>", &type_name[1..])
						}
						else {
							format!("Option<{}>", type_name)
						};

					result.push(templates::Property {
						name,
						comment: schema.description.as_ref().map(AsRef::as_ref),
						field_name,
						field_type_name,
						required: false,
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
				&type_name,
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
						&type_name,
						template_generics,
						vis,
						&template_properties,
						false,
					)?,

				swagger20::Type::DeleteOptional(_) =>
					templates::impl_serialize::generate(
						&mut out,
						&type_name,
						template_generics,
						&template_properties,
						crate_root,
						None,
					)?,

				swagger20::Type::WatchOptional(_) =>
					templates::query_string_optional::generate(
						&mut out,
						&type_name,
						template_generics,
						vis,
						&template_properties,
						true,
					)?,

				_ => unreachable!(),
			}

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::Create,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::Delete,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::List,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::Patch,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::Replace,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => {
			templates::operation_response_common::generate(
				&mut out,
				&type_name,
				crate_root,
				templates::operation_response_common::OperationAction::Watch,
			)?;

			run_result.num_generated_structs += 1;
		},

		swagger20::SchemaKind::Ty(_) => {
			let inner_type_name = get_rust_type(&definition.kind, &replace_namespaces, crate_root)?;

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
				&type_name,
				&inner_type_name,
				datetime_serialization_format,
			)?;

			run_result.num_generated_type_aliases += 1;
		},
	}

	Ok(run_result)
}

fn get_derives(
	kind: &swagger20::SchemaKind,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
) -> Result<Option<templates::type_header::Derives>, Error> {
	if matches!(kind, swagger20::SchemaKind::Ty(swagger20::Type::ListRef { .. })) {
		// ListRef is emitted as a type alias.
		return Ok(None);
	}

	let derive_clone = evaluate_trait_bound(kind, true, definitions, |kind, _| match kind {
		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => Ok(false),

		_ => Ok(true),
	})?;

	let derive_copy = derive_clone && evaluate_trait_bound(kind, false, definitions, |kind, _| match kind {
		swagger20::SchemaKind::Ty(swagger20::Type::CreateOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceOptional(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchOptional(_)) => Ok(true),

		_ => Ok(false),
	})?;

	#[allow(clippy::match_same_arms)]
	let is_default = evaluate_trait_bound(kind, false, definitions, |kind, required| match kind {
		// Option<T>::default is None regardless of T
		_ if !required => Ok(true),

		swagger20::SchemaKind::Ref(swagger20::RefPath { can_be_default: Some(can_be_default), .. }) => Ok(*can_be_default),

		swagger20::SchemaKind::Ref(swagger20::RefPath { relative_to: swagger20::RefPathRelativeTo::Scope, .. }) => Ok(false),

		// Handled by evaluate_trait_bound
		swagger20::SchemaKind::Ref(swagger20::RefPath { relative_to: swagger20::RefPathRelativeTo::Crate, .. }) => unreachable!(),

		// chrono::DateTime<chrono::Utc> is not Default
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok(false),

		// Enums without a default value
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => Ok(false),

		_ => Ok(true),
	})?;
	let derive_default = is_default && match kind {
		// IntOrString has a manual Default impl, so don't #[derive] it.
		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => false,
		_ => true,
	};

	let derive_partial_eq = evaluate_trait_bound(kind, true, definitions, |kind, _| match kind {
		swagger20::SchemaKind::Ty(swagger20::Type::CreateResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::DeleteResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ListResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::PatchResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::ReplaceResponse) |
		swagger20::SchemaKind::Ty(swagger20::Type::WatchResponse) => Ok(false),

		_ => Ok(true),
	})?;

	// The choice of deriving Eq, Ord and PartialOrd is deliberately more conservative than the choice of deriving PartialEq,
	// so as to not change dramatically between Kubernetes versions. For example, ObjectMeta is Ord in v1.15 but not in v1.16 because
	// it indirectly gained a serde_json::Value field (`managed_fields.fields_v1.0`).
	//
	// Also, being conservative means the types generated by #[derive(k8s_openapi_derive::CustomResource)] don't have to require them either.

	let derive_eq = derive_partial_eq && match kind {
		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) |
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => true,

		_ => false,
	};

	let derive_partial_ord = derive_partial_eq && match kind {
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => true,

		_ => false,
	};

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

fn evaluate_trait_bound(
	kind: &swagger20::SchemaKind,
	array_follows_elements: bool,
	definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
	mut f: impl FnMut(&swagger20::SchemaKind, bool) -> Result<bool, Error>,
) -> Result<bool, Error> {
	fn evaluate_trait_bound_inner<'a>(
		kind: &std::borrow::Cow<'a, swagger20::SchemaKind>,
		required: bool,
		array_follows_elements: bool,
		definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
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
							&mut visited,
							f,
						)?;
					if !field_bound {
						return Ok(false);
					}
				}

				Ok(true)
			},

			swagger20::SchemaKind::Ref(swagger20::RefPath { path, relative_to: swagger20::RefPathRelativeTo::Crate, .. }) =>
				if let Some(target) = definitions.get(&swagger20::DefinitionPath(path.to_owned())) {
					let mut visited = visited.clone();
					evaluate_trait_bound_inner(
						&std::borrow::Cow::Borrowed(&target.kind),
						required,
						array_follows_elements,
						definitions,
						&mut visited,
						f,
					)
				}
				else {
					f(&kind, required)
				},

			swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) if array_follows_elements =>
				evaluate_trait_bound_inner(
					&std::borrow::Cow::Owned(items.kind.clone()),
					required,
					array_follows_elements,
					definitions,
					visited,
					f,
				),

			swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray(namespace)) |
			swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool(namespace)) |
			swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray(namespace)) => {
				let json_schema_props_ref_path = swagger20::RefPath {
					path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", namespace),
					relative_to: swagger20::RefPathRelativeTo::Crate,
					can_be_default: None,
				};
				let json_schema_props_bound =
					evaluate_trait_bound_inner(
						&std::borrow::Cow::Owned(swagger20::SchemaKind::Ref(json_schema_props_ref_path)),
						required,
						array_follows_elements,
						definitions,
						visited,
						f,
					)?;
				if !json_schema_props_bound {
					return Ok(false);
				}

				f(&kind, required)
			},

			swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(raw_extension_ref_path)) => {
				let raw_extension_bound =
					evaluate_trait_bound_inner(
						&std::borrow::Cow::Owned(swagger20::SchemaKind::Ref(raw_extension_ref_path.clone())),
						required,
						array_follows_elements,
						definitions,
						visited,
						f,
					)?;
				if !raw_extension_bound {
					return Ok(false);
				}

				f(&kind, required)
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
		&mut visited,
		&mut f,
	)
}

fn get_comment_text<'a>(s: &'a str, indent: &'a str) -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'a {
	s.lines().map(move |line|
		if line.is_empty() {
			"".into()
		}
		else {
			let line =
				line
				.replace(r"\", r"\\")
				.replace("[", r"\[")
				.replace("]", r"\]")
				.replace("<", r"\<")
				.replace(">", r"\>");
			format!("{} {}", indent, line).into()
		})
}

fn get_fully_qualified_type_name(
	ref_path: &swagger20::RefPath,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
) -> Result<String, Error> {
	use std::fmt::Write;

	match ref_path.relative_to {
		swagger20::RefPathRelativeTo::Crate => {
			let mut result = crate_root.to_owned();

			let parts = replace_namespace(ref_path.path.split('.'), replace_namespaces);

			for part in parts.iter().rev().skip(1).rev() {
				write!(result, "::{}", get_rust_ident(part))?;
			}

			write!(result, "::{}", parts.last().ok_or_else(|| format!("path for {} has no parts", ref_path.path))?)?;

			Ok(result)
		},

		swagger20::RefPathRelativeTo::Scope => {
			let last_part = ref_path.path.split('.').last().ok_or_else(|| format!("path for {} has no parts", ref_path.path))?;
			Ok(last_part.to_owned())
		},
	}
}

#[doc(hidden)]
pub fn get_rust_ident(name: &str) -> std::borrow::Cow<'static, str> {
	// Fix cases of invalid rust idents
	match name {
		"$ref" => return "ref_path".into(),
		"$schema" => return "schema".into(),
		"continue" => return "continue_".into(),
		"enum" => return "enum_".into(),
		"type" => return "type_".into(),
		_ => (),
	}

	// Some cases of "ABc" should be converted to "abc" instead of "a_bc".
	// Eg "JSONSchemas" => "json_schemas", but "externalIPs" => "external_ips" instead of "external_i_ps".
	// Mostly happens with plurals of abbreviations.
	match name {
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
				'-' => '_',
				c => c,
			});
		}
	}

	result.into()
}

fn get_rust_borrow_type(
	schema_kind: &swagger20::SchemaKind,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.CreateOptional" =>
			Ok(format!("{}::CreateOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.DeleteOptional" =>
			Ok(format!("{}::DeleteOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.ListOptional" =>
			Ok(format!("{}::ListOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.PatchOptional" =>
			Ok(format!("{}::PatchOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.ReplaceOptional" =>
			Ok(format!("{}::ReplaceOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) if path == "io.k8s.WatchOptional" =>
			Ok(format!("{}::WatchOptional<'_>", crate_root).into()),

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(format!("&{}", get_fully_qualified_type_name(ref_path, replace_namespaces, crate_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("&serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("&[{}]", get_rust_type(&items.kind, replace_namespaces, crate_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("&std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, crate_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok(format!("&{}::ByteString", crate_root).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("&chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("&str".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(namespace)) =>
			Ok(format!("&{}::apiextensions_apiserver::pkg::apis::apiextensions::{}::CustomResourceSubresources", crate_root, namespace).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray(_)) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { .. }) => Err("ListDef type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) =>
			Ok(format!("&{}::List<{}>", crate_root, get_rust_type(items, replace_namespaces, crate_root)?).into()),

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
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
) -> Result<std::borrow::Cow<'static, str>, Error> {
	match schema_kind {
		swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

		swagger20::SchemaKind::Ref(ref_path) =>
			Ok(get_fully_qualified_type_name(ref_path, replace_namespaces, crate_root)?.into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok("serde_json::Value".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
			Ok(format!("Vec<{}>", get_rust_type(&items.kind, replace_namespaces, crate_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
			Ok(format!("std::collections::BTreeMap<String, {}>", get_rust_type(&additional_properties.kind, replace_namespaces, crate_root)?).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) => Ok(format!("{}::ByteString", crate_root).into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) => Ok("chrono::DateTime<chrono::Utc>".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("String".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(namespace)) =>
			Ok(format!("{}::apiextensions_apiserver::pkg::apis::apiextensions::{}::CustomResourceSubresources", crate_root, namespace).into()),

		swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrArray(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrBool(_)) |
		swagger20::SchemaKind::Ty(swagger20::Type::JSONSchemaPropsOrStringArray(_)) => Err("JSON schema types not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),

		swagger20::SchemaKind::Ty(swagger20::Type::ListDef { .. }) => Err("ListDef type not supported".into()),
		swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) =>
			Ok(format!("{}::List<{}>", crate_root, get_rust_type(items, replace_namespaces, crate_root)?).into()),

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

fn replace_namespace<'a, I>(
	parts: I,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
) -> Vec<std::borrow::Cow<'a, str>> where I: IntoIterator<Item = &'a str> {
	let parts: Vec<_> = parts.into_iter().map(Into::into).collect();

	log::trace!("parts = {:?}, replace_namespaces = {:?}", parts, replace_namespaces);

	for (from, to) in replace_namespaces {
		if parts.starts_with(from) {
			let mut result = to.to_vec();
			result.extend(parts.into_iter().skip(from.len()));
			return result;
		}
	}

	parts
}

#[doc(hidden)]
pub fn write_operation(
	out: &mut impl std::io::Write,
	operation: &swagger20::Operation,
	replace_namespaces: &[(&[std::borrow::Cow<'static, str>], &[std::borrow::Cow<'static, str>])],
	crate_root: &str,
	vis: &str,
	type_name_and_ref_path: &mut Option<(&str, &swagger20::RefPath)>,
	is_under_api_feature: bool,
) -> Result<(Option<String>, Option<String>), Error> {
	writeln!(out)?;

	writeln!(out, "// Generated from operation {}", operation.id)?;

	let (operation_fn_name, operation_result_name, operation_optional_parameters_name) =
		get_operation_names(operation, type_name_and_ref_path.is_some())?;

	let indent = if type_name_and_ref_path.is_some() { "    " } else { "" };

	writeln!(out)?;

	if let Some((type_name, _)) = type_name_and_ref_path {
		writeln!(out, "impl {} {{", type_name)?;
	}

	let parameters: Vec<_> = operation.parameters.iter().map(std::ops::Deref::deref).collect();
	let mut previous_parameters: std::collections::HashSet<_> = Default::default();
	let parameters: Result<Vec<_>, Error> =
		parameters.into_iter()
		.map(|parameter| {
			let mut parameter_name = get_rust_ident(&parameter.name);
			while previous_parameters.contains(&parameter_name) {
				parameter_name = format!("{}_", parameter_name).into();
			}
			previous_parameters.insert(parameter_name.clone());

			let parameter_type = match get_rust_borrow_type(&parameter.schema.kind, replace_namespaces, crate_root) {
				Ok(parameter_type) => parameter_type,
				Err(err) => return Err(err),
			};

			Ok((parameter_name, parameter_type, parameter))
		})
		.collect();
	let mut parameters = parameters?;

	let delete_optional_parameter =
		if let Some(index) =
			parameters.iter()
			.position(|(_, _, parameter)|
				if let swagger20::SchemaKind::Ref(swagger20::RefPath { path, .. }) = &parameter.schema.kind {
					path == "io.k8s.DeleteOptional"
				}
				else {
					false
				})
		{
			Some(parameters.swap_remove(index))
		}
		else {
			None
		};

	let query_string_optional_parameter =
		if let Some(index) =
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
		{
			Some(parameters.swap_remove(index))
		}
		else {
			None
		};

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
			"{}/// Use the returned [`{}::ResponseBody`]`<`[`{}`]`>` constructor, or [`{}`] directly, to parse the HTTP response.",
			indent, crate_root, operation_result_name, operation_result_name)?;
		need_empty_line = true;
	}
	else {
		let common_response_type_link = match operation.responses {
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::CreateResponse) => Some(format!("[`{}::CreateResponse`]`<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse) => match operation.kubernetes_action {
				Some(swagger20::KubernetesAction::Delete) => Some(format!("[`{}::DeleteResponse`]`<Self>", crate_root)),
				Some(swagger20::KubernetesAction::DeleteCollection) => Some(format!("[`{}::DeleteResponse`]`<`[`{}::List`]`<Self>>", crate_root, crate_root)),
				_ => unreachable!(),
			},
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ListResponse) => Some(format!("[`{}::ListResponse`]`<Self>`", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::PatchResponse) => Some(format!("[`{}::PatchResponse`]`<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ReplaceResponse) => Some(format!("[`{}::ReplaceResponse`]`<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::WatchResponse) => Some(format!("[`{}::WatchResponse`]`<Self>", crate_root)),
			_ => None,
		};

		if let Some(common_response_type_link) = common_response_type_link {
			if need_empty_line {
				writeln!(out, "{}///", indent)?;
			}

			writeln!(out,
				"{}/// Use the returned [`{}::ResponseBody`]`<`{}>` constructor, or {}` directly, to parse the HTTP response.",
				indent, crate_root, common_response_type_link, common_response_type_link)?;
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

	if is_under_api_feature {
		writeln!(out, r#"{}#[cfg(feature = "api")]"#, indent)?;
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
			"{}) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> {}::ResponseBody<{}>), {}::RequestError> {{",
			indent, crate_root, operation_result_name, crate_root)?;
	}
	else {
		let common_response_type = match operation.responses {
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::CreateResponse) => Some(format!("{}::CreateResponse<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse) => match operation.kubernetes_action {
				Some(swagger20::KubernetesAction::Delete) => Some(format!("{}::DeleteResponse<Self>", crate_root)),
				Some(swagger20::KubernetesAction::DeleteCollection) => Some(format!("{}::DeleteResponse<{}::List<Self>>", crate_root, crate_root)),
				_ => unreachable!(),
			},
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ListResponse) => Some(format!("{}::ListResponse<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::PatchResponse) => Some(format!("{}::PatchResponse<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ReplaceResponse) => Some(format!("{}::ReplaceResponse<Self>", crate_root)),
			crate::swagger20::OperationResponses::Common(crate::swagger20::Type::WatchResponse) => Some(format!("{}::WatchResponse<Self>", crate_root)),
			_ => None,
		};

		if let Some(common_response_type) = common_response_type {
			writeln!(out,
				"{}) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> {}::ResponseBody<{}>), {}::RequestError> {{",
				indent, crate_root, common_response_type, crate_root)?;
		}
		else {
			writeln!(out, "{}) -> Result<http::Request<Vec<u8>>, {}::RequestError> {{", indent, crate_root)?;
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
						"{}        {} = {}::percent_encoding::percent_encode({}.as_bytes(), {}::percent_encoding2::PATH_SEGMENT_ENCODE_SET),",
						indent,
						parameter_name,
						crate_root,
						parameter_name,
						crate_root)?;
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
		writeln!(out, "{}    let mut __query_pairs = {}::url::form_urlencoded::Serializer::new(__url);", indent, crate_root)?;
		if let Some((parameter_name, _, _)) = &query_string_optional_parameter {
			writeln!(out, "{}    {}.__serialize(&mut __query_pairs);", indent, parameter_name)?;
		}
		else {
			for (parameter_name, parameter_type, parameter) in &parameters {
				if parameter.location == swagger20::ParameterLocation::Query {
					if parameter.required {
						match parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
							swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
							swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
								writeln!(out, r#"{}    __query_pairs.append_pair({:?}, &{}.to_string());"#, indent, parameter.name, parameter_name)?,

							swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) =>
								writeln!(out, r#"{}    __query_pairs.append_pair({:?}, &{});"#, indent, parameter.name, parameter_name)?,

							_ => return Err(format!("parameter {} is in the query string but is a {:?}", parameter.name, parameter_type).into()),
						}
					}
					else {
						writeln!(out, "{}    if let Some({}) = {} {{", indent, parameter_name, parameter_name)?;
						match parameter.schema.kind {
							swagger20::SchemaKind::Ty(swagger20::Type::Boolean) |
							swagger20::SchemaKind::Ty(swagger20::Type::Integer { .. }) |
							swagger20::SchemaKind::Ty(swagger20::Type::Number { .. }) =>
								writeln!(out, r#"{}        __query_pairs.append_pair({:?}, &{}.to_string());"#, indent, parameter.name, parameter_name)?,

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

	writeln!(out, "{}    let __request = http::Request::{}(__url);", indent, method)?;

	let body_parameter =
		delete_optional_parameter.as_ref()
		.or_else(|| parameters.iter().find(|(_, _, parameter)| parameter.location == swagger20::ParameterLocation::Body));

	write!(out, "{}    let __body = ", indent)?;
	if let Some((parameter_name, parameter_type, parameter)) = body_parameter {
		if parameter.required {
			if parameter_type.starts_with('&') {
				writeln!(out, "serde_json::to_vec({}).map_err({}::RequestError::Json)?;", parameter_name, crate_root)?;
			}
			else {
				writeln!(out, "serde_json::to_vec(&{}).map_err({}::RequestError::Json)?;", parameter_name, crate_root)?;
			}
		}
		else {
			writeln!(out)?;
			writeln!(out, "{}.unwrap_or(Ok(vec![]), |value| serde_json::to_vec(value).map_err({}::RequestError::Json))?;", parameter_name, crate_root)?;
		}

		let is_patch =
			if let swagger20::SchemaKind::Ref(ref_path) = &parameter.schema.kind {
				ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.Patch"
			}
			else {
				false
			};
		if is_patch {
			let patch_type = get_rust_type(&parameter.schema.kind, replace_namespaces, crate_root)?;
			writeln!(out, "{}    let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static(match {} {{", indent, parameter_name)?;
			writeln!(out, r#"{}        {}::Json(_) => "application/json-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::Merge(_) => "application/merge-patch+json","#, indent, patch_type)?;
			writeln!(out, r#"{}        {}::StrategicMerge(_) => "application/strategic-merge-patch+json","#, indent, patch_type)?;
			writeln!(out, "{}    }}));", indent)?;
		}
		else {
			writeln!(out, r#"{}    let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));"#, indent)?;
		}
	}
	else {
		writeln!(out, "vec![];")?;
	}

	if operation_result_name.is_some() {
		writeln!(out, "{}    match __request.body(__body) {{", indent)?;
		writeln!(out, "{}        Ok(request) => Ok((request, {}::ResponseBody::new)),", indent, crate_root)?;
		writeln!(out, "{}        Err(err) => Err({}::RequestError::Http(err)),", indent, crate_root)?;
		writeln!(out, "{}    }}", indent)?;
	}
	else {
		let is_common_response_type = match operation.responses {
			crate::swagger20::OperationResponses::Common(_) => true,
			_ => false,
		};

		if is_common_response_type {
			writeln!(out, "{}    match __request.body(__body) {{", indent)?;
			writeln!(out, "{}        Ok(request) => Ok((request, {}::ResponseBody::new)),", indent, crate_root)?;
			writeln!(out, "{}        Err(err) => Err({}::RequestError::Http(err)),", indent, crate_root)?;
			writeln!(out, "{}    }}", indent)?;
		}
		else {
			writeln!(out, "{}    __request.body(__body).map_err({}::RequestError::Http)", indent, crate_root)?;
		}
	}
	writeln!(out, "{}}}", indent)?;

	if type_name_and_ref_path.is_some() {
		writeln!(out, "}}")?;
	}

	if !optional_parameters.is_empty() {
		writeln!(out)?;

		if let Some((type_name, _)) = type_name_and_ref_path {
			writeln!(out, "/// Optional parameters of [`{}::{}`]", type_name, operation_fn_name)?;
		}
		else {
			writeln!(out, "/// Optional parameters of [`{}`]", operation_fn_name)?;
		}

		if is_under_api_feature {
			writeln!(out, r#"#[cfg(feature = "api")]"#)?;
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
			if parameter_type.starts_with('&') {
				writeln!(out, "    {}{}: Option<&'a {}>,", vis, parameter_name, &parameter_type[1..])?;
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

			if let Some((type_name, _)) = type_name_and_ref_path {
				writeln!(out,
					"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}::{}`]",
					operation_result_name, type_name, operation_fn_name)?;
			}
			else {
				writeln!(out,
					"/// Use `<{} as Response>::try_from_parts` to parse the HTTP response body of [`{}`]",
					operation_result_name, operation_fn_name)?;
			}

			if is_under_api_feature {
				writeln!(out, r#"#[cfg(feature = "api")]"#)?;
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
				writeln!(out, "    {}({}),", variant_name, get_rust_type(&schema.kind, replace_namespaces, crate_root)?)?;
			}

			writeln!(out, "    Other(Result<Option<serde_json::Value>, serde_json::Error>),")?;
			writeln!(out, "}}")?;
			writeln!(out)?;

			if is_under_api_feature {
				writeln!(out, r#"#[cfg(feature = "api")]"#)?;
			}
			writeln!(out, "impl {}::Response for {} {{", crate_root, operation_result_name)?;
			writeln!(out, "    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {}::ResponseError> {{", crate_root)?;

			writeln!(out, "        match status_code {{")?;
			for &(http_status_code, variant_name, schema) in &operation_responses {
				writeln!(out, "            http::StatusCode::{} => {{", http_status_code)?;

				match &schema.kind {
					swagger20::SchemaKind::Ty(swagger20::Type::String { .. }) => {
						writeln!(out, "                if buf.is_empty() {{")?;
						writeln!(out, "                    return Err({}::ResponseError::NeedMoreData);", crate_root)?;
						writeln!(out, "                }}")?;
						writeln!(out)?;
						writeln!(out, "                let (result, len) = match std::str::from_utf8(buf) {{")?;
						writeln!(out, "                    Ok(s) => (s, buf.len()),")?;
						writeln!(out, "                    Err(err) => match (err.valid_up_to(), err.error_len()) {{")?;
						writeln!(out, "                        (0, Some(_)) => return Err({}::ResponseError::Utf8(err)),", crate_root)?;
						writeln!(out, "                        (0, None) => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
						writeln!(out, "                        (valid_up_to, _) => (")?;
						writeln!(out, "                            unsafe {{ std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) }},")?;
						writeln!(out, "                            valid_up_to,")?;
						writeln!(out, "                        ),")?;
						writeln!(out, "                    }},")?;
						writeln!(out, "                }};")?;
						writeln!(out, "                Ok(({}::{}(result.to_owned()), len))", operation_result_name, variant_name)?;
					},

					swagger20::SchemaKind::Ref(_) => {
						writeln!(out, "                let result = match serde_json::from_slice(buf) {{")?;
						writeln!(out, "                    Ok(value) => value,")?;
						writeln!(out, "                    Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
						writeln!(out, "                    Err(err) => return Err({}::ResponseError::Json(err)),", crate_root)?;
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
			writeln!(out, "                        match serde_json::from_slice(buf) {{")?;
			writeln!(out, "                            Ok(value) => (Ok(Some(value)), buf.len()),")?;
			writeln!(out, "                            Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
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
	if type_name_and_ref_path.is_some() && !optional_parameters.is_empty() {
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
			// For functions associatd with types (eg `Pod::list_core_v1_namespaced_pod`), the API version contained in the operation name
			// is already obvious from the type's path (`core::v1::Pod`), so it can be stripped (`list_namespaced_pod`).
			let tag: String =
				operation.tag.split('_')
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
