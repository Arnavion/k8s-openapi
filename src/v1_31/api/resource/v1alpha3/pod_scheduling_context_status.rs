// Generated from definition io.k8s.api.resource.v1alpha3.PodSchedulingContextStatus

/// PodSchedulingContextStatus describes where resources for the Pod can be allocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSchedulingContextStatus {
    /// ResourceClaims describes resource availability for each pod.spec.resourceClaim entry where the corresponding ResourceClaim uses "WaitForFirstConsumer" allocation mode.
    pub resource_claims: Option<std::vec::Vec<crate::api::resource::v1alpha3::ResourceClaimSchedulingStatus>>,
}

impl crate::DeepMerge for PodSchedulingContextStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.resource_claims,
            other.resource_claims,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodSchedulingContextStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource_claims,
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
                            "resourceClaims" => Field::Key_resource_claims,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodSchedulingContextStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodSchedulingContextStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_resource_claims: Option<std::vec::Vec<crate::api::resource::v1alpha3::ResourceClaimSchedulingStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource_claims => value_resource_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSchedulingContextStatus {
                    resource_claims: value_resource_claims,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSchedulingContextStatus",
            &[
                "resourceClaims",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSchedulingContextStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSchedulingContextStatus",
            self.resource_claims.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.resource_claims {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaims", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSchedulingContextStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.PodSchedulingContextStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodSchedulingContextStatus describes where resources for the Pod can be allocated.",
            "type": "object",
            "properties": {
                "resourceClaims": {
                    "description": "ResourceClaims describes resource availability for each pod.spec.resourceClaim entry where the corresponding ResourceClaim uses \"WaitForFirstConsumer\" allocation mode.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1alpha3::ResourceClaimSchedulingStatus>()),
                },
            },
        })
    }
}
