use k8s_openapi_codegen_common::swagger20;

use super::ResultExt;

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
    impl_deep_merge: bool,
}

impl super::CustomDerive for CustomResourceDefinition {
    fn parse(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
        let ident = input.ident;
        let vis = input.vis;

        let mut group = None;
        let mut version = None;
        let mut plural = None;
        let mut generate_schema = false;
        let mut namespaced = false;
        let mut has_subresources = None;
        let mut impl_deep_merge = false;

        for attr in &input.attrs {
            let syn::AttrStyle::Outer = attr.style else { continue; };

            if !attr.path().is_ident("custom_resource_definition") {
                continue;
            }

            let metas = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?;
            for meta in metas {
                let meta: &dyn quote::ToTokens = match &meta {
                    syn::Meta::NameValue(meta) =>
                        if meta.path.is_ident("group") {
                            let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value else {
                                return Err(r#"#[custom_resource_definition(group = "...")] expects a string literal value"#).spanning(meta);
                            };
                            group = Some(lit.value());
                            continue;
                        }
                        else if meta.path.is_ident("version") {
                            let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value else {
                                return Err(r#"#[custom_resource_definition(version = "...")] expects a string literal value"#).spanning(meta);
                            };
                            version = Some(lit.value());
                            continue;
                        }
                        else if meta.path.is_ident("plural") {
                            let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value else {
                                return Err(r#"#[custom_resource_definition(plural = "...")] expects a string literal value"#).spanning(meta);
                            };
                            plural = Some(lit.value());
                            continue;
                        }
                        else if meta.path.is_ident("has_subresources") {
                            let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. }) = &meta.value else {
                                return Err(r#"#[custom_resource_definition(has_subresources = "...")] expects a string literal value"#).spanning(meta);
                            };
                            has_subresources = Some(lit.value());
                            continue;
                        }
                        else {
                            meta
                        },

                    syn::Meta::Path(path) =>
                        if path.is_ident("generate_schema") {
                            generate_schema = true;
                            continue;
                        }
                        else if path.is_ident("namespaced") {
                            namespaced = true;
                            continue;
                        }
                        else if path.is_ident("impl_deep_merge") {
                            impl_deep_merge = true;
                            continue;
                        }
                        else {
                            &meta
                        },

                    meta @ syn::Meta::List(_) => meta,
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
            impl_deep_merge,
        })
    }

    fn emit(self) -> Result<proc_macro2::TokenStream, syn::Error> {
        let CustomResourceDefinition { ident: cr_spec_name, vis, tokens, group, version, plural, generate_schema, namespaced, has_subresources, impl_deep_merge } = self;

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

        let (namespace_operation_id_component, namespace_path_component) =
            if namespaced {
                ("Namespaced", "/namespaces/{namespace}")
            }
            else {
                ("", "")
            };

        let mut spec = swagger20::Spec {
            info: swagger20::Info {
                title: String::new(),
                version: String::new(),
            },
            definitions: [
                (swagger20::DefinitionPath(cr_name.clone()), swagger20::Schema {
                    description: Some(format!("Custom resource for `{cr_spec_name}`")),
                    kind: swagger20::SchemaKind::Properties([
                        (swagger20::PropertyName("apiVersion".to_owned()), (swagger20::Schema {
                            description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources>".to_owned()),
                            kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
                            kubernetes_group_kind_versions: vec![],
                            list_kind: None,
                            merge_type: swagger20::MergeType::Default,
                            impl_deep_merge: true,
                        }, false)),
                        (swagger20::PropertyName("kind".to_owned()), (swagger20::Schema {
                            description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>".to_owned()),
                            kind: swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }),
                            kubernetes_group_kind_versions: vec![],
                            list_kind: None,
                            merge_type: swagger20::MergeType::Default,
                            impl_deep_merge: true,
                        }, false)),
                        (swagger20::PropertyName("metadata".to_owned()), (swagger20::Schema {
                            description: Some("Standard object's metadata. More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>".to_owned()),
                            kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
                                path: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_owned(),
                                can_be_default: None,
                            }),
                            kubernetes_group_kind_versions: vec![],
                            list_kind: None,
                            merge_type: swagger20::MergeType::Default,
                            impl_deep_merge: true,
                        }, true)),
                        (swagger20::PropertyName("spec".to_owned()), (swagger20::Schema {
                            description: Some(format!("Specification of the `{cr_name}` custom resource")),
                            kind: swagger20::SchemaKind::Ref(swagger20::RefPath {
                                path: cr_spec_name,
                                can_be_default: None,
                            }),
                            kubernetes_group_kind_versions: vec![],
                            list_kind: None,
                            merge_type: swagger20::MergeType::Default,
                            impl_deep_merge: true,
                        }, false)),
                    ].into_iter().chain(
                        has_subresources.map(|has_subresources|
                            (swagger20::PropertyName("subresources".to_owned()), (swagger20::Schema {
                                description: Some(format!("Subresources of the `{cr_name}` custom resource")),
                                kind: swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(has_subresources)),
                                kubernetes_group_kind_versions: vec![],
                                list_kind: None,
                                merge_type: swagger20::MergeType::Default,
                                impl_deep_merge: true,
                            }, true)))
                    ).collect()),
                    kubernetes_group_kind_versions: vec![
                        swagger20::KubernetesGroupKindVersion {
                            group: group.clone(),
                            kind: cr_name.clone(),
                            version: version.clone(),
                        },
                    ],
                    list_kind: Some(format!("{cr_name}List")),
                    merge_type: swagger20::MergeType::Default,
                    impl_deep_merge,
                }),
            ].into(),
            operations: vec![
                swagger20::Operation {
                    id: format!("create{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Post),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}")),
                },

                swagger20::Operation {
                    id: format!("delete{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Delete),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}")),
                },

                swagger20::Operation {
                    id: format!("deleteCollection{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::DeleteCollection),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}")),
                },

                swagger20::Operation {
                    id: format!("list{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::List),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}")),
                },

                swagger20::Operation {
                    id: format!("patch{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Patch),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}")),
                },

                swagger20::Operation {
                    id: format!("patch{namespace_operation_id_component}{cr_name}Status"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Patch),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}/status")),
                },

                swagger20::Operation {
                    id: format!("read{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Get),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}")),
                },

                swagger20::Operation {
                    id: format!("read{namespace_operation_id_component}{cr_name}Status"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Get),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}/status")),
                },

                swagger20::Operation {
                    id: format!("replace{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Put),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}")),
                },

                swagger20::Operation {
                    id: format!("replace{namespace_operation_id_component}{cr_name}Status"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Put),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}/{{name}}/status")),
                },

                swagger20::Operation {
                    id: format!("watch{namespace_operation_id_component}{cr_name}"),
                    kubernetes_action: Some(swagger20::KubernetesAction::Watch),
                    kubernetes_group_kind_version: Some(swagger20::KubernetesGroupKindVersion {
                        group: group.clone(),
                        kind: cr_name.clone(),
                        version: version.clone(),
                    }),
                    path: swagger20::Path(format!("/apis/{group}/{version}{namespace_path_component}/{plural}")),
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
                &mut run_state,
            )
            .map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {err}"))
            .spanning(&tokens)?;

        assert!(spec.operations.is_empty());

        let out = String::from_utf8(run_state.writer).map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {err}")).spanning(&tokens)?;
        let result = out.parse().map_err(|err| format!("#[derive(CustomResourceDefinition)] failed: {err:?}")).spanning(&tokens)?;
        Ok(result)
    }
}

struct MapNamespace;

impl k8s_openapi_codegen_common::MapNamespace for MapNamespace {
    fn map_namespace<'a>(&self, path_parts: &[&'a str]) -> Option<Vec<&'a str>> {
        match path_parts {
            ["io", "k8s", rest @ ..] => Some(std::iter::once("k8s_openapi").chain(rest.iter().copied()).collect()),
            path_parts => Some(path_parts.to_owned()),
        }
    }
}

struct RunState {
    writer: Vec<u8>,
}

impl k8s_openapi_codegen_common::RunState for RunState {
    type Writer = Vec<u8>;

    fn make_writer(&mut self, _parts: &[&str]) -> std::io::Result<Self::Writer> {
        Ok(std::mem::take(&mut self.writer))
    }

    fn finish(&mut self, writer: Self::Writer) {
        self.writer = writer;
    }
}
