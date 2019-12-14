// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.WebhookConversion

/// WebhookConversion describes how to call a conversion webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebhookConversion {
    /// clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
    pub client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig>,

    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail.
    pub conversion_review_versions: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for WebhookConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_conversion_review_versions,
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
                            "conversionReviewVersions" => Field::Key_conversion_review_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebhookConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WebhookConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig> = None;
                let mut value_conversion_review_versions: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conversion_review_versions => value_conversion_review_versions = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebhookConversion {
                    client_config: value_client_config,
                    conversion_review_versions: value_conversion_review_versions.ok_or_else(|| serde::de::Error::missing_field("conversionReviewVersions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "WebhookConversion",
            &[
                "clientConfig",
                "conversionReviewVersions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for WebhookConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WebhookConversion",
            1 +
            self.client_config.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.client_config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "conversionReviewVersions", &self.conversion_review_versions)?;
        serde::ser::SerializeStruct::end(state)
    }
}
