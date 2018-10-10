// Generated from definition io.k8s.api.apps.v1beta1.DeploymentStatus

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    pub available_replicas: Option<i32>,

    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a deployment's current state.
    pub conditions: Option<Vec<::v1_12::api::apps::v1beta1::DeploymentCondition>>,

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

impl<'de> ::serde::Deserialize<'de> for DeploymentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DeploymentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<Vec<::v1_12::api::apps::v1beta1::DeploymentCondition>> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_unavailable_replicas: Option<i32> = None;
                let mut value_updated_replicas: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_collision_count => value_collision_count = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unavailable_replicas => value_unavailable_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_replicas => value_updated_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for DeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentStatus",
            0 +
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
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.collision_count {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.observed_generation {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        if let Some(value) = &self.replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.unavailable_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "unavailableReplicas", value)?;
        }
        if let Some(value) = &self.updated_replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedReplicas", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
