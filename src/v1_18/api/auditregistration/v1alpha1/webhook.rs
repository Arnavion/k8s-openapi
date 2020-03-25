// Generated from definition io.k8s.api.auditregistration.v1alpha1.Webhook

/// Webhook holds the configuration of the webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Webhook {
    /// ClientConfig holds the connection parameters for the webhook required
    pub client_config: crate::api::auditregistration::v1alpha1::WebhookClientConfig,

    /// Throttle holds the options for throttling the webhook
    pub throttle: Option<crate::api::auditregistration::v1alpha1::WebhookThrottleConfig>,
}

impl<'de> serde::Deserialize<'de> for Webhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_throttle,
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
                            "clientConfig" => Field::Key_client_config,
                            "throttle" => Field::Key_throttle,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Webhook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Webhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_client_config: Option<crate::api::auditregistration::v1alpha1::WebhookClientConfig> = None;
                let mut value_throttle: Option<crate::api::auditregistration::v1alpha1::WebhookThrottleConfig> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_throttle => value_throttle = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Webhook {
                    client_config: value_client_config.ok_or_else(|| serde::de::Error::missing_field("clientConfig"))?,
                    throttle: value_throttle,
                })
            }
        }

        deserializer.deserialize_struct(
            "Webhook",
            &[
                "clientConfig",
                "throttle",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Webhook",
            1 +
            self.throttle.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.throttle {
            serde::ser::SerializeStruct::serialize_field(&mut state, "throttle", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
