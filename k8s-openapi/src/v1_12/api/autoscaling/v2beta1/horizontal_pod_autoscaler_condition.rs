// Generated from definition io.k8s.api.autoscaling.v2beta1.HorizontalPodAutoscalerCondition

/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerCondition {
    /// lastTransitionTime is the last time the condition transitioned from one status to another
    pub last_transition_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// message is a human-readable explanation containing details about the transition
    pub message: Option<String>,

    /// reason is the reason for the condition's last transition.
    pub reason: Option<String>,

    /// status is the status of the condition (True, False, Unknown)
    pub status: String,

    /// type describes the current condition
    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for HorizontalPodAutoscalerCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_transition_time,
            Key_message,
            Key_reason,
            Key_status,
            Key_type_,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            "message" => Field::Key_message,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerCondition;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct HorizontalPodAutoscalerCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_last_transition_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_status: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_transition_time => value_last_transition_time = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerCondition {
                    last_transition_time: value_last_transition_time,
                    message: value_message,
                    reason: value_reason,
                    status: value_status.ok_or_else(|| ::serde::de::Error::missing_field("status"))?,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerCondition",
            &[
                "lastTransitionTime",
                "message",
                "reason",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for HorizontalPodAutoscalerCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerCondition",
            0 +
            self.last_transition_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            1 +
            1,
        )?;
        if let Some(value) = &self.last_transition_time {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        if let Some(value) = &self.message {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
