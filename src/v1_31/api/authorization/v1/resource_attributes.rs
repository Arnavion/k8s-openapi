// Generated from definition io.k8s.api.authorization.v1.ResourceAttributes

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceAttributes {
    /// fieldSelector describes the limitation on access based on field.  It can only limit access, not broaden it.
    ///
    /// This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
    pub field_selector: Option<crate::api::authorization::v1::FieldSelectorAttributes>,

    /// Group is the API Group of the Resource.  "*" means all.
    pub group: Option<std::string::String>,

    /// labelSelector describes the limitation on access based on labels.  It can only limit access, not broaden it.
    ///
    /// This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
    pub label_selector: Option<crate::api::authorization::v1::LabelSelectorAttributes>,

    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    pub name: Option<std::string::String>,

    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    pub namespace: Option<std::string::String>,

    /// Resource is one of the existing resource types.  "*" means all.
    pub resource: Option<std::string::String>,

    /// Subresource is one of the existing resource types.  "" means none.
    pub subresource: Option<std::string::String>,

    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verb: Option<std::string::String>,

    /// Version is the API Version of the Resource.  "*" means all.
    pub version: Option<std::string::String>,
}

impl crate::DeepMerge for ResourceAttributes {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.field_selector, other.field_selector);
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.label_selector, other.label_selector);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
        crate::DeepMerge::merge_from(&mut self.subresource, other.subresource);
        crate::DeepMerge::merge_from(&mut self.verb, other.verb);
        crate::DeepMerge::merge_from(&mut self.version, other.version);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field_selector,
            Key_group,
            Key_label_selector,
            Key_name,
            Key_namespace,
            Key_resource,
            Key_subresource,
            Key_verb,
            Key_version,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "fieldSelector" => Field::Key_field_selector,
                            "group" => Field::Key_group,
                            "labelSelector" => Field::Key_label_selector,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "resource" => Field::Key_resource,
                            "subresource" => Field::Key_subresource,
                            "verb" => Field::Key_verb,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceAttributes;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_field_selector: Option<crate::api::authorization::v1::FieldSelectorAttributes> = None;
                let mut value_group: Option<std::string::String> = None;
                let mut value_label_selector: Option<crate::api::authorization::v1::LabelSelectorAttributes> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespace: Option<std::string::String> = None;
                let mut value_resource: Option<std::string::String> = None;
                let mut value_subresource: Option<std::string::String> = None;
                let mut value_verb: Option<std::string::String> = None;
                let mut value_version: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field_selector => value_field_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_label_selector => value_label_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subresource => value_subresource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verb => value_verb = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceAttributes {
                    field_selector: value_field_selector,
                    group: value_group,
                    label_selector: value_label_selector,
                    name: value_name,
                    namespace: value_namespace,
                    resource: value_resource,
                    subresource: value_subresource,
                    verb: value_verb,
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceAttributes",
            &[
                "fieldSelector",
                "group",
                "labelSelector",
                "name",
                "namespace",
                "resource",
                "subresource",
                "verb",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceAttributes",
            self.field_selector.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.label_selector.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.resource.as_ref().map_or(0, |_| 1) +
            self.subresource.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.field_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldSelector", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.label_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        if let Some(value) = &self.subresource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresource", value)?;
        }
        if let Some(value) = &self.verb {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verb", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceAttributes {
    fn schema_name() -> std::string::String {
        "io.k8s.api.authorization.v1.ResourceAttributes".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fieldSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::FieldSelectorAttributes>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fieldSelector describes the limitation on access based on field.  It can only limit access, not broaden it.\n\nThis field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "group".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Group is the API Group of the Resource.  \"*\" means all.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "labelSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::LabelSelectorAttributes>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("labelSelector describes the limitation on access based on labels.  It can only limit access, not broaden it.\n\nThis field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resource".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Resource is one of the existing resource types.  \"*\" means all.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subresource".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Subresource is one of the existing resource types.  \"\" means none.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "verb".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "version".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Version is the API Version of the Resource.  \"*\" means all.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
