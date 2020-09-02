// Generated from definition io.k8s.api.networking.v1.IngressServiceBackend

/// IngressServiceBackend references a Kubernetes Service as a Backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressServiceBackend {
    /// Name is the referenced service. The service must exist in the same namespace as the Ingress object.
    pub name: String,

    /// Port of the referenced service. A port name or port number is required for a IngressServiceBackend.
    pub port: Option<crate::api::networking::v1::ServiceBackendPort>,
}

impl<'de> serde::Deserialize<'de> for IngressServiceBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_port,
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
                            "name" => Field::Key_name,
                            "port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngressServiceBackend;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressServiceBackend")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_port: Option<crate::api::networking::v1::ServiceBackendPort> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_port => value_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressServiceBackend {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    port: value_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressServiceBackend",
            &[
                "name",
                "port",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IngressServiceBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressServiceBackend",
            1 +
            self.port.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
