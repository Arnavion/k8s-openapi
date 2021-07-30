use k8s_openapi_codegen_common::swagger20;

use super::ResultExt;

#[derive(Debug)]
pub(super) struct CustomResourceDefinition {
	ident: proc_macro2::Ident,
	vis: syn::Visibility,
	tokens: proc_macro2::TokenStream,

	group: String,
	version: String,
	plural: String,
	generate_schema: bool,
	namespaced: bool,
	has_subresources: Option<String>,
}

impl super::CustomDerive for CustomResourceDefinition {
	fn parse(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
		let ident = input.ident;
		let vis = input.vis;

		let mut group = None;
		let mut plural = None;
		let mut version = None;
		let mut has_subresources = None;

		let mut generate_schema = false;
		let mut namespaced = false;

		for attr in &input.attrs {
			if attr.style != syn::AttrStyle::Outer {
				continue;
			}

			if !attr.path.is_ident("custom_resource_definition") {
				continue;
			}

			let metas = match attr.parse_meta()? {
				syn::Meta::List(meta) => meta.nested,
				meta => return Err(r#"#[custom_resource_definition] expects a list of metas, like `#[custom_resource_definition(...)]`"#).spanning(meta),
			};

			for meta in metas {
				let meta: &dyn quote::ToTokens = match &meta {
					syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) =>
						if meta.path.is_ident("group") {
							if let syn::Lit::Str(lit) = &meta.lit {
								group = Some(lit.value());
								continue;
							}

							return Err(r#"#[custom_resource_definition(group = "...")] expects a string literal value"#).spanning(meta);
						}
						else if meta.path.is_ident("plural") {
							if let syn::Lit::Str(lit) = &meta.lit {
								plural = Some(lit.value());
								continue;
							}

							return Err(r#"#[custom_resource_definition(plural = "...")] expects a string literal value"#).spanning(meta);
						}
						else if meta.path.is_ident("version") {
							if let syn::Lit::Str(lit) = &meta.lit {
								version = Some(lit.value());
								continue;
							}

							return Err(r#"#[custom_resource_definition(version = "...")] expects a string literal value"#).spanning(meta);
						}
						else if meta.path.is_ident("has_subresources") {
							if let syn::Lit::Str(lit) = &meta.lit {
								has_subresources = Some(lit.value());
								continue;
							}

							return Err(r#"#[custom_resource_definition(has_subresources = "...")] expects a string literal value"#).spanning(meta);
						}
						else {
							meta
						},

					syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
						if path.is_ident("generate_schema") {
							generate_schema = true;
							continue;
						}
						else if path.is_ident("namespaced") {
							namespaced = true;
							continue;
						}
						else {
							&meta
						},

					meta => meta,
				};

				return
					Err(r#"\
						#[derive(CustomResourceDefinition)] found unexpected meta. \
						Expected `group = "..."`, `namespaced`, `plural = "..."`, `version = "..." or `has_subresources` = "..."`"#)
					.spanning(meta);
			}
		}

		let group =
			group
			.ok_or(r#"#[derive(CustomResourceDefinition)] did not find a #[custom_resource_definition(group = "...")] attribute on the struct"#)
			.spanning(&tokens)?;
		let version =
			version
			.ok_or(r#"#[derive(CustomResourceDefinition)] did not find a #[custom_resource_definition(version = "...")] attribute on the struct"#)
			.spanning(&tokens)?;
		let plural =
			plural
			.ok_or(r#"#[derive(CustomResourceDefinition)] did not find a #[custom_resource_definition(plural = "...")] attribute on the struct"#)
			.spanning(&tokens)?;

		Ok(CustomResourceDefinition {
			ident,
			vis,
			tokens,

			group,
			version,
			plural,
			generate_schema,
			namespaced,
			has_subresources,
		})
	}

	fn emit(self) -> Result<proc_macro2::TokenStream, syn::Error> {
		let CustomResourceDefinition { ident: cr_spec_name, vis, tokens, group, version, plural, generate_schema, namespaced, has_subresources } = self;

		let vis: std::borrow::Cow<'_, str> = match vis {
			syn::Visibility::Inherited => "".into(),
			vis => format!("{} ", quote::ToTokens::into_token_stream(vis)).into(),
		};

		let (cr_spec_name, cr_name) = {
			let cr_spec_name_string = cr_spec_name.to_string();
			if !cr_spec_name_string.ends_with("Spec") {
				return Err("#[derive(CustomResourceDefinition)] requires the name of the struct to end with `Spec`").spanning(cr_spec_name);
			}
			let cr_name_string = cr_spec_name_string[..(cr_spec_name_string.len() - 4)].to_owned();
			(cr_spec_name_string, cr_name_string)
		};
		let cr_list_name = format!("{}List", cr_name);

		let body_parameter =
			std::sync::Arc::new(swagger20::Parameter {
				location: swagger20::ParameterLocation::Body,
				name: "body".to_owned(),
				required: true,
				schema: swagger20::Schema {
					description: None,
					kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
						path: cr_name.clone(),
						can_be_default: None,
					}),
					kubernetes_group_kind_versions: vec![],
					list_kind: None,
				},
			});

		let name_parameter =
			std::sync::Arc::new(swagger20::Parameter {
				location: swagger20::ParameterLocation::Path,
				name: "name".to_owned(),
				required: true,
				schema: swagger20::Schema {
					description: Some(format!("name of the {}", cr_name)),
					kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
					kubernetes_group_kind_versions: vec![],
					list_kind: None,
				},
			});

		let (namespace_operation_id_component, namespace_parameter, namespace_path_component) =
			if namespaced {
				("Namespaced", Some(std::sync::Arc::new(swagger20::Parameter {
					location: swagger20::ParameterLocation::Path,
					name: "namespace".to_owned(),
					required: true,
					schema: swagger20::Schema {
						description: Some("object name and auth scope, such as for teams and projects".to_owned()),
						kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
						kubernetes_group_kind_versions: vec![],
						list_kind: None,
					},
				})), "/namespaces/{namespace}")
			}
			else {
				("", None, "")
			};

		let mut spec = swagger20::Spec {
			info: swagger20::Info {
				title: String::new(),
				version: String::new(),
			},
			definitions: vec![
				(swagger20::DefinitionPath(cr_name.clone()), swagger20::Schema {
					description: Some(format!("Custom resource for {}", cr_spec_name)),
					kind: swagger20::SchemaKind::Properties(vec![
						(swagger20::PropertyName("apiVersion".to_owned()), (swagger20::Schema {
							description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}, false)),
						(swagger20::PropertyName("kind".to_owned()), (swagger20::Schema {
							description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}, false)),
						(swagger20::PropertyName("metadata".to_owned()), (swagger20::Schema {
							description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_owned(),
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}, true)),
						(swagger20::PropertyName("spec".to_owned()), (swagger20::Schema {
							description: Some(format!("Specification of the {} custom resource", cr_name)),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_spec_name,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}, false)),
					].into_iter().chain(
						has_subresources.map(|has_subresources|
							(swagger20::PropertyName("subresources".to_owned()), (swagger20::Schema {
								description: Some(format!("Subresources of the {} custom resource", cr_name)),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(has_subresources)),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							}, true)))
					).collect()),
					kubernetes_group_kind_versions: vec![
						swagger20::KubernetesGroupKindVersion {
							group: group.clone(),
							kind: cr_name.clone(),
							version: version.clone(),
						},
					],
					list_kind: Some(cr_list_name.clone()),
				}),

				(swagger20::DefinitionPath(cr_list_name.clone()), swagger20::Schema {
					description: Some(format!("{} is a list of {}", cr_list_name, cr_name)),
					kind: swagger20::SchemaKind::Ty(swagger20::Type::ListRef {
						items: Box::new(swagger20::SchemaKind::Ref(swagger20::RefPath {
							path: cr_name.clone(),
							can_be_default: Some(true),
						})),
					}),
					kubernetes_group_kind_versions: vec![
						swagger20::KubernetesGroupKindVersion {
							group: group.clone(),
							kind: cr_list_name.clone(),
							version: version.clone(),
						},
					],
					list_kind: None,
				}),
			].into_iter().collect(),
			operations: vec![
				swagger20::Operation {
					description: Some(format!("Create a {}", cr_name)),
					id: format!("create{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Post,
					kubernetes_action: Some(swagger20::KubernetesAction::Post),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(body_parameter.clone()),
						namespace_parameter.clone(),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::CreateResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Delete a {}", cr_name)),
					id: format!("delete{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Delete,
					kubernetes_action: Some(swagger20::KubernetesAction::Delete),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.DeleteOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::DeleteResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Delete a collection of objects of kind {}", cr_name)),
					id: format!("deleteCollection{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Delete,
					kubernetes_action: Some(swagger20::KubernetesAction::DeleteCollection),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "deleteOptional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: Some("Delete options. Use `Default::default()` to not pass any.".to_owned()),
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.DeleteOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "listOptional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: Some("List options. Use `Default::default()` to not pass any.".to_owned()),
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.ListOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::DeleteResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("List objects of kind {}", cr_name)),
					id: format!("list{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Get,
					kubernetes_action: Some(swagger20::KubernetesAction::List),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.ListOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::ListResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Partially update the specified {}", cr_name)),
					id: format!("patch{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Patch,
					kubernetes_action: Some(swagger20::KubernetesAction::Patch),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "body".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.apimachinery.pkg.apis.meta.v1.Patch".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.PatchOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::PatchResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Partially update the state of the specified {}", cr_name)),
					id: format!("patch{}{}Status", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Patch,
					kubernetes_action: Some(swagger20::KubernetesAction::Patch),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "body".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.apimachinery.pkg.apis.meta.v1.Patch".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.PatchOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}/status", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::PatchResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Read the specified {}", cr_name)),
					id: format!("read{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Get,
					kubernetes_action: Some(swagger20::KubernetesAction::Get),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Map(vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}),
					].into_iter().collect()),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Read status of the specified {}", cr_name)),
					id: format!("read{}{}Status", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Get,
					kubernetes_action: Some(swagger20::KubernetesAction::Get),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}/status", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Map(vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: vec![],
							list_kind: None,
						}),
					].into_iter().collect()),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Replace the specified {}", cr_name)),
					id: format!("replace{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Put,
					kubernetes_action: Some(swagger20::KubernetesAction::Put),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(body_parameter.clone()),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.ReplaceOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::ReplaceResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Replace status of the specified {}", cr_name)),
					id: format!("replace{}{}Status", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Put,
					kubernetes_action: Some(swagger20::KubernetesAction::Put),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						Some(body_parameter),
						Some(name_parameter),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Body,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.ReplaceOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}/status", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::ReplaceResponse),
					tag: None,
				},

				swagger20::Operation {
					description: Some(format!("Watch objects of kind {}", cr_name)),
					id: format!("watch{}{}", namespace_operation_id_component, cr_name),
					method: swagger20::Method::Get,
					kubernetes_action: Some(swagger20::KubernetesAction::Watch),
					kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
						group: group.clone(),
						kind: cr_name.clone(),
						version: version.clone(),
					}),
					parameters: vec![
						namespace_parameter,
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.WatchOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: swagger20::OperationResponses::Common(swagger20::Type::WatchResponse),
					tag: None,
				},
			],
		};

		let mut run_state = RunState {
			writer: vec![],
		};

		let _ =
			k8s_openapi_codegen_common::run(
				&spec.definitions,
				&mut spec.operations,
				&swagger20::DefinitionPath(cr_name),
				&MapNamespace,
				&vis,
				if generate_schema { k8s_openapi_codegen_common::GenerateSchema::Yes { feature: None } } else { k8s_openapi_codegen_common::GenerateSchema::No },
				None,
				&mut run_state,
			)
			.map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err))
			.spanning(&tokens)?;

		let _ =
			k8s_openapi_codegen_common::run(
				&spec.definitions,
				&mut spec.operations,
				&swagger20::DefinitionPath(cr_list_name),
				&MapNamespace,
				&vis,
				if generate_schema { k8s_openapi_codegen_common::GenerateSchema::Yes { feature: None } } else { k8s_openapi_codegen_common::GenerateSchema::No },
				None,
				&mut run_state,
			)
			.map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err))
			.spanning(&tokens)?;

		assert!(spec.operations.is_empty());

		let out = String::from_utf8(run_state.writer).map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err)).spanning(&tokens)?;
		let result = out.parse().map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {:?}", err)).spanning(&tokens)?;
		Ok(result)
	}
}

struct MapNamespace;

impl k8s_openapi_codegen_common::MapNamespace for MapNamespace {
	fn map_namespace<'a>(&self, path_parts: &[&'a str]) -> Option<Vec<&'a str>> {
		match path_parts {
			["io", "k8s", rest @ ..] => Some(std::iter::once("k8s_openapi").chain(rest.iter().copied()).collect()),
			path_parts => Some(path_parts.iter().copied().collect()),
		}
	}
}

struct RunState {
	writer: Vec<u8>,
}

impl k8s_openapi_codegen_common::RunState for RunState {
	type Writer = Vec<u8>;

	fn make_writer(
		&mut self,
		_parts: &[&str],
		_type_feature: Option<&str>,
	) -> std::io::Result<Self::Writer> {
		Ok(std::mem::take(&mut self.writer))
	}

	fn handle_operation_types(
		&mut self,
		_operation_optional_parameters_name: Option<&str>,
		_operation_result_name: Option<&str>,
	) -> std::io::Result<()> {
		Ok(())
	}

	fn finish(&mut self, writer: Self::Writer) {
		self.writer = writer;
	}
}
