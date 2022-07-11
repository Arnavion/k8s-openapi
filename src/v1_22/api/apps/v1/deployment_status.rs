// Generated from definition io.k8s.api.apps.v1.DeploymentStatus

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    pub available_replicas: Option<i32>,

    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a deployment's current state.
    pub conditions: Option<Vec<crate::api::apps::v1::DeploymentCondition>>,

    /// The generation observed by the deployment controller.
    pub observed_generation: Option<i64>,

    /// Total number of ready pods targeted by this deployment.
    pub ready_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    pub replicas: Option<i32>,

    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    pub unavailable_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    pub updated_replicas: Option<i32>,

}

#[cfg(feature = "dsl")]
impl DeploymentStatus  {
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
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::apps::v1::DeploymentCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::apps::v1::DeploymentCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::apps::v1::DeploymentCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::DeploymentCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::apps::v1::DeploymentCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
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
    pub  fn replicas_set(&mut self, replicas: impl Into<Option<i32>>) -> &mut Self {
        self.replicas = replicas.into(); self
    }

    pub  fn replicas(&mut self) -> &mut i32 {
        if self.replicas.is_none() { self.replicas = Some(Default::default()) }
        self.replicas.as_mut().unwrap()
    }

    /// Modify [`Self::replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.replicas.is_none() { self.replicas = Some(Default::default()) };
        func(self.replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::unavailable_replicas`]
    pub  fn unavailable_replicas_set(&mut self, unavailable_replicas: impl Into<Option<i32>>) -> &mut Self {
        self.unavailable_replicas = unavailable_replicas.into(); self
    }

    pub  fn unavailable_replicas(&mut self) -> &mut i32 {
        if self.unavailable_replicas.is_none() { self.unavailable_replicas = Some(Default::default()) }
        self.unavailable_replicas.as_mut().unwrap()
    }

    /// Modify [`Self::unavailable_replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn unavailable_replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.unavailable_replicas.is_none() { self.unavailable_replicas = Some(Default::default()) };
        func(self.unavailable_replicas.as_mut().unwrap()); self
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


impl<'de> crate::serde::Deserialize<'de> for DeploymentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_collision_count,
            Key_conditions,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
            Key_unavailable_replicas,
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
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            "unavailableReplicas" => Field::Key_unavailable_replicas,
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
            type Value = DeploymentStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeploymentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<Vec<crate::api::apps::v1::DeploymentCondition>> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_unavailable_replicas: Option<i32> = None;
                let mut value_updated_replicas: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_collision_count => value_collision_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unavailable_replicas => value_unavailable_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_replicas => value_updated_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentStatus {
                    available_replicas: value_available_replicas,
                    collision_count: value_collision_count,
                    conditions: value_conditions,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas,
                    unavailable_replicas: value_unavailable_replicas,
                    updated_replicas: value_updated_replicas,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentStatus",
            &[
                "availableReplicas",
                "collisionCount",
                "conditions",
                "observedGeneration",
                "readyReplicas",
                "replicas",
                "unavailableReplicas",
                "updatedReplicas",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentStatus",
            self.available_replicas.as_ref().map_or(0, |_| 1) +
            self.collision_count.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.unavailable_replicas.as_ref().map_or(0, |_| 1) +
            self.updated_replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.available_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.collision_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        if let Some(value) = &self.replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.unavailable_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unavailableReplicas", value)?;
        }
        if let Some(value) = &self.updated_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeploymentStatus {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.DeploymentStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("DeploymentStatus is the most recently observed status of the Deployment.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "availableReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.".to_owned()),
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
                                description: Some("Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.".to_owned()),
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
                                description: Some("Represents the latest available observations of a deployment's current state.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::apps::v1::DeploymentCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observedGeneration".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The generation observed by the deployment controller.".to_owned()),
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
                                description: Some("Total number of ready pods targeted by this deployment.".to_owned()),
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
                                description: Some("Total number of non-terminated pods targeted by this deployment (their labels match the selector).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "unavailableReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "updatedReplicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Total number of non-terminated pods targeted by this deployment that have the desired template spec.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
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
