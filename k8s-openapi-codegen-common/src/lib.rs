#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::too_many_arguments,
    clippy::too_many_lines,
)]

//! This crate contains common code for the [`k8s-openapi` code generator](https://github.com/Arnavion/k8s-openapi/tree/master/k8s-openapi-codegen)
//! and the [`k8s-openapi-derive`](https://crates.io/crates/k8s-openapi-derive) custom derive crate.
//!
//! It can be used by code generators that want to generate crates like `k8s-openapi` and `k8s-openapi-derive` for Kubernetes-like software
//! such as OpenShift.
//!
//! 1. Create a [`swagger20::Spec`] value, either by deserializing it from an OpenAPI spec JSON file or by creating it manually.
//! 1. Invoke the [`run`] function for each definition in the spec.

pub mod swagger20;

mod templates;

/// Statistics from a successful invocation of [`run`]
#[derive(Clone, Copy, Debug)]
pub struct RunResult {
    pub num_generated_structs: usize,
    pub num_generated_type_aliases: usize,
}

/// Error type reported by [`run`]
#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

macro_rules! impl_from_for_error {
    ($($ty:ty ,)*) => {
        $(
            impl From<$ty> for Error {
                fn from(err: $ty) -> Self {
                    Error(err.into())
                }
            }
        )*
    };
}

impl_from_for_error! {
    &'_ str,
    String,
    std::fmt::Error,
    std::io::Error,
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
    fn make_writer(&mut self, parts: &[&str]) -> std::io::Result<Self::Writer>;

    /// This function is invoked when `k8s_openapi_codegen_common::run` is done with the writer and completes successfully.
    /// The implementation can do any cleanup that it wants here.
    fn finish(&mut self, writer: Self::Writer);
}

impl<T> RunState for &'_ mut T where T: RunState {
    type Writer = <T as RunState>::Writer;

    fn make_writer(&mut self, parts: &[&str]) -> std::io::Result<Self::Writer> {
        (*self).make_writer(parts)
    }

    fn finish(&mut self, writer: Self::Writer) {
        (*self).finish(writer);
    }
}

/// Whether [`run`] should generate an impl of `schemars::JsonSchema` for the type or not.
#[derive(Clone, Copy, Debug)]
pub enum GenerateSchema<'a> {
    Yes {
        /// An optional feature that the impl of `schemars::JsonSchema` will be `cfg`-gated by.
        feature: Option<&'a str>,
    },

    No,
}

/// Each invocation of this function generates a single type specified by the `definition_path` parameter.
///
/// # Parameters
///
/// - `definitions`: The definitions parsed from the OpenAPI spec that should be emitted as model types.
///
/// - `operations`: The list of operations parsed from the OpenAPI spec. The list is mutated to remove the operations
///   that are determined to be associated with the type currently being generated.
///
/// - `definition_path`: The specific definition path out of the `definitions` collection that should be emitted.
///
/// - `map_namespace`: An instance of the [`MapNamespace`] trait that controls how OpenAPI namespaces of the definitions are mapped to rust namespaces.
///
/// - `vis`: The visibility modifier that should be emitted on the generated code.
///
/// - `state`: See the documentation of the [`RunState`] trait.
pub fn run(
    definitions: &std::collections::BTreeMap<swagger20::DefinitionPath, swagger20::Schema>,
    operations: &mut Vec<swagger20::Operation>,
    definition_path: &swagger20::DefinitionPath,
    map_namespace: &impl MapNamespace,
    vis: &str,
    generate_schema: GenerateSchema<'_>,
    mut state: impl RunState,
) -> Result<RunResult, Error> {
    let definition = definitions.get(definition_path).ok_or_else(|| format!("definition for {definition_path} does not exist in spec"))?;

    let local = map_namespace_local_to_string(map_namespace)?;

    let mut run_result = RunResult {
        num_generated_structs: 0,
        num_generated_type_aliases: 0,
    };

    let path_parts: Vec<_> = definition_path.split('.').collect();
    let namespace_parts: Vec<_> =
        map_namespace.map_namespace(&path_parts).ok_or_else(|| format!("unexpected path {definition_path:?}"))?
        .into_iter()
        .collect();

    let mut out = state.make_writer(&namespace_parts)?;

    let type_name = path_parts.last().ok_or_else(|| format!("path for {definition_path} has no parts"))?;

    let derives = get_derives(&definition.kind, definitions, map_namespace)?;

    templates::type_header::generate(
        &mut out,
        definition_path,
        definition.description.as_deref(),
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
                                field_type_name.push_str("std::boxed::Box<");
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
                        merge_type: &schema.merge_type,
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
                                definition.list_kind.as_ref().map(|kind| format!("{kind:?}")),
                            )
                        }
                        else {
                            (
                                format!("{:?}", format!("{}/{}", single_group_version_kind.group, single_group_version_kind.version)),
                                format!("{:?}", single_group_version_kind.group),
                                format!("{:?}", single_group_version_kind.kind),
                                format!("{:?}", single_group_version_kind.version),
                                definition.list_kind.as_ref().map(|kind| format!("{kind:?}")),
                            )
                        }),
                    Some((_, true, false)) => return Err(format!("{definition_path} has an apiVersion property but not a kind property").into()),
                    Some((_, false, true)) => return Err(format!("{definition_path} has a kind property but not an apiVersion property").into()),
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

                        for operation in operations {
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

                            let (url_path_segment_, scope_, url_path_segment_and_scope) = match components {
                                ("{name}", Some(url_path_segment), Some("{namespace}"), Some("namespaces")) =>
                                    (
                                        format!("{url_path_segment:?}"),
                                        format!("{local}NamespaceResourceScope"),
                                        &mut namespace_or_cluster_scoped_url_path_segment_and_scope,
                                    ),

                                ("{name}", Some(url_path_segment), _, _) =>
                                    (
                                        format!("{url_path_segment:?}"),
                                        format!("{local}ClusterResourceScope"),
                                        &mut namespace_or_cluster_scoped_url_path_segment_and_scope,
                                    ),

                                (url_path_segment, Some("{name}"), _, _) =>
                                    (
                                        format!("{url_path_segment:?}"),
                                        format!("{local}SubResourceScope"),
                                        &mut subresource_url_path_segment_and_scope,
                                    ),

                                (url_path_segment, Some("{namespace}"), Some("namespaces"), _) =>
                                    (
                                        format!("{url_path_segment:?}"),
                                        format!("{local}NamespaceResourceScope"),
                                        &mut namespace_or_cluster_scoped_url_path_segment_and_scope,
                                    ),

                                (url_path_segment, _, _, _) =>
                                    (
                                        format!("{url_path_segment:?}"),
                                        format!("{local}ClusterResourceScope"),
                                        &mut namespace_or_cluster_scoped_url_path_segment_and_scope,
                                    ),
                            };

                            url_path_segment_and_scope.push((url_path_segment_, scope_));
                        }
                    }
                }

                *operations = operations_by_gkv.into_values().flatten().collect();
            }

            match &**definition_path {
                "io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup" |
                "io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupList" |
                "io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList" |
                "io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions" =>
                    namespace_or_cluster_scoped_url_path_segment_and_scope.push((r#""""#.to_owned(), format!("{local}ClusterResourceScope"))),
                "io.k8s.apimachinery.pkg.apis.meta.v1.Status" =>
                    subresource_url_path_segment_and_scope.push((r#""status""#.to_owned(), format!("{local}SubResourceScope"))),
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
                            "definition {definition_path} is a Resource but its URL path segment and scope could not be inferred").into()),
                        ([_, ..], _) => return Err(format!(
                            "definition {definition_path} is a Resource but was inferred to have multiple scopes {namespace_or_cluster_scoped_url_path_segment_and_scope:?}").into()),
                        ([], [_, ..]) => return Err(format!(
                            "definition {definition_path} is a Resource but was inferred to have multiple scopes {subresource_url_path_segment_and_scope:?}").into()),
                    },
                }),

                (Some(_), Some((_, templates::PropertyRequired::Optional | templates::PropertyRequired::OptionalDefault))) =>
                    return Err(format!("definition {definition_path} has optional metadata").into()),

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
                            "definition {definition_path} is a Resource but its URL path segment and scope could not be inferred").into()),
                        ([_, _, ..], _) => return Err(format!(
                            "definition {definition_path} is a Resource but was inferred to have multiple scopes {namespace_or_cluster_scoped_url_path_segment_and_scope:?}").into()),
                        ([], [_, _, ..]) => return Err(format!(
                            "definition {definition_path} is a Resource but was inferred to have multiple scopes {subresource_url_path_segment_and_scope:?}").into()),
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

            if definition.impl_deep_merge {
                templates::struct_deep_merge::generate(
                    &mut out,
                    type_name,
                    Default::default(),
                    &template_properties,
                    map_namespace,
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

        swagger20::SchemaKind::Ref(_) => return Err(format!("{definition_path} is a Ref").into()),

        swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => {
            templates::int_or_string::generate(
                &mut out,
                type_name,
                map_namespace,
            )?;

            run_result.num_generated_structs += 1;
        },

        swagger20::SchemaKind::Ty(swagger20::Type::JsonSchemaPropsOr(namespace, json_schema_props_or)) => {
            let json_schema_props_or = match json_schema_props_or {
                swagger20::JsonSchemaPropsOr::Array => templates::json_schema_props_or::Or::Array,
                swagger20::JsonSchemaPropsOr::Bool => templates::json_schema_props_or::Or::Bool,
                swagger20::JsonSchemaPropsOr::StringArray => templates::json_schema_props_or::Or::StringArray,
            };

            let json_schema_props_type_name =
                get_fully_qualified_type_name(
                    &swagger20::RefPath {
                        path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{namespace}.JSONSchemaProps"),
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
                &error_status_rust_type,
                &error_other_rust_type,
                map_namespace,
            )?;

            run_result.num_generated_structs += 1;
        },

        swagger20::SchemaKind::Ty(swagger20::Type::ListDef { metadata }) => {
            let metadata_rust_type = get_rust_type(metadata, map_namespace)?;

            let template_generics_where_part = format!("T: {local}ListableResource");
            let template_generics = templates::Generics {
                type_part: Some("T"),
                where_part: Some(&template_generics_where_part),
            };

            let items_merge_type = swagger20::MergeType::List {
                strategy: swagger20::KubernetesListType::Map,
                keys: vec!["metadata().namespace".to_string(), "metadata().name".to_string()],
                item_merge_type: Box::new(swagger20::MergeType::Default),
            };

            let template_properties = vec![
                templates::Property {
                    name: "items",
                    comment: Some("List of objects."),
                    field_name: "items".into(),
                    field_type_name: "std::vec::Vec<T>".to_owned(),
                    required: templates::PropertyRequired::Required { is_default: true },
                    is_flattened: false,
                    merge_type: &items_merge_type,
                },

                templates::Property {
                    name: "metadata",
                    comment: Some("Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"),
                    field_name: "metadata".into(),
                    field_type_name: (*metadata_rust_type).to_owned(),
                    required: templates::PropertyRequired::Required { is_default: true },
                    is_flattened: false,
                    merge_type: &swagger20::MergeType::Default,
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

            if definition.impl_deep_merge {
                let template_generics_where_part = format!("T: {local}DeepMerge + {local}Metadata<Ty = {local}apimachinery::pkg::apis::meta::v1::ObjectMeta> + {local}ListableResource");
                let template_generics = templates::Generics {
                    where_part: Some(&template_generics_where_part),
                    ..template_generics
                };

                templates::struct_deep_merge::generate(
                    &mut out,
                    type_name,
                    template_generics,
                    &template_properties,
                    map_namespace,
                )?;
            }

            {
                let template_generics_where_part = format!("T: {local}serde::Deserialize<'de> + {local}ListableResource");
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
                let template_generics_where_part = format!("T: {local}serde::Serialize + {local}ListableResource");
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

        swagger20::SchemaKind::Ty(swagger20::Type::ListRef { .. }) => return Err(format!("definition {definition_path} is a ListRef").into()),

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
                swagger20::Type::JsonSchemaPropsOr(_, _) |
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
        result.push_str(&get_rust_ident(namespace_part));
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
            |_, _| Ok(true))?;

    let derive_copy =
        derive_clone &&
        evaluate_trait_bound(
            kind,
            false,
            definitions,
            map_namespace,
            |_, _| Ok(false))?;

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
            swagger20::Type::JsonSchemaPropsOr(_, _) |
            swagger20::Type::Patch |
            swagger20::Type::WatchEvent(_)
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
            |_, _| Ok(true))?;

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
            swagger20::Type::JsonSchemaPropsOr(_, _) |
            swagger20::Type::Patch |
            swagger20::Type::WatchEvent(_)
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
        #[allow(clippy::ptr_arg)] // False positive. Clippy wants this to be `&SchemaKind` but we use Cow-specific operations (`.clone()`).
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
                let trait_bound =
                    if let Some(target) = definitions.get(&*ref_path.path) {
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

            swagger20::SchemaKind::Ty(swagger20::Type::JsonSchemaPropsOr(namespace, _)) => {
                let json_schema_props_ref_path = swagger20::RefPath {
                    path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{namespace}.JSONSchemaProps"),
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
                .replace('\t', "    ")
                .replace("```", "");

            let line =
                if *previous_line_was_empty && line.starts_with("    ") {
                    // Collapse this line's spaces into two. Otherwise rustdoc will think this is the start of a code block containing a Rust test.
                    format!("  {}", line.trim_start())
                }
                else {
                    line
                };
            let line = line.trim_end();

            *previous_line_was_empty = false;

            Some(format!("{indent} {line}").into())
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
            result.push_str(&get_rust_ident(namespace_part));
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
        "hostIPs" => return "host_ips".into(),
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

fn get_rust_type(
    schema_kind: &swagger20::SchemaKind,
    map_namespace: &impl MapNamespace,
) -> Result<std::borrow::Cow<'static, str>, Error> {
    let local = map_namespace_local_to_string(map_namespace)?;

    match schema_kind {
        swagger20::SchemaKind::Properties(_) => Err("Nested anonymous types not supported".into()),

        swagger20::SchemaKind::Ref(ref_path) =>
            Ok(get_fully_qualified_type_name(ref_path, map_namespace).into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Any) => Ok(format!("{local}serde_json::Value").into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Array { items }) =>
            Ok(format!("std::vec::Vec<{}>", get_rust_type(&items.kind, map_namespace)?).into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Boolean) => Ok("bool".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int32 }) => Ok("i32".into()),
        swagger20::SchemaKind::Ty(swagger20::Type::Integer { format: swagger20::IntegerFormat::Int64 }) => Ok("i64".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Number { format: swagger20::NumberFormat::Double }) => Ok("f64".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::Object { additional_properties }) =>
            Ok(format!("std::collections::BTreeMap<std::string::String, {}>", get_rust_type(&additional_properties.kind, map_namespace)?).into()),

        swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::Byte) }) =>
            Ok(format!("{local}ByteString").into()),
        swagger20::SchemaKind::Ty(swagger20::Type::String { format: Some(swagger20::StringFormat::DateTime) }) =>
            Ok(format!("{local}chrono::DateTime<{local}chrono::Utc>").into()),
        swagger20::SchemaKind::Ty(swagger20::Type::String { format: None }) => Ok("std::string::String".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::CustomResourceSubresources(namespace)) => {
            let namespace_parts =
                &["io", "k8s", "apiextensions_apiserver", "pkg", "apis", "apiextensions", namespace];
            let namespace_parts =
                map_namespace.map_namespace(namespace_parts)
                .ok_or_else(|| format!("unexpected path {:?}", namespace_parts.join(".")))?;

            let mut result = String::new();
            for namespace_part in namespace_parts {
                result.push_str(&get_rust_ident(namespace_part));
                result.push_str("::");
            }
            result.push_str("CustomResourceSubresources");
            Ok(result.into())
        },

        swagger20::SchemaKind::Ty(swagger20::Type::IntOrString) => Err("nothing should be trying to refer to IntOrString".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::JsonSchemaPropsOr(_, _)) => Err("JSON schema types not supported".into()),
        swagger20::SchemaKind::Ty(swagger20::Type::Patch) => Err("Patch type not supported".into()),
        swagger20::SchemaKind::Ty(swagger20::Type::WatchEvent(_)) => Err("WatchEvent type not supported".into()),

        swagger20::SchemaKind::Ty(swagger20::Type::ListDef { .. }) => Err("ListDef type not supported".into()),
        swagger20::SchemaKind::Ty(swagger20::Type::ListRef { items }) =>
            Ok(format!("{local}List<{}>", get_rust_type(items, map_namespace)?).into()),
    }
}
