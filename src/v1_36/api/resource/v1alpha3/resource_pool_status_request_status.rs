// Generated from definition io.k8s.api.resource.v1alpha3.ResourcePoolStatusRequestStatus

/// ResourcePoolStatusRequestStatus contains the calculated pool status information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePoolStatusRequestStatus {
    /// Conditions provide information about the state of the request. A condition with type=Complete or type=Failed will always be set when the status is populated.
    ///
    /// Known condition types: - "Complete": True when the request has been processed successfully - "Failed": True when the request could not be processed
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// PoolCount is the total number of pools that matched the filter criteria, regardless of truncation. This helps users understand how many pools exist even when the response is truncated. A value of 0 means no pools matched the filter criteria.
    pub pool_count: i32,

    /// Pools contains the first `spec.limit` matching pools, sorted by driver then pool name. If `len(pools) \< poolCount`, the list was truncated. When omitted, no pools matched the request filters.
    pub pools: Option<std::vec::Vec<crate::api::resource::v1alpha3::PoolStatus>>,
}

impl crate::DeepMerge for ResourcePoolStatusRequestStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.pool_count, other.pool_count);
        crate::merge_strategies::list::atomic(&mut self.pools, other.pools);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourcePoolStatusRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_pool_count,
            Key_pools,
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
                            "conditions" => Field::Key_conditions,
                            "poolCount" => Field::Key_pool_count,
                            "pools" => Field::Key_pools,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePoolStatusRequestStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourcePoolStatusRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_pool_count: Option<i32> = None;
                let mut value_pools: Option<std::vec::Vec<crate::api::resource::v1alpha3::PoolStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool_count => value_pool_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pools => value_pools = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourcePoolStatusRequestStatus {
                    conditions: value_conditions,
                    pool_count: value_pool_count.unwrap_or_default(),
                    pools: value_pools,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourcePoolStatusRequestStatus",
            &[
                "conditions",
                "poolCount",
                "pools",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourcePoolStatusRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourcePoolStatusRequestStatus",
            1 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.pools.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "poolCount", &self.pool_count)?;
        if let Some(value) = &self.pools {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pools", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourcePoolStatusRequestStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.ResourcePoolStatusRequestStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourcePoolStatusRequestStatus contains the calculated pool status information.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "Conditions provide information about the state of the request. A condition with type=Complete or type=Failed will always be set when the status is populated.\n\nKnown condition types: - \"Complete\": True when the request has been processed successfully - \"Failed\": True when the request could not be processed",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "poolCount": {
                    "description": "PoolCount is the total number of pools that matched the filter criteria, regardless of truncation. This helps users understand how many pools exist even when the response is truncated. A value of 0 means no pools matched the filter criteria.",
                    "type": "integer",
                    "format": "int32",
                },
                "pools": {
                    "description": "Pools contains the first `spec.limit` matching pools, sorted by driver then pool name. If `len(pools) < poolCount`, the list was truncated. When omitted, no pools matched the request filters.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1alpha3::PoolStatus>()),
                },
            },
            "required": [
                "poolCount",
            ],
        })
    }
}
