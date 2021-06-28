// Generated from definition io.k8s.api.authorization.v1beta1.ResourceAttributes

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceAttributes {
    /// Group is the API Group of the Resource.  "*" means all.
    pub group: Option<String>,

    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    pub name: Option<String>,

    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    pub namespace: Option<String>,

    /// Resource is one of the existing resource types.  "*" means all.
    pub resource: Option<String>,

    /// Subresource is one of the existing resource types.  "" means none.
    pub subresource: Option<String>,

    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verb: Option<String>,

    /// Version is the API Version of the Resource.  "*" means all.
    pub version: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "group" => Field::Key_group,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_resource: Option<String> = None;
                let mut value_subresource: Option<String> = None;
                let mut value_verb: Option<String> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    group: value_group,
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
                "group",
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
            self.group.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.resource.as_ref().map_or(0, |_| 1) +
            self.subresource.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for ResourceAttributes {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface",
          "properties": {
            "group": {
              "description": "Group is the API Group of the Resource.  \"*\" means all.",
              "type": "string"
            },
            "name": {
              "description": "Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all.",
              "type": "string"
            },
            "namespace": {
              "description": "Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview",
              "type": "string"
            },
            "resource": {
              "description": "Resource is one of the existing resource types.  \"*\" means all.",
              "type": "string"
            },
            "subresource": {
              "description": "Subresource is one of the existing resource types.  \"\" means none.",
              "type": "string"
            },
            "verb": {
              "description": "Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.",
              "type": "string"
            },
            "version": {
              "description": "Version is the API Version of the Resource.  \"*\" means all.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
