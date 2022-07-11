// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.WebhookConversion

/// WebhookConversion describes how to call a conversion webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebhookConversion {
    /// clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
    pub client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig>,

    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail.
    pub conversion_review_versions: Vec<String>,

}

#[cfg(feature = "dsl")]
impl WebhookConversion  {
    /// Set [`Self::client_config`]
    pub  fn client_config_set(&mut self, client_config: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig>>) -> &mut Self {
        self.client_config = client_config.into(); self
    }

    pub  fn client_config(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig {
        if self.client_config.is_none() { self.client_config = Some(Default::default()) }
        self.client_config.as_mut().unwrap()
    }

    /// Modify [`Self::client_config`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn client_config_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig)) -> &mut Self {
        if self.client_config.is_none() { self.client_config = Some(Default::default()) };
        func(self.client_config.as_mut().unwrap()); self
    }


    /// Set [`Self::conversion_review_versions`]
    pub  fn conversion_review_versions_set(&mut self, conversion_review_versions: impl Into<Vec<String>>) -> &mut Self {
        self.conversion_review_versions = conversion_review_versions.into(); self
    }

    pub  fn conversion_review_versions(&mut self) -> &mut Vec<String> {
        &mut self.conversion_review_versions
    }

    /// Modify [`Self::conversion_review_versions`] with a `func`
    pub  fn conversion_review_versions_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.conversion_review_versions); self
    }

    /// Push new element to [`Self::conversion_review_versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conversion_review_versions_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.conversion_review_versions.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::conversion_review_versions`]
    pub  fn conversion_review_versions_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.conversion_review_versions.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for WebhookConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_conversion_review_versions,
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
                            "conversionReviewVersions" => Field::Key_conversion_review_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WebhookConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WebhookConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig> = None;
                let mut value_conversion_review_versions: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conversion_review_versions => value_conversion_review_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebhookConversion {
                    client_config: value_client_config,
                    conversion_review_versions: value_conversion_review_versions.unwrap_or_default(),
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

impl crate::serde::Serialize for WebhookConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WebhookConversion",
            1 +
            self.client_config.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.client_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversionReviewVersions", &self.conversion_review_versions)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WebhookConversion {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.WebhookConversion".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("WebhookConversion describes how to call a conversion webhook".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "clientConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "conversionReviewVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "conversionReviewVersions".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
