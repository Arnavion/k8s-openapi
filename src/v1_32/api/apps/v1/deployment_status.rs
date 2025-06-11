// Generated from definition io.k8s.api.apps.v1.DeploymentStatus

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    pub available_replicas: Option<i32>,

    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a deployment's current state.
    pub conditions: Option<std::vec::Vec<crate::api::apps::v1::DeploymentCondition>>,

    /// The generation observed by the deployment controller.
    pub observed_generation: Option<i64>,

    /// readyReplicas is the number of pods targeted by this Deployment with a Ready Condition.
    pub ready_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    pub replicas: Option<i32>,

    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    pub unavailable_replicas: Option<i32>,

    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    pub updated_replicas: Option<i32>,
}

impl crate::DeepMerge for DeploymentStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.available_replicas, other.available_replicas);
        crate::DeepMerge::merge_from(&mut self.collision_count, other.collision_count);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::DeepMerge::merge_from(&mut self.ready_replicas, other.ready_replicas);
        crate::DeepMerge::merge_from(&mut self.replicas, other.replicas);
        crate::DeepMerge::merge_from(&mut self.unavailable_replicas, other.unavailable_replicas);
        crate::DeepMerge::merge_from(&mut self.updated_replicas, other.updated_replicas);
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeploymentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::apps::v1::DeploymentCondition>> = None;
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
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apps.v1.DeploymentStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeploymentStatus is the most recently observed status of the Deployment.",
            "type": "object",
            "properties": {
                "availableReplicas": {
                    "description": "Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.",
                    "type": "integer",
                    "format": "int32",
                },
                "collisionCount": {
                    "description": "Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.",
                    "type": "integer",
                    "format": "int32",
                },
                "conditions": {
                    "description": "Represents the latest available observations of a deployment's current state.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::apps::v1::DeploymentCondition>()),
                },
                "observedGeneration": {
                    "description": "The generation observed by the deployment controller.",
                    "type": "integer",
                    "format": "int64",
                },
                "readyReplicas": {
                    "description": "readyReplicas is the number of pods targeted by this Deployment with a Ready Condition.",
                    "type": "integer",
                    "format": "int32",
                },
                "replicas": {
                    "description": "Total number of non-terminated pods targeted by this deployment (their labels match the selector).",
                    "type": "integer",
                    "format": "int32",
                },
                "unavailableReplicas": {
                    "description": "Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.",
                    "type": "integer",
                    "format": "int32",
                },
                "updatedReplicas": {
                    "description": "Total number of non-terminated pods targeted by this deployment that have the desired template spec.",
                    "type": "integer",
                    "format": "int32",
                },
            },
        })
    }
}
