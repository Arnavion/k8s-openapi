// Generated from definition io.k8s.api.flowcontrol.v1beta1.FlowSchemaSpec

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowSchemaSpec {
    /// `distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
    pub distinguisher_method: Option<crate::api::flowcontrol::v1beta1::FlowDistinguisherMethod>,

    /// `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in \[1,10000\]. Note that if the precedence is not specified, it will be set to 1000 as default.
    pub matching_precedence: Option<i32>,

    /// `priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.
    pub priority_level_configuration: crate::api::flowcontrol::v1beta1::PriorityLevelConfigurationReference,

    /// `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.
    pub rules: Option<Vec<crate::api::flowcontrol::v1beta1::PolicyRulesWithSubjects>>,
}

impl<'de> serde::Deserialize<'de> for FlowSchemaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_distinguisher_method,
            Key_matching_precedence,
            Key_priority_level_configuration,
            Key_rules,
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
                            "distinguisherMethod" => Field::Key_distinguisher_method,
                            "matchingPrecedence" => Field::Key_matching_precedence,
                            "priorityLevelConfiguration" => Field::Key_priority_level_configuration,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FlowSchemaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowSchemaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_distinguisher_method: Option<crate::api::flowcontrol::v1beta1::FlowDistinguisherMethod> = None;
                let mut value_matching_precedence: Option<i32> = None;
                let mut value_priority_level_configuration: Option<crate::api::flowcontrol::v1beta1::PriorityLevelConfigurationReference> = None;
                let mut value_rules: Option<Vec<crate::api::flowcontrol::v1beta1::PolicyRulesWithSubjects>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_distinguisher_method => value_distinguisher_method = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_matching_precedence => value_matching_precedence = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_level_configuration => value_priority_level_configuration = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rules => value_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowSchemaSpec {
                    distinguisher_method: value_distinguisher_method,
                    matching_precedence: value_matching_precedence,
                    priority_level_configuration: value_priority_level_configuration.ok_or_else(|| serde::de::Error::missing_field("priorityLevelConfiguration"))?,
                    rules: value_rules,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlowSchemaSpec",
            &[
                "distinguisherMethod",
                "matchingPrecedence",
                "priorityLevelConfiguration",
                "rules",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for FlowSchemaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowSchemaSpec",
            1 +
            self.distinguisher_method.as_ref().map_or(0, |_| 1) +
            self.matching_precedence.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.distinguisher_method {
            serde::ser::SerializeStruct::serialize_field(&mut state, "distinguisherMethod", value)?;
        }
        if let Some(value) = &self.matching_precedence {
            serde::ser::SerializeStruct::serialize_field(&mut state, "matchingPrecedence", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "priorityLevelConfiguration", &self.priority_level_configuration)?;
        if let Some(value) = &self.rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
