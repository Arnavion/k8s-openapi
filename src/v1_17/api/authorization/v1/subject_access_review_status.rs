// Generated from definition io.k8s.api.authorization.v1.SubjectAccessReviewStatus

/// SubjectAccessReviewStatus
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    pub allowed: bool,

    /// Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
    pub denied: Option<bool>,

    /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
    pub evaluation_error: Option<String>,

    /// Reason is optional.  It indicates why a request was allowed or denied.
    pub reason: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SubjectAccessReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed,
            Key_denied,
            Key_evaluation_error,
            Key_reason,
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
                            "allowed" => Field::Key_allowed,
                            "denied" => Field::Key_denied,
                            "evaluationError" => Field::Key_evaluation_error,
                            "reason" => Field::Key_reason,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubjectAccessReviewStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SubjectAccessReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allowed: Option<bool> = None;
                let mut value_denied: Option<bool> = None;
                let mut value_evaluation_error: Option<String> = None;
                let mut value_reason: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed => value_allowed = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_denied => value_denied = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_evaluation_error => value_evaluation_error = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectAccessReviewStatus {
                    allowed: value_allowed.ok_or_else(|| serde::de::Error::missing_field("allowed"))?,
                    denied: value_denied,
                    evaluation_error: value_evaluation_error,
                    reason: value_reason,
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectAccessReviewStatus",
            &[
                "allowed",
                "denied",
                "evaluationError",
                "reason",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SubjectAccessReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectAccessReviewStatus",
            1 +
            self.denied.as_ref().map_or(0, |_| 1) +
            self.evaluation_error.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowed", &self.allowed)?;
        if let Some(value) = &self.denied {
            serde::ser::SerializeStruct::serialize_field(&mut state, "denied", value)?;
        }
        if let Some(value) = &self.evaluation_error {
            serde::ser::SerializeStruct::serialize_field(&mut state, "evaluationError", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
