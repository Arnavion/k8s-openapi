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
                        Field::Key_service_name => value_service_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_port => value_service_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressBackend {
                    service_name: value_service_name.unwrap_or_default(),
                    service_port: value_service_port.unwrap_or_default(),
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressBackend {
    fn schema_name() -> String {
        "io.k8s.api.extensions.v1beta1.IngressBackend".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("IngressBackend describes all endpoints for a given service and port.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "serviceName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the name of the referenced service.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "servicePort".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the port of the referenced service.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: IntoIterator::into_iter([
                    "serviceName",
                    "servicePort",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
