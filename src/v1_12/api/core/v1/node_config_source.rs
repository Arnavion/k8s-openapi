// Generated from definition io.k8s.api.core.v1.NodeConfigSource

/// NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeConfigSource {
    /// ConfigMap is a reference to a Node's ConfigMap
    pub config_map: Option<crate::api::core::v1::ConfigMapNodeConfigSource>,
}

impl<'de> crate::serde::Deserialize<'de> for NodeConfigSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map,
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
                            "configMap" => Field::Key_config_map,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeConfigSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeConfigSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config_map: Option<crate::api::core::v1::ConfigMapNodeConfigSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map => value_config_map = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeConfigSource {
                    config_map: value_config_map,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeConfigSource",
            &[
                "configMap",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeConfigSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeConfigSource",
            self.config_map.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_map {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NodeConfigSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil.",
          "properties": {
            "configMap": crate::schema_ref_with_description(crate::api::core::v1::ConfigMapNodeConfigSource::schema(), "ConfigMap is a reference to a Node's ConfigMap")
          },
          "type": "object"
        })
    }
}
