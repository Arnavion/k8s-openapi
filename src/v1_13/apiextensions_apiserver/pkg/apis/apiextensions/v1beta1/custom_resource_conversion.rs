// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceConversion

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceConversion {
    /// `strategy` specifies the conversion strategy. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the CR. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information is needed for this option.
    pub strategy: String,

    /// `webhookClientConfig` is the instructions for how to call the webhook if strategy is `Webhook`. This field is alpha-level and is only honored by servers that enable the CustomResourceWebhookConversion feature.
    pub webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_strategy,
            Key_webhook_client_config,
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
                            "strategy" => Field::Key_strategy,
                            "webhookClientConfig" => Field::Key_webhook_client_config,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_strategy: Option<String> = None;
                let mut value_webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_strategy => value_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_webhook_client_config => value_webhook_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceConversion {
                    strategy: value_strategy.unwrap_or_default(),
                    webhook_client_config: value_webhook_client_config,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceConversion",
            &[
                "strategy",
                "webhookClientConfig",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceConversion",
            1 +
            self.webhook_client_config.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        if let Some(value) = &self.webhook_client_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "webhookClientConfig", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceConversion {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceConversion".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CustomResourceConversion describes how to convert different versions of a CR.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: std::array::IntoIter::new([
                    (
                        "strategy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`strategy` specifies the conversion strategy. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the CR. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information is needed for this option.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "webhookClientConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`webhookClientConfig` is the instructions for how to call the webhook if strategy is `Webhook`. This field is alpha-level and is only honored by servers that enable the CustomResourceWebhookConversion feature.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: std::array::IntoIter::new([
                    "strategy",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
