// Generated from definition io.k8s.api.authorization.v1.SubjectRulesReviewStatus

/// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectRulesReviewStatus {
    /// EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
    pub evaluation_error: Option<std::string::String>,

    /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
    pub incomplete: bool,

    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    pub non_resource_rules: std::vec::Vec<crate::api::authorization::v1::NonResourceRule>,

    /// ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    pub resource_rules: std::vec::Vec<crate::api::authorization::v1::ResourceRule>,
}

impl crate::DeepMerge for SubjectRulesReviewStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.evaluation_error, other.evaluation_error);
        crate::DeepMerge::merge_from(&mut self.incomplete, other.incomplete);
        crate::merge_strategies::list::atomic(&mut self.non_resource_rules, other.non_resource_rules);
        crate::merge_strategies::list::atomic(&mut self.resource_rules, other.resource_rules);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SubjectRulesReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_evaluation_error,
            Key_incomplete,
            Key_non_resource_rules,
            Key_resource_rules,
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
                            "evaluationError" => Field::Key_evaluation_error,
                            "incomplete" => Field::Key_incomplete,
                            "nonResourceRules" => Field::Key_non_resource_rules,
                            "resourceRules" => Field::Key_resource_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SubjectRulesReviewStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SubjectRulesReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_evaluation_error: Option<std::string::String> = None;
                let mut value_incomplete: Option<bool> = None;
                let mut value_non_resource_rules: Option<std::vec::Vec<crate::api::authorization::v1::NonResourceRule>> = None;
                let mut value_resource_rules: Option<std::vec::Vec<crate::api::authorization::v1::ResourceRule>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_evaluation_error => value_evaluation_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_incomplete => value_incomplete = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_rules => value_non_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_rules => value_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectRulesReviewStatus {
                    evaluation_error: value_evaluation_error,
                    incomplete: value_incomplete.unwrap_or_default(),
                    non_resource_rules: value_non_resource_rules.unwrap_or_default(),
                    resource_rules: value_resource_rules.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "SubjectRulesReviewStatus",
            &[
                "evaluationError",
                "incomplete",
                "nonResourceRules",
                "resourceRules",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SubjectRulesReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectRulesReviewStatus",
            3 +
            self.evaluation_error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.evaluation_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "evaluationError", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "incomplete", &self.incomplete)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceRules", &self.non_resource_rules)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceRules", &self.resource_rules)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SubjectRulesReviewStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authorization.v1.SubjectRulesReviewStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.",
            "type": "object",
            "properties": {
                "evaluationError": {
                    "description": "EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.",
                    "type": "string",
                },
                "incomplete": {
                    "description": "Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.",
                    "type": "boolean",
                },
                "nonResourceRules": {
                    "description": "NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::authorization::v1::NonResourceRule>()),
                },
                "resourceRules": {
                    "description": "ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::authorization::v1::ResourceRule>()),
                },
            },
            "required": [
                "incomplete",
                "nonResourceRules",
                "resourceRules",
            ],
        })
    }
}
