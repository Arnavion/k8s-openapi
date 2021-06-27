// Generated from definition io.k8s.api.core.v1.HostAlias

/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HostAlias {
    /// Hostnames for the above IP address.
    pub hostnames: Vec<String>,

    /// IP address of the host file entry.
    pub ip: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for HostAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostnames,
            Key_ip,
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
                            "hostnames" => Field::Key_hostnames,
                            "ip" => Field::Key_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HostAlias;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HostAlias")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hostnames: Option<Vec<String>> = None;
                let mut value_ip: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostnames => value_hostnames = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HostAlias {
                    hostnames: value_hostnames.unwrap_or_default(),
                    ip: value_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "HostAlias",
            &[
                "hostnames",
                "ip",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HostAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HostAlias",
            usize::from(!self.hostnames.is_empty()) +
            self.ip.as_ref().map_or(0, |_| 1),
        )?;
        if !self.hostnames.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostnames", &self.hostnames)?;
        }
        if let Some(value) = &self.ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl HostAlias {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.",
          "properties": {
            "hostnames": {
              "description": "Hostnames for the above IP address.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "ip": {
              "description": "IP address of the host file entry.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
