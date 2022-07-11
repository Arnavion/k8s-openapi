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

#[cfg(feature = "dsl")]
impl CustomResourceConversion  {
    /// Set [`Self::conversion_review_versions`]
    pub  fn conversion_review_versions_set(&mut self, conversion_review_versions: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.conversion_review_versions = conversion_review_versions.into(); self
    }

    pub  fn conversion_review_versions(&mut self) -> &mut Vec<String> {
        if self.conversion_review_versions.is_none() { self.conversion_review_versions = Some(Default::default()) }
        self.conversion_review_versions.as_mut().unwrap()
    }

    /// Modify [`Self::conversion_review_versions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conversion_review_versions_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.conversion_review_versions.is_none() { self.conversion_review_versions = Some(Default::default()) };
        func(self.conversion_review_versions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conversion_review_versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conversion_review_versions_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.conversion_review_versions.is_none() {
            self.conversion_review_versions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conversion_review_versions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conversion_review_versions`]
    pub  fn conversion_review_versions_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.conversion_review_versions.is_none() { self.conversion_review_versions = Some(Vec::new()); }
         let conversion_review_versions = &mut self.conversion_review_versions.as_mut().unwrap();
         for item in other.borrow() {
             conversion_review_versions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::strategy`]
    pub  fn strategy_set(&mut self, strategy: impl Into<String>) -> &mut Self {
        self.strategy = strategy.into(); self
    }

    pub  fn strategy(&mut self) -> &mut String {
        &mut self.strategy
    }

    /// Modify [`Self::strategy`] with a `func`
    pub  fn strategy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.strategy); self
    }


    /// Set [`Self::webhook_client_config`]
    pub  fn webhook_client_config_set(&mut self, webhook_client_config: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig>>) -> &mut Self {
        self.webhook_client_config = webhook_client_config.into(); self
    }

    pub  fn webhook_client_config(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig {
        if self.webhook_client_config.is_none() { self.webhook_client_config = Some(Default::default()) }
        self.webhook_client_config.as_mut().unwrap()
    }

    /// Modify [`Self::webhook_client_config`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn webhook_client_config_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig)) -> &mut Self {
        if self.webhook_client_config.is_none() { self.webhook_client_config = Some(Default::default()) };
        func(self.webhook_client_config.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for CustomResourceConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conversion_review_versions,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conversion_review_versions: Option<Vec<String>> = None;
                let mut value_strategy: Option<String> = None;
                let mut value_webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conversion_review_versions => value_conversion_review_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_webhook_client_config => value_webhook_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceConversion {
                    conversion_review_versions: value_conversion_review_versions,
                    strategy: value_strategy.unwrap_or_default(),
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

impl crate::serde::Serialize for CustomResourceConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceConversion",
            1 +
            self.conversion_review_versions.as_ref().map_or(0, |_| 1) +
            self.webhook_client_config.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conversion_review_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversionReviewVersions", value)?;
        }
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
                properties: [
                    (
                        "conversionReviewVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. Defaults to `[\"v1beta1\"]`.".to_owned()),
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
                    (
                        "strategy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information\n  is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set.".to_owned()),
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
                                description: Some("webhookClientConfig is the instructions for how to call the webhook if strategy is `Webhook`. Required when `strategy` is set to `Webhook`.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "strategy".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
