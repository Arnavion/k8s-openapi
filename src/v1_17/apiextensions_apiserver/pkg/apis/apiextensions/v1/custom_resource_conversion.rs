// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceConversion

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceConversion {
    /// strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information
    ///   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
    pub strategy: String,

    /// webhook describes how to call the conversion webhook. Required when `strategy` is set to `Webhook`.
    pub webhook: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_strategy,
            Key_webhook,
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
                            "webhook" => Field::Key_webhook,
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
                let mut value_webhook: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_strategy => value_strategy = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_webhook => value_webhook = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceConversion {
                    strategy: value_strategy.ok_or_else(|| crate::serde::de::Error::missing_field("strategy"))?,
                    webhook: value_webhook,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceConversion",
            &[
                "strategy",
                "webhook",
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
            self.webhook.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        if let Some(value) = &self.webhook {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "webhook", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
