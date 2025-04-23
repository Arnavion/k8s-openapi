// Generated from definition io.k8s.api.core.v1.SessionAffinityConfig

/// SessionAffinityConfig represents the configurations of session affinity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SessionAffinityConfig {
    /// clientIP contains the configurations of Client IP based session affinity.
    pub client_ip: Option<crate::api::core::v1::ClientIPConfig>,
}

impl crate::DeepMerge for SessionAffinityConfig {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.client_ip, other.client_ip);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SessionAffinityConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_ip,
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
                            "clientIP" => Field::Key_client_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SessionAffinityConfig;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SessionAffinityConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_client_ip: Option<crate::api::core::v1::ClientIPConfig> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_ip => value_client_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SessionAffinityConfig {
                    client_ip: value_client_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "SessionAffinityConfig",
            &[
                "clientIP",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SessionAffinityConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SessionAffinityConfig",
            self.client_ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.client_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientIP", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SessionAffinityConfig {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.SessionAffinityConfig".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SessionAffinityConfig represents the configurations of session affinity.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "clientIP".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ClientIPConfig>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("clientIP contains the configurations of Client IP based session affinity.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
