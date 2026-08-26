// Generated from definition io.k8s.api.scheduling.v1alpha3.CompositePodGroupStatus

/// CompositePodGroupStatus represents information about the status of a composite pod group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositePodGroupStatus {
    /// conditions represent the latest observations of the CompositePodGroup's state.
    ///
    /// Known condition types: - "CompositePodGroupInitiallyScheduled": Indicates whether the overall scheduling requirement
    ///   for the subtree under this CompositePodGroup has been satisfied. Once this condition
    ///   transitions to True, it serves as a terminal state and will never revert to False,
    ///   even if pods are subsequently deleted and group constraints are no longer met.
    /// - "DisruptionTarget": Indicates whether the CompositePodGroup is about to be terminated
    ///   due to disruption such as preemption.
    ///
    /// Known reasons for the CompositePodGroupInitiallyScheduled condition: - "Unschedulable": The CompositePodGroup's subtree could not be placed due to resource constraints,
    ///   affinity/anti-affinity, or topological constraints.
    /// - "SchedulerError": The CompositePodGroup cannot be scheduled due to some internal error
    ///   that occurred during scheduling.
    /// - "Invalid": Set to True when kube-scheduler detects an invalid group layout during
    ///   runtime validation. The `message` field details the specific layout violation (such as
    ///   a detected cycle, exceeding the maximum depth of 4, or referencing multiple distinct Workloads).
    ///
    /// Known reasons for the DisruptionTarget condition: - "PreemptionByScheduler": The CompositePodGroup was targeted by the scheduler's preemption loop
    ///   to free up capacity for higher-priority preemptors.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,
}

impl crate::DeepMerge for CompositePodGroupStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for CompositePodGroupStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
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
                            "conditions" => Field::Key_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CompositePodGroupStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CompositePodGroupStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CompositePodGroupStatus {
                    conditions: value_conditions,
                })
            }
        }

        deserializer.deserialize_struct(
            "CompositePodGroupStatus",
            &[
                "conditions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CompositePodGroupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CompositePodGroupStatus",
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CompositePodGroupStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CompositePodGroupStatus represents information about the status of a composite pod group.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "conditions represent the latest observations of the CompositePodGroup's state.\n\nKnown condition types: - \"CompositePodGroupInitiallyScheduled\": Indicates whether the overall scheduling requirement\n  for the subtree under this CompositePodGroup has been satisfied. Once this condition\n  transitions to True, it serves as a terminal state and will never revert to False,\n  even if pods are subsequently deleted and group constraints are no longer met.\n- \"DisruptionTarget\": Indicates whether the CompositePodGroup is about to be terminated\n  due to disruption such as preemption.\n\nKnown reasons for the CompositePodGroupInitiallyScheduled condition: - \"Unschedulable\": The CompositePodGroup's subtree could not be placed due to resource constraints,\n  affinity/anti-affinity, or topological constraints.\n- \"SchedulerError\": The CompositePodGroup cannot be scheduled due to some internal error\n  that occurred during scheduling.\n- \"Invalid\": Set to True when kube-scheduler detects an invalid group layout during\n  runtime validation. The `message` field details the specific layout violation (such as\n  a detected cycle, exceeding the maximum depth of 4, or referencing multiple distinct Workloads).\n\nKnown reasons for the DisruptionTarget condition: - \"PreemptionByScheduler\": The CompositePodGroup was targeted by the scheduler's preemption loop\n  to free up capacity for higher-priority preemptors.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CompositePodGroupStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CompositePodGroupStatus represents information about the status of a composite pod group.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("conditions represent the latest observations of the CompositePodGroup's state.\n\nKnown condition types: - \"CompositePodGroupInitiallyScheduled\": Indicates whether the overall scheduling requirement\n  for the subtree under this CompositePodGroup has been satisfied. Once this condition\n  transitions to True, it serves as a terminal state and will never revert to False,\n  even if pods are subsequently deleted and group constraints are no longer met.\n- \"DisruptionTarget\": Indicates whether the CompositePodGroup is about to be terminated\n  due to disruption such as preemption.\n\nKnown reasons for the CompositePodGroupInitiallyScheduled condition: - \"Unschedulable\": The CompositePodGroup's subtree could not be placed due to resource constraints,\n  affinity/anti-affinity, or topological constraints.\n- \"SchedulerError\": The CompositePodGroup cannot be scheduled due to some internal error\n  that occurred during scheduling.\n- \"Invalid\": Set to True when kube-scheduler detects an invalid group layout during\n  runtime validation. The `message` field details the specific layout violation (such as\n  a detected cycle, exceeding the maximum depth of 4, or referencing multiple distinct Workloads).\n\nKnown reasons for the DisruptionTarget condition: - \"PreemptionByScheduler\": The CompositePodGroup was targeted by the scheduler's preemption loop\n  to free up capacity for higher-priority preemptors.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
