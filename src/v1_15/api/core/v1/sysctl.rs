// Generated from definition io.k8s.api.core.v1.Sysctl

/// Sysctl defines a kernel parameter to be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sysctl {
    /// Name of a property to set
    pub name: String,

    /// Value of a property to set
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for Sysctl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_value,
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
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Sysctl;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Sysctl")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Sysctl {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    value: value_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Sysctl",
            &[
                "name",
                "value",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Sysctl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Sysctl",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        serde::ser::SerializeStruct::end(state)
    }
}
