// Generated from definition io.k8s.api.core.v1.ClientIPConfig

/// ClientIPConfig represents the configurations of Client IP based session affinity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClientIPConfig {
    /// timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be \>0 && \<=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
    pub timeout_seconds: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for ClientIPConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_timeout_seconds,
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
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientIPConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ClientIPConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_timeout_seconds => value_timeout_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClientIPConfig {
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClientIPConfig",
            &[
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClientIPConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClientIPConfig",
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.timeout_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
