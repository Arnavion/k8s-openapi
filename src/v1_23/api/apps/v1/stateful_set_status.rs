// Generated from definition io.k8s.api.apps.v1.StatefulSetStatus

/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset. This is a beta field and enabled/disabled by StatefulSetMinReadySeconds feature gate.
    pub available_replicas: i32,

    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a statefulset's current state.
    pub conditions: Option<Vec<crate::api::apps::v1::StatefulSetCondition>>,

    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
    pub current_replicas: Option<i32>,

    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[0,currentReplicas).
    pub current_revision: Option<String>,

    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
    pub observed_generation: Option<i64>,

    /// readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
    pub ready_replicas: Option<i32>,

    /// replicas is the number of Pods created by the StatefulSet controller.
    pub replicas: i32,

    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[replicas-updatedReplicas,replicas)
    pub update_revision: Option<String>,

    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
    pub updated_replicas: Option<i32>,

}

#[cfg(feature = "dsl")]
impl StatefulSetStatus  {
    /// Set [`Self::available_replicas`]
    pub  fn available_replicas_set(&mut self, available_replicas: impl Into<i32>) -> &mut Self {
        self.available_replicas = available_replicas.into(); self
    }

    pub  fn available_replicas(&mut self) -> &mut i32 {
        &mut self.available_replicas
    }

    /// Modify [`Self::available_replicas`] with a `func`
    pub  fn available_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.available_replicas); self
    }


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
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::apps::v1::StatefulSetCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::apps::v1::StatefulSetCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apps::v1::StatefulSetCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::StatefulSetCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apps::v1::StatefulSetCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::current_replicas`]
    pub  fn current_replicas_set(&mut self, current_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.current_replicas = current_replicas.into(); self
    }

    pub  fn current_replicas(&mut self) -> &mut i32 {
        if self.current_replicas.is_none() { self.current_replicas = Some(Default::default()) }
        self.current_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::current_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn current_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.current_replicas.is_none() { self.current_replicas = Some(Default::default()) };
        func(self.current_replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::current_revision`]
    pub  fn current_revision_set(&mut self, current_revision: impl Into<Option<String>>) -> &mut Self {
        self.current_revision = current_revision.into(); self
    }

    pub  fn current_revision(&mut self) -> &mut String {
        if self.current_revision.is_none() { self.current_revision = Some(Default::default()) }
        self.current_revision.as_mut().unwrap()
    }

    /// Modify [`Self::current_revision`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn current_revision_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.current_revision.is_none() { self.current_revision = Some(Default::default()) };
        func(self.current_revision.as_mut().unwrap()); self
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


    /// Set [`Self::ready_replicas`]
    pub  fn ready_replicas_set(&mut self, ready_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.ready_replicas = ready_replicas.into(); self
    }

    pub  fn ready_replicas(&mut self) -> &mut i32 {
        if self.ready_replicas.is_none() { self.ready_replicas = Some(Default::default()) }
        self.ready_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::ready_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ready_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.ready_replicas.is_none() { self.ready_replicas = Some(Default::default()) };
        func(self.ready_replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::replicas`]
    pub  fn replicas_set(&mut self, replicas: impl Into<i32>) -> &mut Self {
        self.replicas = replicas.into(); self
    }

    pub  fn replicas(&mut self) -> &mut i32 {
        &mut self.replicas
    }

    /// Modify [`Self::replicas`] with a `func`
    pub  fn replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.replicas); self
    }


    /// Set [`Self::update_revision`]
    pub  fn update_revision_set(&mut self, update_revision: impl Into<Option<String>>) -> &mut Self {
        self.update_revision = update_revision.into(); self
    }

    pub  fn update_revision(&mut self) -> &mut String {
        if self.update_revision.is_none() { self.update_revision = Some(Default::default()) }
        self.update_revision.as_mut().unwrap()
    }

    /// Modify [`Self::update_revision`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn update_revision_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.update_revision.is_none() { self.update_revision = Some(Default::default()) };
        func(self.update_revision.as_mut().unwrap()); self
    }


    /// Set [`Self::updated_replicas`]
    pub  fn updated_replicas_set(&mut self, updated_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.updated_replicas = updated_replicas.into(); self
    }

    pub  fn updated_replicas(&mut self) -> &mut i32 {
        if self.updated_replicas.is_none() { self.updated_replicas = Some(Default::default()) }
        self.updated_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::updated_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn updated_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.updated_replicas.is_none() { self.updated_replicas = Some(Default::default()) };
        func(self.updated_replicas.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StatefulSetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_collision_count,
            Key_conditions,
            Key_current_replicas,
            Key_current_revision,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
            Key_update_revision,
            Key_updated_replicas,
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
                            "availableReplicas" => Field::Key_available_replicas,
                            "collisionCount" => Field::Key_collision_count,
                            "conditions" => Field::Key_conditions,
                            "currentReplicas" => Field::Key_current_replicas,
                            "currentRevision" => Field::Key_current_revision,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            "updateRevision" => Field::Key_update_revision,
                            "updatedReplicas" => Field::Key_updated_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatefulSetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<Vec<crate::api::apps::v1::StatefulSetCondition>> = None;
                let mut value_current_replicas: Option<i32> = None;
                let mut value_current_revision: Option<String> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_update_revision: Option<String> = None;
                let mut value_updated_replicas: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_collision_count => value_collision_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_replicas => value_current_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_revision => value_current_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_revision => value_update_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_replicas => value_updated_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetStatus {
                    available_replicas: value_available_replicas.unwrap_or_default(),
                    collision_count: value_collision_count,
                    conditions: value_conditions,
                    current_replicas: value_current_replicas,
                    current_revision: value_current_revision,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.unwrap_or_default(),
                    update_revision: value_update_revision,
                    updated_replicas: value_updated_replicas,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetStatus",
            &[
                "availableReplicas",
                "collisionCount",
                "conditions",
                "currentReplicas",
                "currentRevision",
                "observedGeneration",
                "readyReplicas",
                "replicas",
                "updateRevision",
                "updatedReplicas",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetStatus",
            2 +
            self.collision_count.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.current_replicas.as_ref().map_or(0, |_| 1) +
            self.current_revision.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1) +
            self.update_revision.as_ref().map_or(0, |_| 1) +
            self.updated_replicas.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", &self.available_replicas)?;
        if let Some(value) = &self.collision_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.current_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentReplicas", value)?;
        }
        if let Some(value) = &self.current_revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentRevision", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        if let Some(value) = &self.update_revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updateRevision", value)?;
        }
        if let Some(value) = &self.updated_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetStatus {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.StatefulSetStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("StatefulSetStatus represents the current state of a StatefulSet.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "availableReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset. This is a beta field and enabled/disabled by StatefulSetMinReadySeconds feature gate.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "collisionCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.".to_owned()),
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
                                description: Some("Represents the latest available observations of a statefulset's current state.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apps::v1::StatefulSetCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentRevision".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observedGeneration".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readyReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "replicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("replicas is the number of Pods created by the StatefulSet controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "updateRevision".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "updatedReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "availableReplicas".to_owned(),
                    "replicas".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
