// Generated from definition io.k8s.api.core.v1.PodStatus

/// PodStatus represents information about the status of a pod. Status may trail the actual state of a system.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodStatus {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    pub conditions: Option<Vec<crate::api::core::v1::PodCondition>>,

    /// The list has one entry per container in the manifest. Each entry is currently the output of `docker inspect`. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>>,

    /// IP address of the host to which the pod is assigned. Empty if not yet scheduled.
    pub host_ip: Option<String>,

    /// The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub init_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>>,

    /// A human readable message indicating details about why the pod is in this condition.
    pub message: Option<String>,

    /// Current condition of the pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    pub phase: Option<String>,

    /// IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    pub pod_ip: Option<String>,

    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://github.com/kubernetes/kubernetes/blob/master/docs/design/resource-qos.md
    pub qos_class: Option<String>,

    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    pub reason: Option<String>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> serde::Deserialize<'de> for PodStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_container_statuses,
            Key_host_ip,
            Key_init_container_statuses,
            Key_message,
            Key_phase,
            Key_pod_ip,
            Key_qos_class,
            Key_reason,
            Key_start_time,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "conditions" => Field::Key_conditions,
                            "containerStatuses" => Field::Key_container_statuses,
                            "hostIP" => Field::Key_host_ip,
                            "initContainerStatuses" => Field::Key_init_container_statuses,
                            "message" => Field::Key_message,
                            "phase" => Field::Key_phase,
                            "podIP" => Field::Key_pod_ip,
                            "qosClass" => Field::Key_qos_class,
                            "reason" => Field::Key_reason,
                            "startTime" => Field::Key_start_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::core::v1::PodCondition>> = None;
                let mut value_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_host_ip: Option<String> = None;
                let mut value_init_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_message: Option<String> = None;
                let mut value_phase: Option<String> = None;
                let mut value_pod_ip: Option<String> = None;
                let mut value_qos_class: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_statuses => value_container_statuses = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ip => value_host_ip = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_container_statuses => value_init_container_statuses = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_ip => value_pod_ip = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_qos_class => value_qos_class = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodStatus {
                    conditions: value_conditions,
                    container_statuses: value_container_statuses,
                    host_ip: value_host_ip,
                    init_container_statuses: value_init_container_statuses,
                    message: value_message,
                    phase: value_phase,
                    pod_ip: value_pod_ip,
                    qos_class: value_qos_class,
                    reason: value_reason,
                    start_time: value_start_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodStatus",
            &[
                "conditions",
                "containerStatuses",
                "hostIP",
                "initContainerStatuses",
                "message",
                "phase",
                "podIP",
                "qosClass",
                "reason",
                "startTime",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.container_statuses.as_ref().map_or(0, |_| 1) +
            self.host_ip.as_ref().map_or(0, |_| 1) +
            self.init_container_statuses.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.pod_ip.as_ref().map_or(0, |_| 1) +
            self.qos_class.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.container_statuses {
            serde::ser::SerializeStruct::serialize_field(&mut state, "containerStatuses", value)?;
        }
        if let Some(value) = &self.host_ip {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostIP", value)?;
        }
        if let Some(value) = &self.init_container_statuses {
            serde::ser::SerializeStruct::serialize_field(&mut state, "initContainerStatuses", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.phase {
            serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.pod_ip {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podIP", value)?;
        }
        if let Some(value) = &self.qos_class {
            serde::ser::SerializeStruct::serialize_field(&mut state, "qosClass", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.start_time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
