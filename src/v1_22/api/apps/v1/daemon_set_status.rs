// Generated from definition io.k8s.api.apps.v1.DaemonSetStatus

/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetStatus {
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a DaemonSet's current state.
    pub conditions: Option<Vec<crate::api::apps::v1::DaemonSetCondition>>,

    /// The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    pub current_number_scheduled: i32,

    /// The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    pub desired_number_scheduled: i32,

    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
    pub number_available: Option<i32>,

    /// The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    pub number_misscheduled: i32,

    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready.
    pub number_ready: i32,

    /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
    pub number_unavailable: Option<i32>,

    /// The most recent generation observed by the daemon set controller.
    pub observed_generation: Option<i64>,

    /// The total number of nodes that are running updated daemon pod
    pub updated_number_scheduled: Option<i32>,
}

impl crate::DeepMerge for DaemonSetStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.collision_count, other.collision_count);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.current_number_scheduled, other.current_number_scheduled);
        crate::DeepMerge::merge_from(&mut self.desired_number_scheduled, other.desired_number_scheduled);
        crate::DeepMerge::merge_from(&mut self.number_available, other.number_available);
        crate::DeepMerge::merge_from(&mut self.number_misscheduled, other.number_misscheduled);
        crate::DeepMerge::merge_from(&mut self.number_ready, other.number_ready);
        crate::DeepMerge::merge_from(&mut self.number_unavailable, other.number_unavailable);
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::DeepMerge::merge_from(&mut self.updated_number_scheduled, other.updated_number_scheduled);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DaemonSetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_collision_count,
            Key_conditions,
            Key_current_number_scheduled,
            Key_desired_number_scheduled,
            Key_number_available,
            Key_number_misscheduled,
            Key_number_ready,
            Key_number_unavailable,
            Key_observed_generation,
            Key_updated_number_scheduled,
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
                            "collisionCount" => Field::Key_collision_count,
                            "conditions" => Field::Key_conditions,
                            "currentNumberScheduled" => Field::Key_current_number_scheduled,
                            "desiredNumberScheduled" => Field::Key_desired_number_scheduled,
                            "numberAvailable" => Field::Key_number_available,
                            "numberMisscheduled" => Field::Key_number_misscheduled,
                            "numberReady" => Field::Key_number_ready,
                            "numberUnavailable" => Field::Key_number_unavailable,
                            "observedGeneration" => Field::Key_observed_generation,
                            "updatedNumberScheduled" => Field::Key_updated_number_scheduled,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DaemonSetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<Vec<crate::api::apps::v1::DaemonSetCondition>> = None;
                let mut value_current_number_scheduled: Option<i32> = None;
                let mut value_desired_number_scheduled: Option<i32> = None;
                let mut value_number_available: Option<i32> = None;
                let mut value_number_misscheduled: Option<i32> = None;
                let mut value_number_ready: Option<i32> = None;
                let mut value_number_unavailable: Option<i32> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_updated_number_scheduled: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_collision_count => value_collision_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_number_scheduled => value_current_number_scheduled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_desired_number_scheduled => value_desired_number_scheduled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number_available => value_number_available = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number_misscheduled => value_number_misscheduled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number_ready => value_number_ready = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number_unavailable => value_number_unavailable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_number_scheduled => value_updated_number_scheduled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetStatus {
                    collision_count: value_collision_count,
                    conditions: value_conditions,
                    current_number_scheduled: value_current_number_scheduled.unwrap_or_default(),
                    desired_number_scheduled: value_desired_number_scheduled.unwrap_or_default(),
                    number_available: value_number_available,
                    number_misscheduled: value_number_misscheduled.unwrap_or_default(),
                    number_ready: value_number_ready.unwrap_or_default(),
                    number_unavailable: value_number_unavailable,
                    observed_generation: value_observed_generation,
                    updated_number_scheduled: value_updated_number_scheduled,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSetStatus",
            &[
                "collisionCount",
                "conditions",
                "currentNumberScheduled",
                "desiredNumberScheduled",
                "numberAvailable",
                "numberMisscheduled",
                "numberReady",
                "numberUnavailable",
                "observedGeneration",
                "updatedNumberScheduled",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DaemonSetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetStatus",
            4 +
            self.collision_count.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.number_available.as_ref().map_or(0, |_| 1) +
            self.number_unavailable.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.updated_number_scheduled.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.collision_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentNumberScheduled", &self.current_number_scheduled)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "desiredNumberScheduled", &self.desired_number_scheduled)?;
        if let Some(value) = &self.number_available {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "numberAvailable", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "numberMisscheduled", &self.number_misscheduled)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "numberReady", &self.number_ready)?;
        if let Some(value) = &self.number_unavailable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "numberUnavailable", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.updated_number_scheduled {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedNumberScheduled", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DaemonSetStatus {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.DaemonSetStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("DaemonSetStatus represents the current status of a daemon set.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "collisionCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents the latest available observations of a DaemonSet's current state.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apps::v1::DaemonSetCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentNumberScheduled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "desiredNumberScheduled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "numberAvailable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "numberMisscheduled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "numberReady".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "numberUnavailable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observedGeneration".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The most recent generation observed by the daemon set controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "updatedNumberScheduled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The total number of nodes that are running updated daemon pod".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "currentNumberScheduled".to_owned(),
                    "desiredNumberScheduled".to_owned(),
                    "numberMisscheduled".to_owned(),
                    "numberReady".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
