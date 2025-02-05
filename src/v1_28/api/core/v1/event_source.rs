// Generated from definition io.k8s.api.core.v1.EventSource

/// EventSource contains information for an event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventSource {
    /// Component from which the event is generated.
    pub component: Option<std::string::String>,

    /// Node name on which the event is generated.
    pub host: Option<std::string::String>,
}

impl crate::DeepMerge for EventSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.component, other.component);
        crate::DeepMerge::merge_from(&mut self.host, other.host);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EventSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_component,
            Key_host,
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
                            "component" => Field::Key_component,
                            "host" => Field::Key_host,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EventSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EventSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_component: Option<std::string::String> = None;
                let mut value_host: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_component => value_component = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host => value_host = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EventSource {
                    component: value_component,
                    host: value_host,
                })
            }
        }

        deserializer.deserialize_struct(
            "EventSource",
            &[
                "component",
                "host",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EventSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EventSource",
            self.component.as_ref().map_or(0, |_| 1) +
            self.host.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.component {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "component", value)?;
        }
        if let Some(value) = &self.host {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EventSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.EventSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("EventSource contains information for an event.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "component".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Component from which the event is generated.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "host".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Node name on which the event is generated.".into()),
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
