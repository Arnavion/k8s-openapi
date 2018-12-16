// Generated from definition io.k8s.api.admissionregistration.v1alpha1.AdmissionHookClientConfig

/// AdmissionHookClientConfig contains the information to make a TLS connection with the webhook
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdmissionHookClientConfig {
    /// CABundle is a PEM encoded CA bundle which will be used to validate webhook's server certificate. Required
    pub ca_bundle: crate::ByteString,

    /// Service is a reference to the service for this webhook. If there is only one port open for the service, that port will be used. If there are multiple ports open, port 443 will be used if it is open, otherwise it is an error. Required
    pub service: crate::v1_8::api::admissionregistration::v1alpha1::ServiceReference,
}

impl<'de> serde::Deserialize<'de> for AdmissionHookClientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ca_bundle,
            Key_service,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "caBundle" => Field::Key_ca_bundle,
                            "service" => Field::Key_service,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdmissionHookClientConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct AdmissionHookClientConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ca_bundle: Option<crate::ByteString> = None;
                let mut value_service: Option<crate::v1_8::api::admissionregistration::v1alpha1::ServiceReference> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ca_bundle => value_ca_bundle = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service => value_service = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AdmissionHookClientConfig {
                    ca_bundle: value_ca_bundle.ok_or_else(|| serde::de::Error::missing_field("caBundle"))?,
                    service: value_service.ok_or_else(|| serde::de::Error::missing_field("service"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AdmissionHookClientConfig",
            &[
                "caBundle",
                "service",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AdmissionHookClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AdmissionHookClientConfig",
            0 +
            1 +
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "caBundle", &self.ca_bundle)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "service", &self.service)?;
        serde::ser::SerializeStruct::end(state)
    }
}
