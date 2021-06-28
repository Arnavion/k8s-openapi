// Generated from definition io.k8s.api.core.v1.PortStatus

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortStatus {
    /// Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    pub error: Option<String>,

    /// Port is the port number of the service port of which status is recorded here
    pub port: i32,

    /// Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
    pub protocol: String,
}

impl<'de> crate::serde::Deserialize<'de> for PortStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_error,
            Key_port,
            Key_protocol,
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
                            "error" => Field::Key_error,
                            "port" => Field::Key_port,
                            "protocol" => Field::Key_protocol,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PortStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PortStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_error: Option<String> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_error => value_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_protocol => value_protocol = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PortStatus {
                    error: value_error,
                    port: value_port.ok_or_else(|| crate::serde::de::Error::missing_field("port"))?,
                    protocol: value_protocol.ok_or_else(|| crate::serde::de::Error::missing_field("protocol"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PortStatus",
            &[
                "error",
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PortStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PortStatus",
            2 +
            self.error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", &self.protocol)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PortStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "properties": {
            "error": {
              "description": "Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use\n  CamelCase names\n- cloud provider specific error values must have names that comply with the\n  format foo.example.com/CamelCase.",
              "type": "string"
            },
            "port": {
              "description": "Port is the port number of the service port of which status is recorded here",
              "format": "int32",
              "type": "integer"
            },
            "protocol": {
              "description": "Protocol is the protocol of the service port of which status is recorded here The supported values are: \"TCP\", \"UDP\", \"SCTP\"",
              "type": "string"
            }
          },
          "required": [
            "port",
            "protocol"
          ],
          "type": "object"
        })
    }
}
