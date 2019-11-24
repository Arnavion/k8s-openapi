// Generated from definition io.k8s.api.core.v1.EnvVar

/// EnvVar represents an environment variable present in a Container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvVar {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,

    /// Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    pub value: Option<String>,

    /// Source for the environment variable's value. Cannot be used if value is not empty.
    pub value_from: Option<crate::api::core::v1::EnvVarSource>,
}

impl<'de> serde::Deserialize<'de> for EnvVar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_value,
            Key_value_from,
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
                            "valueFrom" => Field::Key_value_from,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnvVar;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EnvVar")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;
                let mut value_value_from: Option<crate::api::core::v1::EnvVarSource> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value_from => value_value_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EnvVar {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    value: value_value,
                    value_from: value_value_from,
                })
            }
        }

        deserializer.deserialize_struct(
            "EnvVar",
            &[
                "name",
                "value",
                "valueFrom",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EnvVar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvVar",
            1 +
            self.value.as_ref().map_or(0, |_| 1) +
            self.value_from.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.value {
            serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        if let Some(value) = &self.value_from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "valueFrom", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
