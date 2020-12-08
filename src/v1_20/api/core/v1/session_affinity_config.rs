// Generated from definition io.k8s.api.core.v1.SessionAffinityConfig

/// SessionAffinityConfig represents the configurations of session affinity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SessionAffinityConfig {
    /// clientIP contains the configurations of Client IP based session affinity.
    pub client_ip: Option<crate::api::core::v1::ClientIPConfig>,
}

impl<'de> serde::Deserialize<'de> for SessionAffinityConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_ip,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "clientIP" => Field::Key_client_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SessionAffinityConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SessionAffinityConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_client_ip: Option<crate::api::core::v1::ClientIPConfig> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_ip => value_client_ip = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SessionAffinityConfig {
                    client_ip: value_client_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "SessionAffinityConfig",
            &[
                "clientIP",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SessionAffinityConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SessionAffinityConfig",
            self.client_ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.client_ip {
            serde::ser::SerializeStruct::serialize_field(&mut state, "clientIP", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
