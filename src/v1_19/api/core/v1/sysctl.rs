// Generated from definition io.k8s.api.core.v1.Sysctl

/// Sysctl defines a kernel parameter to be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sysctl {
    /// Name of a property to set
    pub name: String,

    /// Value of a property to set
    pub value: String,
}

impl<'de> crate::serde::Deserialize<'de> for Sysctl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_value,
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
                            "name" => Field::Key_name,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Sysctl;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Sysctl")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Sysctl {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    value: value_value.ok_or_else(|| crate::serde::de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Sysctl",
            &[
                "name",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Sysctl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Sysctl",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Sysctl {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Sysctl defines a kernel parameter to be set",
          "properties": {
            "name": {
              "description": "Name of a property to set",
              "type": "string"
            },
            "value": {
              "description": "Value of a property to set",
              "type": "string"
            }
          },
          "required": [
            "name",
            "value"
          ],
          "type": "object"
        })
    }
}
