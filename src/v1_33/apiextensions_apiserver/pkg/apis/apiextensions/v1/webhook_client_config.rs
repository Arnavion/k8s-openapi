// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.WebhookClientConfig

/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebhookClientConfig {
    /// caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.
    pub ca_bundle: Option<crate::ByteString>,

    /// service is a reference to the service for this webhook. Either service or url must be specified.
    ///
    /// If the webhook is running within the cluster, then you should use `service`.
    pub service: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference>,

    /// url gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.
    ///
    /// The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.
    ///
    /// Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.
    ///
    /// The scheme must be "https"; the URL must begin with "https://".
    ///
    /// A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.
    ///
    /// Attempting to use a user or basic auth e.g. "user:password@" is not allowed. Fragments ("#...") and query parameters ("?...") are not allowed, either.
    pub url: Option<std::string::String>,
}

impl crate::DeepMerge for WebhookClientConfig {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ca_bundle, other.ca_bundle);
        crate::DeepMerge::merge_from(&mut self.service, other.service);
        crate::DeepMerge::merge_from(&mut self.url, other.url);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WebhookClientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ca_bundle,
            Key_service,
            Key_url,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "caBundle" => Field::Key_ca_bundle,
                            "service" => Field::Key_service,
                            "url" => Field::Key_url,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WebhookClientConfig;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WebhookClientConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ca_bundle: Option<crate::ByteString> = None;
                let mut value_service: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference> = None;
                let mut value_url: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ca_bundle => value_ca_bundle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service => value_service = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_url => value_url = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebhookClientConfig {
                    ca_bundle: value_ca_bundle,
                    service: value_service,
                    url: value_url,
                })
            }
        }

        deserializer.deserialize_struct(
            "WebhookClientConfig",
            &[
                "caBundle",
                "service",
                "url",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WebhookClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WebhookClientConfig",
            self.ca_bundle.as_ref().map_or(0, |_| 1) +
            self.service.as_ref().map_or(0, |_| 1) +
            self.url.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ca_bundle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "caBundle", value)?;
        }
        if let Some(value) = &self.service {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "service", value)?;
        }
        if let Some(value) = &self.url {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "url", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WebhookClientConfig {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.WebhookClientConfig".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("WebhookClientConfig contains the information to make a TLS connection with the webhook.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "caBundle".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            format: Some("byte".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "service".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("service is a reference to the service for this webhook. Either service or url must be specified.\n\nIf the webhook is running within the cluster, then you should use `service`.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "url".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("url gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.\n\nThe `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.\n\nPlease note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.\n\nThe scheme must be \"https\"; the URL must begin with \"https://\".\n\nA path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.\n\nAttempting to use a user or basic auth e.g. \"user:password@\" is not allowed. Fragments (\"#...\") and query parameters (\"?...\") are not allowed, either.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
