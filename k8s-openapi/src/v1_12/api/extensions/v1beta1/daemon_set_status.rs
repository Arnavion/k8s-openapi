// Generated from definition io.k8s.api.extensions.v1beta1.DaemonSetStatus

/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetStatus {
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a DaemonSet's current state.
    pub conditions: Option<Vec<::v1_12::api::extensions::v1beta1::DaemonSetCondition>>,

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

impl<'de> ::serde::Deserialize<'de> for DaemonSetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DaemonSetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<Vec<::v1_12::api::extensions::v1beta1::DaemonSetCondition>> = None;
                let mut value_current_number_scheduled: Option<i32> = None;
                let mut value_desired_number_scheduled: Option<i32> = None;
                let mut value_number_available: Option<i32> = None;
                let mut value_number_misscheduled: Option<i32> = None;
                let mut value_number_ready: Option<i32> = None;
                let mut value_number_unavailable: Option<i32> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_updated_number_scheduled: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_collision_count => value_collision_count = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_number_scheduled => value_current_number_scheduled = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_desired_number_scheduled => value_desired_number_scheduled = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_number_available => value_number_available = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_number_misscheduled => value_number_misscheduled = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_number_ready => value_number_ready = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_number_unavailable => value_number_unavailable = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_number_scheduled => value_updated_number_scheduled = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetStatus {
                    collision_count: value_collision_count,
                    conditions: value_conditions,
                    current_number_scheduled: value_current_number_scheduled.ok_or_else(|| ::serde::de::Error::missing_field("currentNumberScheduled"))?,
                    desired_number_scheduled: value_desired_number_scheduled.ok_or_else(|| ::serde::de::Error::missing_field("desiredNumberScheduled"))?,
                    number_available: value_number_available,
                    number_misscheduled: value_number_misscheduled.ok_or_else(|| ::serde::de::Error::missing_field("numberMisscheduled"))?,
                    number_ready: value_number_ready.ok_or_else(|| ::serde::de::Error::missing_field("numberReady"))?,
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

impl ::serde::Serialize for DaemonSetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetStatus",
            0 +
            self.collision_count.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.number_available.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.number_unavailable.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.updated_number_scheduled.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.collision_count {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "currentNumberScheduled", &self.current_number_scheduled)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "desiredNumberScheduled", &self.desired_number_scheduled)?;
        if let Some(value) = &self.number_available {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "numberAvailable", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "numberMisscheduled", &self.number_misscheduled)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "numberReady", &self.number_ready)?;
        if let Some(value) = &self.number_unavailable {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "numberUnavailable", value)?;
        }
        if let Some(value) = &self.observed_generation {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.updated_number_scheduled {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedNumberScheduled", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
