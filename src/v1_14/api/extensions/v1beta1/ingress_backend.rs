// Generated from definition io.k8s.api.extensions.v1beta1.IngressBackend

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressBackend {
    /// Specifies the name of the referenced service.
    pub service_name: String,

    /// Specifies the port of the referenced service.
    pub service_port: crate::apimachinery::pkg::util::intstr::IntOrString,
}

impl<'de> crate::serde::Deserialize<'de> for IngressBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_service_name,
            Key_service_port,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressBackend;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressBackend")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_service_name: Option<String> = None;
                let mut value_service_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_service_name => value_service_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service_port => value_service_port = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressBackend {
                    service_name: value_service_name.ok_or_else(|| crate::serde::de::Error::missing_field("serviceName"))?,
                    service_port: value_service_port.ok_or_else(|| crate::serde::de::Error::missing_field("servicePort"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressBackend",
            &[
                "serviceName",
                "servicePort",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressBackend",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceName", &self.service_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "servicePort", &self.service_port)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for IngressBackend {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "IngressBackend describes all endpoints for a given service and port.",
          "properties": {
            "serviceName": {
              "description": "Specifies the name of the referenced service.",
              "type": "string"
            },
            "servicePort": crate::schema_ref_with_description(crate::apimachinery::pkg::util::intstr::IntOrString::schema(), "Specifies the port of the referenced service.")
          },
          "required": [
            "serviceName",
            "servicePort"
          ],
          "type": "object"
        })
    }
}
