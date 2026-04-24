// Generated from definition io.k8s.api.core.v1.PodExtendedResourceClaimStatus

/// PodExtendedResourceClaimStatus is stored in the PodStatus for the extended resource requests backed by DRA. It stores the generated name for the corresponding special ResourceClaim created by the scheduler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodExtendedResourceClaimStatus {
    /// RequestMappings identifies the mapping of \<container, extended resource backed by DRA\> to  device request in the generated ResourceClaim.
    pub request_mappings: std::vec::Vec<crate::api::core::v1::ContainerExtendedResourceRequest>,

    /// ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod.
    pub resource_claim_name: std::string::String,
}

impl crate::DeepMerge for PodExtendedResourceClaimStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.request_mappings, other.request_mappings);
        crate::DeepMerge::merge_from(&mut self.resource_claim_name, other.resource_claim_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodExtendedResourceClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_request_mappings,
            Key_resource_claim_name,
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
                            "requestMappings" => Field::Key_request_mappings,
                            "resourceClaimName" => Field::Key_resource_claim_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodExtendedResourceClaimStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodExtendedResourceClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_request_mappings: Option<std::vec::Vec<crate::api::core::v1::ContainerExtendedResourceRequest>> = None;
                let mut value_resource_claim_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_request_mappings => value_request_mappings = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_name => value_resource_claim_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodExtendedResourceClaimStatus {
                    request_mappings: value_request_mappings.unwrap_or_default(),
                    resource_claim_name: value_resource_claim_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodExtendedResourceClaimStatus",
            &[
                "requestMappings",
                "resourceClaimName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodExtendedResourceClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodExtendedResourceClaimStatus",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requestMappings", &self.request_mappings)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimName", &self.resource_claim_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodExtendedResourceClaimStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.PodExtendedResourceClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodExtendedResourceClaimStatus is stored in the PodStatus for the extended resource requests backed by DRA. It stores the generated name for the corresponding special ResourceClaim created by the scheduler.",
            "type": "object",
            "properties": {
                "requestMappings": {
                    "description": "RequestMappings identifies the mapping of <container, extended resource backed by DRA> to  device request in the generated ResourceClaim.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ContainerExtendedResourceRequest>()),
                },
                "resourceClaimName": {
                    "description": "ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod.",
                    "type": "string",
                },
            },
            "required": [
                "requestMappings",
                "resourceClaimName",
            ],
        })
    }
}
