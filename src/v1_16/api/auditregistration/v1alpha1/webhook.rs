// Generated from definition io.k8s.api.auditregistration.v1alpha1.Webhook

/// Webhook holds the configuration of the webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Webhook {
    /// ClientConfig holds the connection parameters for the webhook required
    pub client_config: crate::api::auditregistration::v1alpha1::WebhookClientConfig,

    /// Throttle holds the options for throttling the webhook
    pub throttle: Option<crate::api::auditregistration::v1alpha1::WebhookThrottleConfig>,
}

impl<'de> crate::serde::Deserialize<'de> for Webhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_throttle,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Webhook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Webhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_client_config: Option<crate::api::auditregistration::v1alpha1::WebhookClientConfig> = None;
                let mut value_throttle: Option<crate::api::auditregistration::v1alpha1::WebhookThrottleConfig> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_throttle => value_throttle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Webhook {
                    client_config: value_client_config.unwrap_or_default(),
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

impl crate::serde::Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Webhook",
            1 +
            self.throttle.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.throttle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "throttle", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Webhook {
    fn schema_name() -> String {
        "io.k8s.api.auditregistration.v1alpha1.Webhook".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Webhook holds the configuration of the webhook".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "clientConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::auditregistration::v1alpha1::WebhookClientConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ClientConfig holds the connection parameters for the webhook required".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "throttle".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::auditregistration::v1alpha1::WebhookThrottleConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Throttle holds the options for throttling the webhook".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "clientConfig".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
