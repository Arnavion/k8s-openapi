// Generated from definition io.k8s.api.batch.v1.PodFailurePolicyRule

/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met. One of OnExitCodes and onPodConditions, but not both, can be used in each rule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodFailurePolicyRule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied. Possible values are: - FailJob: indicates that the pod's job is marked as Failed and all
    ///   running pods are terminated.
    /// - Ignore: indicates that the counter towards the .backoffLimit is not
    ///   incremented and a replacement pod is created.
    /// - Count: indicates that the pod is handled in the default way - the
    ///   counter towards the .backoffLimit is incremented.
    /// Additional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.
    ///
    pub action: String,

    /// Represents the requirement on the container exit codes.
    pub on_exit_codes: Option<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement>,

    /// Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most 20 elements are allowed.
    pub on_pod_conditions: Vec<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>,
}

impl crate::DeepMerge for PodFailurePolicyRule {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.action, other.action);
        crate::DeepMerge::merge_from(&mut self.on_exit_codes, other.on_exit_codes);
        crate::merge_strategies::list::atomic(&mut self.on_pod_conditions, other.on_pod_conditions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodFailurePolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action,
            Key_on_exit_codes,
            Key_on_pod_conditions,
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
                            "action" => Field::Key_action,
                            "onExitCodes" => Field::Key_on_exit_codes,
                            "onPodConditions" => Field::Key_on_pod_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodFailurePolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodFailurePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_action: Option<String> = None;
                let mut value_on_exit_codes: Option<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement> = None;
                let mut value_on_pod_conditions: Option<Vec<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action => value_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_on_exit_codes => value_on_exit_codes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_on_pod_conditions => value_on_pod_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodFailurePolicyRule {
                    action: value_action.unwrap_or_default(),
                    on_exit_codes: value_on_exit_codes,
                    on_pod_conditions: value_on_pod_conditions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodFailurePolicyRule",
            &[
                "action",
                "onExitCodes",
                "onPodConditions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodFailurePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodFailurePolicyRule",
            2 +
            self.on_exit_codes.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "action", &self.action)?;
        if let Some(value) = &self.on_exit_codes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "onExitCodes", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "onPodConditions", &self.on_pod_conditions)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodFailurePolicyRule {
    fn schema_name() -> String {
        "io.k8s.api.batch.v1.PodFailurePolicyRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodFailurePolicyRule describes how a pod failure is handled when the requirements are met. One of OnExitCodes and onPodConditions, but not both, can be used in each rule.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "action".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the action taken on a pod failure when the requirements are satisfied. Possible values are: - FailJob: indicates that the pod's job is marked as Failed and all\n  running pods are terminated.\n- Ignore: indicates that the counter towards the .backoffLimit is not\n  incremented and a replacement pod is created.\n- Count: indicates that the pod is handled in the default way - the\n  counter towards the .backoffLimit is incremented.\nAdditional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "onExitCodes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents the requirement on the container exit codes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "onPodConditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most 20 elements are allowed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "action".to_owned(),
                    "onPodConditions".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
