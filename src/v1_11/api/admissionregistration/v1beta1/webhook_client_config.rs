// Generated from definition io.k8s.api.admissionregistration.v1beta1.WebhookClientConfig

/// WebhookClientConfig contains the information to make a TLS connection with the webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebhookClientConfig {
    /// `caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. Required.
    pub ca_bundle: crate::ByteString,

    /// `service` is a reference to the service for this webhook. Either `service` or `url` must be specified.
    ///
    /// If the webhook is running within the cluster, then you should use `service`.
    ///
    /// Port 443 will be used if it is open, otherwise it is an error.
    pub service: Option<crate::api::admissionregistration::v1beta1::ServiceReference>,

    /// `url` gives the location of the webhook, in standard URL form (`\[scheme://\]host:port/path`). Exactly one of `url` or `service` must be specified.
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
    pub url: Option<String>,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WebhookClientConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ca_bundle: Option<crate::ByteString> = None;
                let mut value_service: Option<crate::api::admissionregistration::v1beta1::ServiceReference> = None;
                let mut value_url: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ca_bundle => value_ca_bundle = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service => value_service = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_url => value_url = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WebhookClientConfig {
                    ca_bundle: value_ca_bundle.ok_or_else(|| crate::serde::de::Error::missing_field("caBundle"))?,
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
            1 +
            self.service.as_ref().map_or(0, |_| 1) +
            self.url.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "caBundle", &self.ca_bundle)?;
        if let Some(value) = &self.service {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "service", value)?;
        }
        if let Some(value) = &self.url {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "url", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for WebhookClientConfig {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "WebhookClientConfig contains the information to make a TLS connection with the webhook",
          "properties": {
            "caBundle": {
              "description": "`caBundle` is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. Required.",
              "format": "byte",
              "type": "string"
            },
            "service": crate::schema_ref_with_description(crate::api::admissionregistration::v1beta1::ServiceReference::schema(), "`service` is a reference to the service for this webhook. Either `service` or `url` must be specified.\n\nIf the webhook is running within the cluster, then you should use `service`.\n\nPort 443 will be used if it is open, otherwise it is an error."),
            "url": {
              "description": "`url` gives the location of the webhook, in standard URL form (`[scheme://]host:port/path`). Exactly one of `url` or `service` must be specified.\n\nThe `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.\n\nPlease note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.\n\nThe scheme must be \"https\"; the URL must begin with \"https://\".\n\nA path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.\n\nAttempting to use a user or basic auth e.g. \"user:password@\" is not allowed. Fragments (\"#...\") and query parameters (\"?...\") are not allowed, either.",
              "type": "string"
            }
          },
          "required": [
            "caBundle"
          ],
          "type": "object"
        })
    }
}
