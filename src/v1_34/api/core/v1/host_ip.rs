// Generated from definition io.k8s.api.core.v1.HostIP

/// HostIP represents a single IP address allocated to the host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HostIP {
    /// IP is the IP address assigned to the host
    pub ip: std::string::String,
}

impl crate::DeepMerge for HostIP {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ip, other.ip);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HostIP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip,
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
            type Value = HostIP;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HostIP")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HostIP {
                    ip: value_ip.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "HostIP",
            &[
                "ip",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HostIP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HostIP",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", &self.ip)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HostIP {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.HostIP".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "HostIP represents a single IP address allocated to the host.",
            "type": "object",
            "properties": {
                "ip": {
                    "description": "IP is the IP address assigned to the host",
                    "type": "string",
                },
            },
            "required": [
                "ip",
            ],
        })
    }
}
