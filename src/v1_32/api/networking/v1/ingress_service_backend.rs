// Generated from definition io.k8s.api.networking.v1.IngressServiceBackend

/// IngressServiceBackend references a Kubernetes Service as a Backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressServiceBackend {
    /// name is the referenced service. The service must exist in the same namespace as the Ingress object.
    pub name: std::string::String,

    /// port of the referenced service. A port name or port number is required for a IngressServiceBackend.
    pub port: Option<crate::api::networking::v1::ServiceBackendPort>,
}

impl crate::DeepMerge for IngressServiceBackend {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.port, other.port);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IngressServiceBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_port,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressServiceBackend;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IngressServiceBackend")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_port: Option<crate::api::networking::v1::ServiceBackendPort> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressServiceBackend {
                    name: value_name.unwrap_or_default(),
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

impl crate::serde::Serialize for IngressServiceBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressServiceBackend",
            1 +
            self.port.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressServiceBackend {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IngressServiceBackend".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IngressServiceBackend references a Kubernetes Service as a Backend.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "name is the referenced service. The service must exist in the same namespace as the Ingress object.",
                    "type": "string",
                },
                "port": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::ServiceBackendPort>();
                    schema_obj.ensure_object().insert("description".into(), "port of the referenced service. A port name or port number is required for a IngressServiceBackend.".into());
                    schema_obj
                }),
            },
            "required": [
                "name",
            ],
        })
    }
}
