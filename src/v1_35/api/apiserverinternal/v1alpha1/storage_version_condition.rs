// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionCondition

/// Describes the state of the storageVersion at a certain point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionCondition {
    /// Last time the condition transitioned from one status to another.
    pub last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human readable message indicating details about the transition.
    pub message: std::string::String,

    /// If set, this represents the .metadata.generation that the condition was set based upon.
    pub observed_generation: Option<i64>,

    /// The reason for the condition's last transition.
    pub reason: std::string::String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: std::string::String,

    /// Type of the condition.
    pub type_: std::string::String,
}

impl crate::DeepMerge for StorageVersionCondition {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.last_transition_time, other.last_transition_time);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageVersionCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_transition_time,
            Key_message,
            Key_observed_generation,
            Key_reason,
            Key_status,
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
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            "message" => Field::Key_message,
                            "observedGeneration" => Field::Key_observed_generation,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
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
            type Value = StorageVersionCondition;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageVersionCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_status: Option<std::string::String> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_transition_time => value_last_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageVersionCondition {
                    last_transition_time: value_last_transition_time,
                    message: value_message.unwrap_or_default(),
                    observed_generation: value_observed_generation,
                    reason: value_reason.unwrap_or_default(),
                    status: value_status.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageVersionCondition",
            &[
                "lastTransitionTime",
                "message",
                "observedGeneration",
                "reason",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageVersionCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageVersionCondition",
            4 +
            self.last_transition_time.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.last_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", &self.message)?;
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", &self.reason)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageVersionCondition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apiserverinternal.v1alpha1.StorageVersionCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Describes the state of the storageVersion at a certain point.",
            "type": "object",
            "properties": {
                "lastTransitionTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "Last time the condition transitioned from one status to another.".into());
                    schema_obj
                }),
                "message": {
                    "description": "A human readable message indicating details about the transition.",
                    "type": "string",
                },
                "observedGeneration": {
                    "description": "If set, this represents the .metadata.generation that the condition was set based upon.",
                    "type": "integer",
                    "format": "int64",
                },
                "reason": {
                    "description": "The reason for the condition's last transition.",
                    "type": "string",
                },
                "status": {
                    "description": "Status of the condition, one of True, False, Unknown.",
                    "type": "string",
                },
                "type": {
                    "description": "Type of the condition.",
                    "type": "string",
                },
            },
            "required": [
                "message",
                "reason",
                "status",
                "type",
            ],
        })
    }
}
