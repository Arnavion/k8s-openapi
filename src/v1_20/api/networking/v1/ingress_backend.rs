// Generated from definition io.k8s.api.networking.v1.IngressBackend

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressBackend {
    /// Resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a service.Name and service.Port must not be specified. This is a mutually exclusive setting with "Service".
    pub resource: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Service references a Service as a Backend. This is a mutually exclusive setting with "Resource".
    pub service: Option<crate::api::networking::v1::IngressServiceBackend>,
}

impl<'de> serde::Deserialize<'de> for IngressBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource,
            Key_service,
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
                            "resource" => Field::Key_resource,
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
            type Value = IngressBackend;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressBackend")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_resource: Option<crate::api::core::v1::TypedLocalObjectReference> = None;
                let mut value_service: Option<crate::api::networking::v1::IngressServiceBackend> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource => value_resource = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service => value_service = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressBackend {
                    resource: value_resource,
                    service: value_service,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressBackend",
            &[
                "resource",
                "service",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IngressBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressBackend",
            self.resource.as_ref().map_or(0, |_| 1) +
            self.service.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.resource {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        if let Some(value) = &self.service {
            serde::ser::SerializeStruct::serialize_field(&mut state, "service", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
