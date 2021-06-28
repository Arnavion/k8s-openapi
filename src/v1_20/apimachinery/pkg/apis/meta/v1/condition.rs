// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Condition

/// Condition contains details for one aspect of the current state of this API Resource.
#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    pub last_transition_time: crate::apimachinery::pkg::apis::meta::v1::Time,

    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,

    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions\[x\].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    pub observed_generation: Option<i64>,

    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,

    /// status of the condition, one of True, False, Unknown.
    pub status: String,

    /// type of condition in CamelCase or in foo.example.com/CamelCase.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for Condition {
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            type Value = Condition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Condition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_reason: Option<String> = None;
                let mut value_status: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_transition_time => value_last_transition_time = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_message => value_message = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status => value_status = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Condition {
                    last_transition_time: value_last_transition_time.ok_or_else(|| crate::serde::de::Error::missing_field("lastTransitionTime"))?,
                    message: value_message.ok_or_else(|| crate::serde::de::Error::missing_field("message"))?,
                    observed_generation: value_observed_generation,
                    reason: value_reason.ok_or_else(|| crate::serde::de::Error::missing_field("reason"))?,
                    status: value_status.ok_or_else(|| crate::serde::de::Error::missing_field("status"))?,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Condition",
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

impl crate::serde::Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Condition",
            5 +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", &self.last_transition_time)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for Condition {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Condition contains details for one aspect of the current state of this API Resource.",
          "properties": {
            "lastTransitionTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable."),
            "message": {
              "description": "message is a human readable message indicating details about the transition. This may be an empty string.",
              "type": "string"
            },
            "observedGeneration": {
              "description": "observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.",
              "format": "int64",
              "type": "integer"
            },
            "reason": {
              "description": "reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.",
              "type": "string"
            },
            "status": {
              "description": "status of the condition, one of True, False, Unknown.",
              "type": "string"
            },
            "type": {
              "description": "type of condition in CamelCase or in foo.example.com/CamelCase.",
              "type": "string"
            }
          },
          "required": [
            "lastTransitionTime",
            "message",
            "reason",
            "status",
            "type"
          ],
          "type": "object"
        })
    }
}
