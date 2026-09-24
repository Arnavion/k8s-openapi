// Generated from definition io.k8s.api.core.v1.ContainerExtendedResourceRequest

/// ContainerExtendedResourceRequest has the mapping of container name, extended resource name to the device request name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerExtendedResourceRequest {
    /// The name of the container requesting resources.
    pub container_name: std::string::String,

    /// The name of the request in the special ResourceClaim which corresponds to the extended resource.
    pub request_name: std::string::String,

    /// The name of the extended resource in that container which gets backed by DRA.
    pub resource_name: std::string::String,
}

impl crate::DeepMerge for ContainerExtendedResourceRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container_name, other.container_name);
        crate::DeepMerge::merge_from(&mut self.request_name, other.request_name);
        crate::DeepMerge::merge_from(&mut self.resource_name, other.resource_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerExtendedResourceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_name,
            Key_request_name,
            Key_resource_name,
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
                            "containerName" => Field::Key_container_name,
                            "requestName" => Field::Key_request_name,
                            "resourceName" => Field::Key_resource_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerExtendedResourceRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerExtendedResourceRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_name: Option<std::string::String> = None;
                let mut value_request_name: Option<std::string::String> = None;
                let mut value_resource_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_name => value_container_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request_name => value_request_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_name => value_resource_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerExtendedResourceRequest {
                    container_name: value_container_name.unwrap_or_default(),
                    request_name: value_request_name.unwrap_or_default(),
                    resource_name: value_resource_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerExtendedResourceRequest",
            &[
                "containerName",
                "requestName",
                "resourceName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerExtendedResourceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerExtendedResourceRequest",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", &self.container_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requestName", &self.request_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceName", &self.resource_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerExtendedResourceRequest {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerExtendedResourceRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerExtendedResourceRequest has the mapping of container name, extended resource name to the device request name.",
            "type": "object",
            "properties": {
                "containerName": {
                    "description": "The name of the container requesting resources.",
                    "type": "string",
                },
                "requestName": {
                    "description": "The name of the request in the special ResourceClaim which corresponds to the extended resource.",
                    "type": "string",
                },
                "resourceName": {
                    "description": "The name of the extended resource in that container which gets backed by DRA.",
                    "type": "string",
                },
            },
            "required": [
                "containerName",
                "requestName",
                "resourceName",
            ],
        })
    }
}
