// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceConversion

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceConversion {
    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. Defaults to `\["v1beta1"\]`.
    pub conversion_review_versions: Option<Vec<String>>,

    /// strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information
    ///   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set.
    pub strategy: String,

    /// webhookClientConfig is the instructions for how to call the webhook if strategy is `Webhook`. Required when `strategy` is set to `Webhook`.
    pub webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig>,
}

impl<'de> serde::Deserialize<'de> for CustomResourceConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conversion_review_versions,
            Key_strategy,
            Key_webhook_client_config,
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
                            "conversionReviewVersions" => Field::Key_conversion_review_versions,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conversion_review_versions: Option<Vec<String>> = None;
                let mut value_strategy: Option<String> = None;
                let mut value_webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conversion_review_versions => value_conversion_review_versions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_webhook_client_config => value_webhook_client_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceConversion {
                    conversion_review_versions: value_conversion_review_versions,
                    strategy: value_strategy.ok_or_else(|| serde::de::Error::missing_field("strategy"))?,
                    webhook_client_config: value_webhook_client_config,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceConversion",
            &[
                "conversionReviewVersions",
                "strategy",
                "webhookClientConfig",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomResourceConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceConversion",
            1 +
            self.conversion_review_versions.as_ref().map_or(0, |_| 1) +
            self.webhook_client_config.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conversion_review_versions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conversionReviewVersions", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        if let Some(value) = &self.webhook_client_config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "webhookClientConfig", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
