// Generated from definition io.k8s.api.resource.v1beta1.ResourceClaimConsumerReference

/// ResourceClaimConsumerReference contains enough information to let you locate the consumer of a ResourceClaim. The user must be a resource in the same namespace as the ResourceClaim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimConsumerReference {
    /// APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.
    pub api_group: Option<std::string::String>,

    /// Name is the name of resource being referenced.
    pub name: std::string::String,

    /// Resource is the type of resource being referenced, for example "pods".
    pub resource: std::string::String,

    /// UID identifies exactly one incarnation of the resource.
    pub uid: std::string::String,
}

impl crate::DeepMerge for ResourceClaimConsumerReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_group, other.api_group);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimConsumerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_group,
            Key_name,
            Key_resource,
            Key_uid,
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
                            "apiGroup" => Field::Key_api_group,
                            "name" => Field::Key_name,
                            "resource" => Field::Key_resource,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimConsumerReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimConsumerReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_group: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_resource: Option<std::string::String> = None;
                let mut value_uid: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_group => value_api_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimConsumerReference {
                    api_group: value_api_group,
                    name: value_name.unwrap_or_default(),
                    resource: value_resource.unwrap_or_default(),
                    uid: value_uid.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimConsumerReference",
            &[
                "apiGroup",
                "name",
                "resource",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimConsumerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimConsumerReference",
            3 +
            self.api_group.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroup", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimConsumerReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.ResourceClaimConsumerReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceClaimConsumerReference contains enough information to let you locate the consumer of a ResourceClaim. The user must be a resource in the same namespace as the ResourceClaim.",
            "type": "object",
            "properties": {
                "apiGroup": {
                    "description": "APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.",
                    "type": "string",
                },
                "name": {
                    "description": "Name is the name of resource being referenced.",
                    "type": "string",
                },
                "resource": {
                    "description": "Resource is the type of resource being referenced, for example \"pods\".",
                    "type": "string",
                },
                "uid": {
                    "description": "UID identifies exactly one incarnation of the resource.",
                    "type": "string",
                },
            },
            "required": [
                "name",
                "resource",
                "uid",
            ],
        })
    }
}
