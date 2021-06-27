// Generated from definition io.k8s.api.core.v1.DaemonEndpoint

/// DaemonEndpoint contains information about a single Daemon endpoint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    pub port: i32,
}

impl<'de> crate::serde::Deserialize<'de> for DaemonEndpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_port,
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
                            "Port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonEndpoint;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DaemonEndpoint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_port: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port => value_port = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonEndpoint {
                    port: value_port.ok_or_else(|| crate::serde::de::Error::missing_field("Port"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonEndpoint",
            &[
                "Port",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DaemonEndpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonEndpoint",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "Port", &self.port)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl DaemonEndpoint {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DaemonEndpoint contains information about a single Daemon endpoint.",
          "properties": {
            "Port": {
              "description": "Port number of the given endpoint.",
              "format": "int32",
              "type": "integer"
            }
          },
          "required": [
            "Port"
          ],
          "type": "object"
        })
    }
}
