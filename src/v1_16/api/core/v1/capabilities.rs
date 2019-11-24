// Generated from definition io.k8s.api.core.v1.Capabilities

/// Adds and removes POSIX capabilities from running containers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Capabilities {
    /// Added capabilities
    pub add: Option<Vec<String>>,

    /// Removed capabilities
    pub drop: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for Capabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_add,
            Key_drop,
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
                            "add" => Field::Key_add,
                            "drop" => Field::Key_drop,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Capabilities;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Capabilities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_add: Option<Vec<String>> = None;
                let mut value_drop: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_add => value_add = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_drop => value_drop = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Capabilities {
                    add: value_add,
                    drop: value_drop,
                })
            }
        }

        deserializer.deserialize_struct(
            "Capabilities",
            &[
                "add",
                "drop",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Capabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Capabilities",
            self.add.as_ref().map_or(0, |_| 1) +
            self.drop.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.add {
            serde::ser::SerializeStruct::serialize_field(&mut state, "add", value)?;
        }
        if let Some(value) = &self.drop {
            serde::ser::SerializeStruct::serialize_field(&mut state, "drop", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
