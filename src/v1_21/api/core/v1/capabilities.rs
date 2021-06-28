// Generated from definition io.k8s.api.core.v1.Capabilities

/// Adds and removes POSIX capabilities from running containers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Capabilities {
    /// Added capabilities
    pub add: Vec<String>,

    /// Removed capabilities
    pub drop: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for Capabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_add,
            Key_drop,
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
                            "add" => Field::Key_add,
                            "drop" => Field::Key_drop,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Capabilities;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Capabilities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_add: Option<Vec<String>> = None;
                let mut value_drop: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_add => value_add = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_drop => value_drop = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Capabilities {
                    add: value_add.unwrap_or_default(),
                    drop: value_drop.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Capabilities",
            &[
                "add",
                "drop",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Capabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Capabilities",
            usize::from(!self.add.is_empty()) +
            usize::from(!self.drop.is_empty()),
        )?;
        if !self.add.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "add", &self.add)?;
        }
        if !self.drop.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "drop", &self.drop)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Capabilities {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Adds and removes POSIX capabilities from running containers.",
          "properties": {
            "add": {
              "description": "Added capabilities",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "drop": {
              "description": "Removed capabilities",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
