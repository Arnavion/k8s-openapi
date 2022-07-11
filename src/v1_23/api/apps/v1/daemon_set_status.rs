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

    /// numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running with a Ready Condition.
    pub number_ready: i32,

    /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
    pub number_unavailable: Option<i32>,

    /// The most recent generation observed by the daemon set controller.
    pub observed_generation: Option<i64>,

    /// The total number of nodes that are running updated daemon pod
    pub updated_number_scheduled: Option<i32>,

}

#[cfg(feature = "dsl")]
impl DaemonSetStatus  {
    /// Set [`Self::collision_count`]
    pub  fn collision_count_set(&mut self, collision_count: impl Into<Option<i32>>) -> &mut Self {
        self.collision_count = collision_count.into(); self
    }

    pub  fn collision_count(&mut self) -> &mut i32 {
        if self.collision_count.is_none() { self.collision_count = Some(Default::default()) }
        self.collision_count.as_mut().unwrap()
    }

    /// Modify [`Self::collision_count`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn collision_count_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.collision_count.is_none() { self.collision_count = Some(Default::default()) };
        func(self.collision_count.as_mut().unwrap()); self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::apps::v1::DaemonSetCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::apps::v1::DaemonSetCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apps::v1::DaemonSetCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::DaemonSetCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apps::v1::DaemonSetCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::current_number_scheduled`]
    pub  fn current_number_scheduled_set(&mut self, current_number_scheduled: impl Into<i32>) -> &mut Self {
        self.current_number_scheduled = current_number_scheduled.into(); self
    }

    pub  fn current_number_scheduled(&mut self) -> &mut i32 {
        &mut self.current_number_scheduled
    }

    /// Modify [`Self::current_number_scheduled`] with a `func`
    pub  fn current_number_scheduled_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.current_number_scheduled); self
    }


    /// Set [`Self::desired_number_scheduled`]
    pub  fn desired_number_scheduled_set(&mut self, desired_number_scheduled: impl Into<i32>) -> &mut Self {
        self.desired_number_scheduled = desired_number_scheduled.into(); self
    }

    pub  fn desired_number_scheduled(&mut self) -> &mut i32 {
        &mut self.desired_number_scheduled
    }

    /// Modify [`Self::desired_number_scheduled`] with a `func`
    pub  fn desired_number_scheduled_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.desired_number_scheduled); self
    }


    /// Set [`Self::number_available`]
    pub  fn number_available_set(&mut self, number_available: impl Into<Option<i32>>) -> &mut Self {
        self.number_available = number_available.into(); self
    }

    pub  fn number_available(&mut self) -> &mut i32 {
        if self.number_available.is_none() { self.number_available = Some(Default::default()) }
        self.number_available.as_mut().unwrap()
    }

    /// Modify [`Self::number_available`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn number_available_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.number_available.is_none() { self.number_available = Some(Default::default()) };
        func(self.number_available.as_mut().unwrap()); self
    }


    /// Set [`Self::number_misscheduled`]
    pub  fn number_misscheduled_set(&mut self, number_misscheduled: impl Into<i32>) -> &mut Self {
        self.number_misscheduled = number_misscheduled.into(); self
    }

    pub  fn number_misscheduled(&mut self) -> &mut i32 {
        &mut self.number_misscheduled
    }

    /// Modify [`Self::number_misscheduled`] with a `func`
    pub  fn number_misscheduled_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.number_misscheduled); self
    }


    /// Set [`Self::number_ready`]
    pub  fn number_ready_set(&mut self, number_ready: impl Into<i32>) -> &mut Self {
        self.number_ready = number_ready.into(); self
    }

    pub  fn number_ready(&mut self) -> &mut i32 {
        &mut self.number_ready
    }

    /// Modify [`Self::number_ready`] with a `func`
    pub  fn number_ready_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.number_ready); self
    }


    /// Set [`Self::number_unavailable`]
    pub  fn number_unavailable_set(&mut self, number_unavailable: impl Into<Option<i32>>) -> &mut Self {
        self.number_unavailable = number_unavailable.into(); self
    }

    pub  fn number_unavailable(&mut self) -> &mut i32 {
        if self.number_unavailable.is_none() { self.number_unavailable = Some(Default::default()) }
        self.number_unavailable.as_mut().unwrap()
    }

    /// Modify [`Self::number_unavailable`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn number_unavailable_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.number_unavailable.is_none() { self.number_unavailable = Some(Default::default()) };
        func(self.number_unavailable.as_mut().unwrap()); self
    }


    /// Set [`Self::observed_generation`]
    pub  fn observed_generation_set(&mut self, observed_generation: impl Into<Option<i64>>) -> &mut Self {
        self.observed_generation = observed_generation.into(); self
    }

    pub  fn observed_generation(&mut self) -> &mut i64 {
        if self.observed_generation.is_none() { self.observed_generation = Some(Default::default()) }
        self.observed_generation.as_mut().unwrap()
    }

    /// Modify [`Self::observed_generation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn observed_generation_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.observed_generation.is_none() { self.observed_generation = Some(Default::default()) };
        func(self.observed_generation.as_mut().unwrap()); self
    }


    /// Set [`Self::updated_number_scheduled`]
    pub  fn updated_number_scheduled_set(&mut self, updated_number_scheduled: impl Into<Option<i32>>) -> &mut Self {
        self.updated_number_scheduled = updated_number_scheduled.into(); self
    }

    pub  fn updated_number_scheduled(&mut self) -> &mut i32 {
        if self.updated_number_scheduled.is_none() { self.updated_number_scheduled = Some(Default::default()) }
        self.updated_number_scheduled.as_mut().unwrap()
    }

    /// Modify [`Self::updated_number_scheduled`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn updated_number_scheduled_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.updated_number_scheduled.is_none() { self.updated_number_scheduled = Some(Default::default()) };
        func(self.updated_number_scheduled.as_mut().unwrap()); self
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
                                description: Some("numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running with a Ready Condition.".to_owned()),
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
