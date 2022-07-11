// Generated from definition io.k8s.api.apps.v1.ReplicaSetStatus

/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicaSetStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this replica set.
    pub available_replicas: Option<i32>,

    /// Represents the latest available observations of a replica set's current state.
    pub conditions: Option<Vec<crate::api::apps::v1::ReplicaSetCondition>>,

    /// The number of pods that have labels matching the labels of the pod template of the replicaset.
    pub fully_labeled_replicas: Option<i32>,

    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    pub observed_generation: Option<i64>,

    /// The number of ready replicas for this replica set.
    pub ready_replicas: Option<i32>,

    /// Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    pub replicas: i32,

}

#[cfg(feature = "dsl")]
impl ReplicaSetStatus  {
    /// Set [`Self::available_replicas`]
    pub  fn available_replicas_set(&mut self, available_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.available_replicas = available_replicas.into(); self
    }

    pub  fn available_replicas(&mut self) -> &mut i32 {
        if self.available_replicas.is_none() { self.available_replicas = Some(Default::default()) }
        self.available_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::available_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn available_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.available_replicas.is_none() { self.available_replicas = Some(Default::default()) };
        func(self.available_replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::apps::v1::ReplicaSetCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::apps::v1::ReplicaSetCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apps::v1::ReplicaSetCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::ReplicaSetCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apps::v1::ReplicaSetCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::fully_labeled_replicas`]
    pub  fn fully_labeled_replicas_set(&mut self, fully_labeled_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.fully_labeled_replicas = fully_labeled_replicas.into(); self
    }

    pub  fn fully_labeled_replicas(&mut self) -> &mut i32 {
        if self.fully_labeled_replicas.is_none() { self.fully_labeled_replicas = Some(Default::default()) }
        self.fully_labeled_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::fully_labeled_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fully_labeled_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.fully_labeled_replicas.is_none() { self.fully_labeled_replicas = Some(Default::default()) };
        func(self.fully_labeled_replicas.as_mut().unwrap()); self
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


}


impl<'de> crate::serde::Deserialize<'de> for ReplicaSetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_conditions,
            Key_fully_labeled_replicas,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
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
                            "conditions" => Field::Key_conditions,
                            "fullyLabeledReplicas" => Field::Key_fully_labeled_replicas,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicaSetStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ReplicaSetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_conditions: Option<Vec<crate::api::apps::v1::ReplicaSetCondition>> = None;
                let mut value_fully_labeled_replicas: Option<i32> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fully_labeled_replicas => value_fully_labeled_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicaSetStatus {
                    available_replicas: value_available_replicas,
                    conditions: value_conditions,
                    fully_labeled_replicas: value_fully_labeled_replicas,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicaSetStatus",
            &[
                "availableReplicas",
                "conditions",
                "fullyLabeledReplicas",
                "observedGeneration",
                "readyReplicas",
                "replicas",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ReplicaSetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicaSetStatus",
            1 +
            self.available_replicas.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.fully_labeled_replicas.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.available_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.fully_labeled_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fullyLabeledReplicas", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ReplicaSetStatus {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.ReplicaSetStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ReplicaSetStatus represents the current status of a ReplicaSet.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "availableReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of available replicas (ready for at least minReadySeconds) for this replica set.".to_owned()),
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
                                description: Some("Represents the latest available observations of a replica set's current state.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apps::v1::ReplicaSetCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fullyLabeledReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods that have labels matching the labels of the pod template of the replicaset.".to_owned()),
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
                                description: Some("ObservedGeneration reflects the generation of the most recently observed ReplicaSet.".to_owned()),
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
                                description: Some("The number of ready replicas for this replica set.".to_owned()),
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
                                description: Some("Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "replicas".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
