// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ShardInfo

/// ShardInfo describes the shard selector that was applied to produce a list response. Its presence on a list response indicates the list is a filtered subset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShardInfo {
    /// selector is the shard selector string from the request, echoed back so clients can verify which shard they received and merge responses from multiple shards.
    pub selector: std::string::String,
}

impl crate::DeepMerge for ShardInfo {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ShardInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_selector,
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
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ShardInfo;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ShardInfo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_selector: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ShardInfo {
                    selector: value_selector.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ShardInfo",
            &[
                "selector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ShardInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ShardInfo",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ShardInfo {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.ShardInfo".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ShardInfo describes the shard selector that was applied to produce a list response. Its presence on a list response indicates the list is a filtered subset.",
            "type": "object",
            "properties": {
                "selector": {
                    "description": "selector is the shard selector string from the request, echoed back so clients can verify which shard they received and merge responses from multiple shards.",
                    "type": "string",
                },
            },
            "required": [
                "selector",
            ],
        })
    }
}
