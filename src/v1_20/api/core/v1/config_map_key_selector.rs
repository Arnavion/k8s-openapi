// Generated from definition io.k8s.api.core.v1.ConfigMapKeySelector

/// Selects a key from a ConfigMap.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapKeySelector {
    /// The key to select.
    pub key: String,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Specify whether the ConfigMap or its key must be defined
    pub optional: Option<bool>,
}

impl<'de> crate::serde::Deserialize<'de> for ConfigMapKeySelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_name,
            Key_optional,
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
                            "key" => Field::Key_key,
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigMapKeySelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ConfigMapKeySelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapKeySelector {
                    key: value_key.ok_or_else(|| crate::serde::de::Error::missing_field("key"))?,
                    name: value_name,
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapKeySelector",
            &[
                "key",
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ConfigMapKeySelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapKeySelector",
            1 +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ConfigMapKeySelector {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Selects a key from a ConfigMap.",
          "properties": {
            "key": {
              "description": "The key to select.",
              "type": "string"
            },
            "name": {
              "description": "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names",
              "type": "string"
            },
            "optional": {
              "description": "Specify whether the ConfigMap or its key must be defined",
              "type": "boolean"
            }
          },
          "required": [
            "key"
          ],
          "type": "object"
        })
    }
}
