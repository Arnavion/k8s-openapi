// Generated from definition io.k8s.api.core.v1.ContainerRestartRule

/// ContainerRestartRule describes how a container exit is handled.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerRestartRule {
    /// Specifies the action taken on a container exit if the requirements are satisfied. The only possible value is "Restart" to restart the container.
    pub action: std::string::String,

    /// Represents the exit codes to check on container exits.
    pub exit_codes: Option<crate::api::core::v1::ContainerRestartRuleOnExitCodes>,
}

impl crate::DeepMerge for ContainerRestartRule {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.action, other.action);
        crate::DeepMerge::merge_from(&mut self.exit_codes, other.exit_codes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerRestartRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action,
            Key_exit_codes,
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
                            "action" => Field::Key_action,
                            "exitCodes" => Field::Key_exit_codes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerRestartRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerRestartRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_action: Option<std::string::String> = None;
                let mut value_exit_codes: Option<crate::api::core::v1::ContainerRestartRuleOnExitCodes> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action => value_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exit_codes => value_exit_codes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerRestartRule {
                    action: value_action.unwrap_or_default(),
                    exit_codes: value_exit_codes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerRestartRule",
            &[
                "action",
                "exitCodes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerRestartRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerRestartRule",
            1 +
            self.exit_codes.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "action", &self.action)?;
        if let Some(value) = &self.exit_codes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exitCodes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerRestartRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerRestartRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerRestartRule describes how a container exit is handled.",
            "type": "object",
            "properties": {
                "action": {
                    "description": "Specifies the action taken on a container exit if the requirements are satisfied. The only possible value is \"Restart\" to restart the container.",
                    "type": "string",
                },
                "exitCodes": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerRestartRuleOnExitCodes>();
                    schema_obj.ensure_object().insert("description".into(), "Represents the exit codes to check on container exits.".into());
                    schema_obj
                }),
            },
            "required": [
                "action",
            ],
        })
    }
}
