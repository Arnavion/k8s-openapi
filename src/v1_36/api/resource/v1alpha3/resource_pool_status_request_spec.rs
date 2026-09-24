// Generated from definition io.k8s.api.resource.v1alpha3.ResourcePoolStatusRequestSpec

/// ResourcePoolStatusRequestSpec defines the filters for the pool status request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePoolStatusRequestSpec {
    /// Driver specifies the DRA driver name to filter pools. Only pools from ResourceSlices with this driver will be included. Must be a DNS subdomain (e.g., "gpu.example.com").
    pub driver: std::string::String,

    /// Limit optionally specifies the maximum number of pools to return in the status. If more pools match the filter criteria, the response will be truncated (i.e., len(status.pools) \< status.poolCount).
    ///
    /// Default: 100 Minimum: 1 Maximum: 1000
    pub limit: Option<i32>,

    /// PoolName optionally filters to a specific pool name. If not specified, all pools from the specified driver are included. When specified, must be a non-empty valid resource pool name (DNS subdomains separated by "/").
    pub pool_name: Option<std::string::String>,
}

impl crate::DeepMerge for ResourcePoolStatusRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.limit, other.limit);
        crate::DeepMerge::merge_from(&mut self.pool_name, other.pool_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourcePoolStatusRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_limit,
            Key_pool_name,
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
                            "driver" => Field::Key_driver,
                            "limit" => Field::Key_limit,
                            "poolName" => Field::Key_pool_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePoolStatusRequestSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourcePoolStatusRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<std::string::String> = None;
                let mut value_limit: Option<i32> = None;
                let mut value_pool_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_limit => value_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool_name => value_pool_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourcePoolStatusRequestSpec {
                    driver: value_driver.unwrap_or_default(),
                    limit: value_limit,
                    pool_name: value_pool_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourcePoolStatusRequestSpec",
            &[
                "driver",
                "limit",
                "poolName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourcePoolStatusRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourcePoolStatusRequestSpec",
            1 +
            self.limit.as_ref().map_or(0, |_| 1) +
            self.pool_name.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "limit", value)?;
        }
        if let Some(value) = &self.pool_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "poolName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourcePoolStatusRequestSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.ResourcePoolStatusRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourcePoolStatusRequestSpec defines the filters for the pool status request.",
            "type": "object",
            "properties": {
                "driver": {
                    "description": "Driver specifies the DRA driver name to filter pools. Only pools from ResourceSlices with this driver will be included. Must be a DNS subdomain (e.g., \"gpu.example.com\").",
                    "type": "string",
                },
                "limit": {
                    "description": "Limit optionally specifies the maximum number of pools to return in the status. If more pools match the filter criteria, the response will be truncated (i.e., len(status.pools) < status.poolCount).\n\nDefault: 100 Minimum: 1 Maximum: 1000",
                    "type": "integer",
                    "format": "int32",
                },
                "poolName": {
                    "description": "PoolName optionally filters to a specific pool name. If not specified, all pools from the specified driver are included. When specified, must be a non-empty valid resource pool name (DNS subdomains separated by \"/\").",
                    "type": "string",
                },
            },
            "required": [
                "driver",
            ],
        })
    }
}
