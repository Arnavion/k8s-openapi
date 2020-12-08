// Generated from definition io.k8s.api.networking.v1.ServiceBackendPort

/// ServiceBackendPort is the service port being referenced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceBackendPort {
    /// Name is the name of the port on the Service. This is a mutually exclusive setting with "Number".
    pub name: Option<String>,

    /// Number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with "Name".
    pub number: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for ServiceBackendPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_number,
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
                            "number" => Field::Key_number,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceBackendPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceBackendPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_number: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number => value_number = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceBackendPort {
                    name: value_name,
                    number: value_number,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceBackendPort",
            &[
                "name",
                "number",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceBackendPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceBackendPort",
            self.name.as_ref().map_or(0, |_| 1) +
            self.number.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.number {
            serde::ser::SerializeStruct::serialize_field(&mut state, "number", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
