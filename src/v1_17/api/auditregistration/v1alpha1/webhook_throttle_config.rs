// Generated from definition io.k8s.api.auditregistration.v1alpha1.WebhookThrottleConfig

/// WebhookThrottleConfig holds the configuration for throttling events
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebhookThrottleConfig {
    /// ThrottleBurst is the maximum number of events sent at the same moment default 15 QPS
    pub burst: Option<i64>,

    /// ThrottleQPS maximum number of batches per second default 10 QPS
    pub qps: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for WebhookThrottleConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_burst,
            Key_qps,
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
                            "burst" => Field::Key_burst,
                            "qps" => Field::Key_qps,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebhookThrottleConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WebhookThrottleConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_burst: Option<i64> = None;
                let mut value_qps: Option<i64> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_burst => value_burst = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_qps => value_qps = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebhookThrottleConfig {
                    burst: value_burst,
                    qps: value_qps,
                })
            }
        }

        deserializer.deserialize_struct(
            "WebhookThrottleConfig",
            &[
                "burst",
                "qps",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for WebhookThrottleConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WebhookThrottleConfig",
            self.burst.as_ref().map_or(0, |_| 1) +
            self.qps.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.burst {
            serde::ser::SerializeStruct::serialize_field(&mut state, "burst", value)?;
        }
        if let Some(value) = &self.qps {
            serde::ser::SerializeStruct::serialize_field(&mut state, "qps", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
