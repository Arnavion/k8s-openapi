// Generated from definition io.k8s.api.networking.v1beta1.IngressBackend

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressBackend {
    /// Resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, serviceName and servicePort must not be specified.
    pub resource: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Specifies the name of the referenced service.
    pub service_name: Option<String>,

    /// Specifies the port of the referenced service.
    pub service_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,
}

impl<'de> serde::Deserialize<'de> for IngressBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource,
            Key_service_name,
            Key_service_port,
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
                            "serviceName" => Field::Key_service_name,
                            "servicePort" => Field::Key_service_port,
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
                let mut value_service_name: Option<String> = None;
                let mut value_service_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource => value_resource = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_name => value_service_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_port => value_service_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressBackend {
                    resource: value_resource,
                    service_name: value_service_name,
                    service_port: value_service_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressBackend",
            &[
                "resource",
                "serviceName",
                "servicePort",
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
            self.service_name.as_ref().map_or(0, |_| 1) +
            self.service_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.resource {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        if let Some(value) = &self.service_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceName", value)?;
        }
        if let Some(value) = &self.service_port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "servicePort", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
