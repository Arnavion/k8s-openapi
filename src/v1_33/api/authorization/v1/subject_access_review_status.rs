// Generated from definition io.k8s.api.authorization.v1.SubjectAccessReviewStatus

/// SubjectAccessReviewStatus
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    pub allowed: bool,

    /// Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
    pub denied: Option<bool>,

    /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
    pub evaluation_error: Option<std::string::String>,

    /// Reason is optional.  It indicates why a request was allowed or denied.
    pub reason: Option<std::string::String>,
}

impl crate::DeepMerge for SubjectAccessReviewStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allowed, other.allowed);
        crate::DeepMerge::merge_from(&mut self.denied, other.denied);
        crate::DeepMerge::merge_from(&mut self.evaluation_error, other.evaluation_error);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SubjectAccessReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed,
            Key_denied,
            Key_evaluation_error,
            Key_reason,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SubjectAccessReviewStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SubjectAccessReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allowed: Option<bool> = None;
                let mut value_denied: Option<bool> = None;
                let mut value_evaluation_error: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed => value_allowed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_denied => value_denied = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_evaluation_error => value_evaluation_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectAccessReviewStatus {
                    allowed: value_allowed.unwrap_or_default(),
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

impl crate::serde::Serialize for SubjectAccessReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectAccessReviewStatus",
            1 +
            self.denied.as_ref().map_or(0, |_| 1) +
            self.evaluation_error.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowed", &self.allowed)?;
        if let Some(value) = &self.denied {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "denied", value)?;
        }
        if let Some(value) = &self.evaluation_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "evaluationError", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SubjectAccessReviewStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.authorization.v1.SubjectAccessReviewStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SubjectAccessReviewStatus".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowed".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Allowed is required. True if the action would be allowed, false otherwise.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "denied".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "evaluationError".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Reason is optional.  It indicates why a request was allowed or denied.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "allowed".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
