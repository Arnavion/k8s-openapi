// Generated from definition io.k8s.api.flowcontrol.v1beta2.FlowSchemaSpec

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowSchemaSpec {
    /// `distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
    pub distinguisher_method: Option<crate::api::flowcontrol::v1beta2::FlowDistinguisherMethod>,

    /// `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in \[1,10000\]. Note that if the precedence is not specified, it will be set to 1000 as default.
    pub matching_precedence: Option<i32>,

    /// `priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.
    pub priority_level_configuration: crate::api::flowcontrol::v1beta2::PriorityLevelConfigurationReference,

    /// `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.
    pub rules: Option<Vec<crate::api::flowcontrol::v1beta2::PolicyRulesWithSubjects>>,
}

impl crate::DeepMerge for FlowSchemaSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.distinguisher_method, other.distinguisher_method);
        crate::DeepMerge::merge_from(&mut self.matching_precedence, other.matching_precedence);
        crate::DeepMerge::merge_from(&mut self.priority_level_configuration, other.priority_level_configuration);
        crate::merge_strategies::list::atomic(&mut self.rules, other.rules);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FlowSchemaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_distinguisher_method,
            Key_matching_precedence,
            Key_priority_level_configuration,
            Key_rules,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlowSchemaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowSchemaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_distinguisher_method: Option<crate::api::flowcontrol::v1beta2::FlowDistinguisherMethod> = None;
                let mut value_matching_precedence: Option<i32> = None;
                let mut value_priority_level_configuration: Option<crate::api::flowcontrol::v1beta2::PriorityLevelConfigurationReference> = None;
                let mut value_rules: Option<Vec<crate::api::flowcontrol::v1beta2::PolicyRulesWithSubjects>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_distinguisher_method => value_distinguisher_method = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_matching_precedence => value_matching_precedence = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_level_configuration => value_priority_level_configuration = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowSchemaSpec {
                    distinguisher_method: value_distinguisher_method,
                    matching_precedence: value_matching_precedence,
                    priority_level_configuration: value_priority_level_configuration.unwrap_or_default(),
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

impl crate::serde::Serialize for FlowSchemaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowSchemaSpec",
            1 +
            self.distinguisher_method.as_ref().map_or(0, |_| 1) +
            self.matching_precedence.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.distinguisher_method {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "distinguisherMethod", value)?;
        }
        if let Some(value) = &self.matching_precedence {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchingPrecedence", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priorityLevelConfiguration", &self.priority_level_configuration)?;
        if let Some(value) = &self.rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FlowSchemaSpec {
    fn schema_name() -> String {
        "io.k8s.api.flowcontrol.v1beta2.FlowSchemaSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FlowSchemaSpec describes how the FlowSchema's specification looks like.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "distinguisherMethod".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1beta2::FlowDistinguisherMethod>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "matchingPrecedence".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in [1,10000]. Note that if the precedence is not specified, it will be set to 1000 as default.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priorityLevelConfiguration".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1beta2::PriorityLevelConfigurationReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "rules".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::flowcontrol::v1beta2::PolicyRulesWithSubjects>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "priorityLevelConfiguration".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
