// Generated from definition io.k8s.api.apps.v1.DeploymentStrategy

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentStrategy {
    /// Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
    pub rolling_update: Option<crate::api::apps::v1::RollingUpdateDeployment>,

    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    pub type_: Option<std::string::String>,
}

impl crate::DeepMerge for DeploymentStrategy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.rolling_update, other.rolling_update);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeploymentStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rolling_update,
            Key_type_,
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
                            "rollingUpdate" => Field::Key_rolling_update,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentStrategy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeploymentStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_rolling_update: Option<crate::api::apps::v1::RollingUpdateDeployment> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rolling_update => value_rolling_update = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentStrategy {
                    rolling_update: value_rolling_update,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentStrategy",
            &[
                "rollingUpdate",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeploymentStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentStrategy",
            self.rolling_update.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.rolling_update {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rollingUpdate", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeploymentStrategy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apps.v1.DeploymentStrategy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeploymentStrategy describes how to replace existing pods with new ones.",
            "type": "object",
            "properties": {
                "rollingUpdate": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::RollingUpdateDeployment>();
                    schema_obj.ensure_object().insert("description".into(), "Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.".into());
                    schema_obj
                }),
                "type": {
                    "description": "Type of deployment. Can be \"Recreate\" or \"RollingUpdate\". Default is RollingUpdate.",
                    "type": "string",
                },
            },
        })
    }
}
