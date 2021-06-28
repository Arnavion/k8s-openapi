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

impl<'de> crate::serde::Deserialize<'de> for EnvVar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_value,
            Key_value_from,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EnvVar;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EnvVar")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;
                let mut value_value_from: Option<crate::api::core::v1::EnvVarSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value_from => value_value_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EnvVar {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
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

impl crate::serde::Serialize for EnvVar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvVar",
            1 +
            self.value.as_ref().map_or(0, |_| 1) +
            self.value_from.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        if let Some(value) = &self.value_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "valueFrom", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for EnvVar {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "EnvVar represents an environment variable present in a Container.",
          "properties": {
            "name": {
              "description": "Name of the environment variable. Must be a C_IDENTIFIER.",
              "type": "string"
            },
            "value": {
              "description": "Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\".",
              "type": "string"
            },
            "valueFrom": crate::schema_ref_with_description(crate::api::core::v1::EnvVarSource::schema(), "Source for the environment variable's value. Cannot be used if value is not empty.")
          },
          "required": [
            "name"
          ],
          "type": "object"
        })
    }
}
