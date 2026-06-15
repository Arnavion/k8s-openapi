// Generated from definition io.k8s.api.scheduling.v1alpha2.PodGroupStatus

/// PodGroupStatus represents information about the status of a pod group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupStatus {
    /// Conditions represent the latest observations of the PodGroup's state.
    ///
    /// Known condition types: - "PodGroupScheduled": Indicates whether the scheduling requirement has been satisfied. - "DisruptionTarget": Indicates whether the PodGroup is about to be terminated
    ///   due to disruption such as preemption.
    ///
    /// Known reasons for the PodGroupScheduled condition: - "Unschedulable": The PodGroup cannot be scheduled due to resource constraints,
    ///   affinity/anti-affinity rules, or insufficient capacity for the gang.
    /// - "SchedulerError": The PodGroup cannot be scheduled due to some internal error
    ///   that happened during scheduling, for example due to nodeAffinity parsing errors.
    ///
    /// Known reasons for the DisruptionTarget condition: - "PreemptionByScheduler": The PodGroup was preempted by the scheduler to make room for
    ///   higher-priority PodGroups or Pods.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// Status of resource claims.
    pub resource_claim_statuses: Option<std::vec::Vec<crate::api::scheduling::v1alpha2::PodGroupResourceClaimStatus>>,
}

impl crate::DeepMerge for PodGroupStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.resource_claim_statuses,
            other.resource_claim_statuses,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_resource_claim_statuses,
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
                            "resourceClaimStatuses" => Field::Key_resource_claim_statuses,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodGroupStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_resource_claim_statuses: Option<std::vec::Vec<crate::api::scheduling::v1alpha2::PodGroupResourceClaimStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_statuses => value_resource_claim_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupStatus {
                    conditions: value_conditions,
                    resource_claim_statuses: value_resource_claim_statuses,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupStatus",
            &[
                "conditions",
                "resourceClaimStatuses",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodGroupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodGroupStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.resource_claim_statuses.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.resource_claim_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimStatuses", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha2.PodGroupStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupStatus represents information about the status of a pod group.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "Conditions represent the latest observations of the PodGroup's state.\n\nKnown condition types: - \"PodGroupScheduled\": Indicates whether the scheduling requirement has been satisfied. - \"DisruptionTarget\": Indicates whether the PodGroup is about to be terminated\n  due to disruption such as preemption.\n\nKnown reasons for the PodGroupScheduled condition: - \"Unschedulable\": The PodGroup cannot be scheduled due to resource constraints,\n  affinity/anti-affinity rules, or insufficient capacity for the gang.\n- \"SchedulerError\": The PodGroup cannot be scheduled due to some internal error\n  that happened during scheduling, for example due to nodeAffinity parsing errors.\n\nKnown reasons for the DisruptionTarget condition: - \"PreemptionByScheduler\": The PodGroup was preempted by the scheduler to make room for\n  higher-priority PodGroups or Pods.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "resourceClaimStatuses": {
                    "description": "Status of resource claims.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupResourceClaimStatus>()),
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for PodGroupStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha2.PodGroupStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("PodGroupStatus represents information about the status of a pod group.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Conditions represent the latest observations of the PodGroup's state.\n\nKnown condition types: - \"PodGroupScheduled\": Indicates whether the scheduling requirement has been satisfied. - \"DisruptionTarget\": Indicates whether the PodGroup is about to be terminated\n  due to disruption such as preemption.\n\nKnown reasons for the PodGroupScheduled condition: - \"Unschedulable\": The PodGroup cannot be scheduled due to resource constraints,\n  affinity/anti-affinity rules, or insufficient capacity for the gang.\n- \"SchedulerError\": The PodGroup cannot be scheduled due to some internal error\n  that happened during scheduling, for example due to nodeAffinity parsing errors.\n\nKnown reasons for the DisruptionTarget condition: - \"PreemptionByScheduler\": The PodGroup was preempted by the scheduler to make room for\n  higher-priority PodGroups or Pods.".into()),
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
                    (
                        "resourceClaimStatuses".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Status of resource claims.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupResourceClaimStatus>()))),
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
