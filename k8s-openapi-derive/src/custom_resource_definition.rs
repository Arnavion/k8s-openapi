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
	namespaced: bool,
}

impl super::CustomDerive for CustomResourceDefinition {
	fn parse(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
		let ident = input.ident;
		let vis = input.vis;

		let mut group = None;
		let mut plural = None;
		let mut version = None;

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
						if meta.ident == "group" {
							if let syn::Lit::Str(lit) = &meta.lit {
								group = Some(lit.value());
								continue;
							}
							else {
								return Err(r#"#[custom_resource_definition(group = "...")] expects a string literal value"#).spanning(meta);
							}
						}
						else if meta.ident == "plural" {
							if let syn::Lit::Str(lit) = &meta.lit {
								plural = Some(lit.value());
								continue;
							}
							else {
								return Err(r#"#[custom_resource_definition(plural = "...")] expects a string literal value"#).spanning(meta);
							}
						}
						else if meta.ident == "version" {
							if let syn::Lit::Str(lit) = &meta.lit {
								version = Some(lit.value());
								continue;
							}
							else {
								return Err(r#"#[custom_resource_definition(version = "...")] expects a string literal value"#).spanning(meta);
							}
						}
						else {
							meta
						},

					syn::NestedMeta::Meta(syn::Meta::Word(ident)) =>
						if ident == "namespaced" {
							namespaced = true;
							continue;
						}
						else {
							&meta
						},

					meta => meta,
				};

				return
					Err(r#"#[derive(CustomResourceDefinition)] found unexpected meta. Expected `group = "..."`, `namespaced`, `plural = "..."` or `version = "..."`"#)
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
			namespaced,
			plural,
		})
	}

	fn emit(self) -> Result<proc_macro2::TokenStream, syn::Error> {
		let CustomResourceDefinition { ident: cr_spec_name, vis, tokens, group, version, plural, namespaced } = self;

		let vis: std::borrow::Cow<'_, str> = match vis {
			syn::Visibility::Inherited => "".into(),
			vis => format!("{} ", quote::ToTokens::into_token_stream(vis.clone())).into(),
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
						relative_to: swagger20::RefPathRelativeTo::Scope,
						can_be_default: None,
					}),
					kubernetes_group_kind_versions: None,
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
					kubernetes_group_kind_versions: None,
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
						kubernetes_group_kind_versions: None,
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
							description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: None,
						}, false)),
						(swagger20::PropertyName("kind".to_owned()), (swagger20::Schema {
							description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: None,
						}, false)),
						(swagger20::PropertyName("metadata".to_owned()), (swagger20::Schema {
							description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}, false)),
						(swagger20::PropertyName("spec".to_owned()), (swagger20::Schema {
							description: Some(format!("Specification of the {} custom resource", cr_name)),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_spec_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}, false)),
					].into_iter().collect()),
					kubernetes_group_kind_versions: Some(vec![
						swagger20::KubernetesGroupKindVersion {
							group: group.clone(),
							kind: cr_name.clone(),
							version: version.clone(),
						},
					]),
				}),

				(swagger20::DefinitionPath(cr_list_name.clone()), swagger20::Schema {
					description: Some(format!("{} is a list of {}", cr_list_name, cr_name)),
					kind: swagger20::SchemaKind::Properties(vec![
						(swagger20::PropertyName("apiVersion".to_owned()), (swagger20::Schema {
							description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: None,
						}, false)),
						(swagger20::PropertyName("items".to_owned()), (swagger20::Schema {
							description: Some(format!("List of objects of kind {}. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md", cr_name)),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::Array {
								items: Box::new(swagger20::Schema {
									description: None,
									kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
										path: cr_name.clone(),
										relative_to: swagger20::RefPathRelativeTo::Scope,
										can_be_default: Some(true),
									}),
									kubernetes_group_kind_versions: None,
								}),
							}),
							kubernetes_group_kind_versions: None,
						}, true)),
						(swagger20::PropertyName("kind".to_owned()), (swagger20::Schema {
							description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds".to_owned()),
							kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
							kubernetes_group_kind_versions: None,
						}, false)),
						(swagger20::PropertyName("metadata".to_owned()), (swagger20::Schema {
							description: Some("Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}, false)),
					].into_iter().collect()),
					kubernetes_group_kind_versions: Some(vec![
						swagger20::KubernetesGroupKindVersion {
							group: group.clone(),
							kind: cr_list_name.clone(),
							version: version.clone(),
						},
					]),
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
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::CREATED, swagger20::Schema {
							description: Some("Created".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::ACCEPTED, swagger20::Schema {
							description: Some("Accepted".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::UNPROCESSABLE_ENTITY, swagger20::Schema {
							description: Some("Unprocessable Entity".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::ACCEPTED, swagger20::Schema {
							description: Some("Accepted".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
				},

				// TODO: deleteCollection

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
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.ListOptional".to_owned(),
									relative_to: swagger20::RefPathRelativeTo::Crate,
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_list_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
									relative_to: swagger20::RefPathRelativeTo::Crate,
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "dryRun".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "fieldManager".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "force".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some(r#"Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests."#.to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::Boolean),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "pretty".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("If 'true', then the output is pretty printed.".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
									relative_to: swagger20::RefPathRelativeTo::Crate,
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "dryRun".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "fieldManager".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "force".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some(r#"Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests."#.to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::Boolean),
								kubernetes_group_kind_versions: None,
							},
						})),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "pretty".to_owned(),
							required: false,
							schema: swagger20::Schema {
								description: Some("If 'true', then the output is pretty printed.".to_owned()),
								kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
								kubernetes_group_kind_versions: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}/status", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::ACCEPTED, swagger20::Schema {
							description: Some("Accepted".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
						Some(body_parameter.clone()),
						Some(name_parameter.clone()),
						namespace_parameter.clone(),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}/{{name}}/status", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
						(http::StatusCode::ACCEPTED, swagger20::Schema {
							description: Some("Accepted".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: cr_name.clone(),
								relative_to: swagger20::RefPathRelativeTo::Scope,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
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
						namespace_parameter.clone(),
						Some(std::sync::Arc::new(swagger20::Parameter {
							location: swagger20::ParameterLocation::Query,
							name: "optional".to_owned(),
							required: true,
							schema: swagger20::Schema {
								description: None,
								kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
									path: "io.k8s.WatchOptional".to_owned(),
									relative_to: swagger20::RefPathRelativeTo::Crate,
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: None,
							},
						})),
					].into_iter().flatten().collect(),
					path: swagger20::Path(format!("/apis/{}/{}{}/{}", group, version, namespace_path_component, plural)),
					responses: vec![
						(http::StatusCode::OK, swagger20::Schema {
							description: Some("OK".to_owned()),
							kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
								path: "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent".to_owned(),
								relative_to: swagger20::RefPathRelativeTo::Crate,
								can_be_default: None,
							}),
							kubernetes_group_kind_versions: None,
						}),
					].into_iter().collect(),
					tag: String::new(),
				},
			],
		};

		let mut out = vec![];

		let _ =
			k8s_openapi_codegen_common::run(
				&spec.definitions,
				&mut spec.operations,
				&swagger20::DefinitionPath(cr_name.clone()),
				&[
					(&["io".into(), "k8s".into()], &[]),
				],
				"k8s_openapi",
				None,
				&vis,
				|_| Ok(&mut out),
				|_, _| Ok(()),
			)
			.map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err))
			.spanning(&tokens)?;


		let _ =
			k8s_openapi_codegen_common::run(
				&spec.definitions,
				&mut spec.operations,
				&swagger20::DefinitionPath(cr_list_name.clone()),
				&[
					(&["io".into(), "k8s".into()], &[]),
				],
				"k8s_openapi",
				None,
				&vis,
				|_| Ok(&mut out),
				|_, _| Ok(()),
			)
			.map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err))
			.spanning(&tokens)?;

		assert!(spec.operations.is_empty());

		let out = String::from_utf8(out).map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {}", err)).spanning(&tokens)?;
		let result = out.parse().map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {:?}", err)).spanning(&tokens)?;
		Ok(result)
	}
}
