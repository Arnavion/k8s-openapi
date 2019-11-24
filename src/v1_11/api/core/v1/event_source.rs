// Generated from definition io.k8s.api.core.v1.EventSource

/// EventSource contains information for an event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventSource {
    /// Component from which the event is generated.
    pub component: Option<String>,

    /// Node name on which the event is generated.
    pub host: Option<String>,
}

impl<'de> serde::Deserialize<'de> for EventSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_component,
            Key_host,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EventSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EventSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_component: Option<String> = None;
                let mut value_host: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_component => value_component = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host => value_host = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for EventSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EventSource",
            self.component.as_ref().map_or(0, |_| 1) +
            self.host.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.component {
            serde::ser::SerializeStruct::serialize_field(&mut state, "component", value)?;
        }
        if let Some(value) = &self.host {
            serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
