// Generated from definition io.k8s.api.certificates.v1.CertificateSigningRequestCondition

/// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestCondition {
    /// lastTransitionTime is the time the condition last transitioned from one status to another. If unset, when a new condition type is added or an existing condition's status is changed, the server defaults this to the current time.
    pub last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// lastUpdateTime is the time of the last update to this condition
    pub last_update_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// message contains a human readable message with details about the request state
    pub message: Option<String>,

    /// reason indicates a brief reason for the request state
    pub reason: Option<String>,

    /// status of the condition, one of True, False, Unknown. Approved, Denied, and Failed conditions may not be "False" or "Unknown".
    pub status: String,

    /// type of the condition. Known conditions are "Approved", "Denied", and "Failed".
    ///
    /// An "Approved" condition is added via the /approval subresource, indicating the request was approved and should be issued by the signer.
    ///
    /// A "Denied" condition is added via the /approval subresource, indicating the request was denied and should not be issued by the signer.
    ///
    /// A "Failed" condition is added via the /status subresource, indicating the signer failed to issue the certificate.
    ///
    /// Approved and Denied conditions are mutually exclusive. Approved, Denied, and Failed conditions cannot be removed once added.
    ///
    /// Only one condition of a given type is allowed.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for CertificateSigningRequestCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_transition_time,
            Key_last_update_time,
            Key_message,
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
                            "lastUpdateTime" => Field::Key_last_update_time,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestCondition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CertificateSigningRequestCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_last_update_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_status: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_transition_time => value_last_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_update_time => value_last_update_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestCondition {
                    last_transition_time: value_last_transition_time,
                    last_update_time: value_last_update_time,
                    message: value_message,
                    reason: value_reason,
                    status: value_status.ok_or_else(|| crate::serde::de::Error::missing_field("status"))?,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequestCondition",
            &[
                "lastTransitionTime",
                "lastUpdateTime",
                "message",
                "reason",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CertificateSigningRequestCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestCondition",
            2 +
            self.last_transition_time.as_ref().map_or(0, |_| 1) +
            self.last_update_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.last_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        if let Some(value) = &self.last_update_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastUpdateTime", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CertificateSigningRequestCondition {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object",
          "properties": {
            "lastTransitionTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "lastTransitionTime is the time the condition last transitioned from one status to another. If unset, when a new condition type is added or an existing condition's status is changed, the server defaults this to the current time."),
            "lastUpdateTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "lastUpdateTime is the time of the last update to this condition"),
            "message": {
              "description": "message contains a human readable message with details about the request state",
              "type": "string"
            },
            "reason": {
              "description": "reason indicates a brief reason for the request state",
              "type": "string"
            },
            "status": {
              "description": "status of the condition, one of True, False, Unknown. Approved, Denied, and Failed conditions may not be \"False\" or \"Unknown\".",
              "type": "string"
            },
            "type": {
              "description": "type of the condition. Known conditions are \"Approved\", \"Denied\", and \"Failed\".\n\nAn \"Approved\" condition is added via the /approval subresource, indicating the request was approved and should be issued by the signer.\n\nA \"Denied\" condition is added via the /approval subresource, indicating the request was denied and should not be issued by the signer.\n\nA \"Failed\" condition is added via the /status subresource, indicating the signer failed to issue the certificate.\n\nApproved and Denied conditions are mutually exclusive. Approved, Denied, and Failed conditions cannot be removed once added.\n\nOnly one condition of a given type is allowed.",
              "type": "string"
            }
          },
          "required": [
            "status",
            "type"
          ],
          "type": "object"
        })
    }
}
