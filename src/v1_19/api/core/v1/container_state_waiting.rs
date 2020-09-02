// Generated from definition io.k8s.api.core.v1.ContainerStateWaiting

/// ContainerStateWaiting is a waiting state of a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStateWaiting {
    /// Message regarding why the container is not yet running.
    pub message: Option<String>,

    /// (brief) reason the container is not yet running.
    pub reason: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ContainerStateWaiting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_message,
            Key_reason,
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
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStateWaiting;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStateWaiting")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStateWaiting {
                    message: value_message,
                    reason: value_reason,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStateWaiting",
            &[
                "message",
                "reason",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ContainerStateWaiting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStateWaiting",
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
