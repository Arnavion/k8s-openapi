// Generated from definition io.k8s.api.authorization.v1.SubjectRulesReviewStatus

/// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubjectRulesReviewStatus {
    /// EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
    pub evaluation_error: Option<String>,

    /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
    pub incomplete: bool,

    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    pub non_resource_rules: Vec<::v1_12::api::authorization::v1::NonResourceRule>,

    /// ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    pub resource_rules: Vec<::v1_12::api::authorization::v1::ResourceRule>,
}

impl<'de> ::serde::Deserialize<'de> for SubjectRulesReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_evaluation_error,
            Key_incomplete,
            Key_non_resource_rules,
            Key_resource_rules,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubjectRulesReviewStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct SubjectRulesReviewStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_evaluation_error: Option<String> = None;
                let mut value_incomplete: Option<bool> = None;
                let mut value_non_resource_rules: Option<Vec<::v1_12::api::authorization::v1::NonResourceRule>> = None;
                let mut value_resource_rules: Option<Vec<::v1_12::api::authorization::v1::ResourceRule>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_evaluation_error => value_evaluation_error = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_incomplete => value_incomplete = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_non_resource_rules => value_non_resource_rules = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_resource_rules => value_resource_rules = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubjectRulesReviewStatus {
                    evaluation_error: value_evaluation_error,
                    incomplete: value_incomplete.ok_or_else(|| ::serde::de::Error::missing_field("incomplete"))?,
                    non_resource_rules: value_non_resource_rules.ok_or_else(|| ::serde::de::Error::missing_field("nonResourceRules"))?,
                    resource_rules: value_resource_rules.ok_or_else(|| ::serde::de::Error::missing_field("resourceRules"))?,
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

impl ::serde::Serialize for SubjectRulesReviewStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubjectRulesReviewStatus",
            0 +
            self.evaluation_error.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            1,
        )?;
        if let Some(value) = &self.evaluation_error {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "evaluationError", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "incomplete", &self.incomplete)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceRules", &self.non_resource_rules)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceRules", &self.resource_rules)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
