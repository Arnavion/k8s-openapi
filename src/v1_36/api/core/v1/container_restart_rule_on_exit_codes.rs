// Generated from definition io.k8s.api.core.v1.ContainerRestartRuleOnExitCodes

/// ContainerRestartRuleOnExitCodes describes the condition for handling an exited container based on its exit codes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerRestartRuleOnExitCodes {
    /// Represents the relationship between the container exit code(s) and the specified values. Possible values are: - In: the requirement is satisfied if the container exit code is in the
    ///   set of specified values.
    /// - NotIn: the requirement is satisfied if the container exit code is
    ///   not in the set of specified values.
    pub operator: std::string::String,

    /// Specifies the set of values to check for container exit codes. At most 255 elements are allowed.
    pub values: Option<std::vec::Vec<i32>>,
}

impl crate::DeepMerge for ContainerRestartRuleOnExitCodes {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.operator, other.operator);
        crate::merge_strategies::list::set(&mut self.values, other.values);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerRestartRuleOnExitCodes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_operator,
            Key_values,
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
                            "operator" => Field::Key_operator,
                            "values" => Field::Key_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerRestartRuleOnExitCodes;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerRestartRuleOnExitCodes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_operator: Option<std::string::String> = None;
                let mut value_values: Option<std::vec::Vec<i32>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerRestartRuleOnExitCodes {
                    operator: value_operator.unwrap_or_default(),
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerRestartRuleOnExitCodes",
            &[
                "operator",
                "values",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerRestartRuleOnExitCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerRestartRuleOnExitCodes",
            1 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        if let Some(value) = &self.values {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerRestartRuleOnExitCodes {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerRestartRuleOnExitCodes".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerRestartRuleOnExitCodes describes the condition for handling an exited container based on its exit codes.",
            "type": "object",
            "properties": {
                "operator": {
                    "description": "Represents the relationship between the container exit code(s) and the specified values. Possible values are: - In: the requirement is satisfied if the container exit code is in the\n  set of specified values.\n- NotIn: the requirement is satisfied if the container exit code is\n  not in the set of specified values.",
                    "type": "string",
                },
                "values": {
                    "description": "Specifies the set of values to check for container exit codes. At most 255 elements are allowed.",
                    "type": "array",
                    "items": {
                        "type": "integer",
                        "format": "int32",
                    },
                },
            },
            "required": [
                "operator",
            ],
        })
    }
}
