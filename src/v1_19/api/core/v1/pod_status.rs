// Generated from definition io.k8s.api.core.v1.PodStatus

/// PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodStatus {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    pub conditions: Option<Vec<crate::api::core::v1::PodCondition>>,

    /// The list has one entry per container in the manifest. Each entry is currently the output of `docker inspect`. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>>,

    /// Status for any ephemeral containers that have run in this pod. This field is alpha-level and is only populated by servers that enable the EphemeralContainers feature.
    pub ephemeral_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>>,

    /// IP address of the host to which the pod is assigned. Empty if not yet scheduled.
    pub host_ip: Option<String>,

    /// The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub init_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>>,

    /// A human readable message indicating details about why the pod is in this condition.
    pub message: Option<String>,

    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
    pub nominated_node_name: Option<String>,

    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:
    ///
    /// Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.
    ///
    /// More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    pub phase: Option<String>,

    /// IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    pub pod_ip: Option<String>,

    /// podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.
    pub pod_ips: Option<Vec<crate::api::core::v1::PodIP>>,

    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://git.k8s.io/community/contributors/design-proposals/node/resource-qos.md
    pub qos_class: Option<String>,

    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    pub reason: Option<String>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

}

#[cfg(feature = "dsl")]
impl PodStatus  {
    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::core::v1::PodCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::core::v1::PodCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PodCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PodCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::container_statuses`]
    pub  fn container_statuses_set(&mut self, container_statuses: impl Into<Option<Vec<crate::api::core::v1::ContainerStatus>>>) -> &mut Self {
        self.container_statuses = container_statuses.into(); self
    }

    pub  fn container_statuses(&mut self) -> &mut Vec<crate::api::core::v1::ContainerStatus> {
        if self.container_statuses.is_none() { self.container_statuses = Some(Default::default()) }
        self.container_statuses.as_mut().unwrap()
    }

    /// Modify [`Self::container_statuses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn container_statuses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::ContainerStatus>)) -> &mut Self {
        if self.container_statuses.is_none() { self.container_statuses = Some(Default::default()) };
        func(self.container_statuses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::container_statuses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn container_statuses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerStatus)) -> &mut Self {
        if self.container_statuses.is_none() {
            self.container_statuses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.container_statuses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::container_statuses`]
    pub  fn container_statuses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::ContainerStatus]>) -> &mut Self {
         if self.container_statuses.is_none() { self.container_statuses = Some(Vec::new()); }
         let container_statuses = &mut self.container_statuses.as_mut().unwrap();
         for item in other.borrow() {
             container_statuses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::ephemeral_container_statuses`]
    pub  fn ephemeral_container_statuses_set(&mut self, ephemeral_container_statuses: impl Into<Option<Vec<crate::api::core::v1::ContainerStatus>>>) -> &mut Self {
        self.ephemeral_container_statuses = ephemeral_container_statuses.into(); self
    }

    pub  fn ephemeral_container_statuses(&mut self) -> &mut Vec<crate::api::core::v1::ContainerStatus> {
        if self.ephemeral_container_statuses.is_none() { self.ephemeral_container_statuses = Some(Default::default()) }
        self.ephemeral_container_statuses.as_mut().unwrap()
    }

    /// Modify [`Self::ephemeral_container_statuses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ephemeral_container_statuses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::ContainerStatus>)) -> &mut Self {
        if self.ephemeral_container_statuses.is_none() { self.ephemeral_container_statuses = Some(Default::default()) };
        func(self.ephemeral_container_statuses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ephemeral_container_statuses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ephemeral_container_statuses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerStatus)) -> &mut Self {
        if self.ephemeral_container_statuses.is_none() {
            self.ephemeral_container_statuses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ephemeral_container_statuses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ephemeral_container_statuses`]
    pub  fn ephemeral_container_statuses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::ContainerStatus]>) -> &mut Self {
         if self.ephemeral_container_statuses.is_none() { self.ephemeral_container_statuses = Some(Vec::new()); }
         let ephemeral_container_statuses = &mut self.ephemeral_container_statuses.as_mut().unwrap();
         for item in other.borrow() {
             ephemeral_container_statuses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::host_ip`]
    pub  fn host_ip_set(&mut self, host_ip: impl Into<Option<String>>) -> &mut Self {
        self.host_ip = host_ip.into(); self
    }

    pub  fn host_ip(&mut self) -> &mut String {
        if self.host_ip.is_none() { self.host_ip = Some(Default::default()) }
        self.host_ip.as_mut().unwrap()
    }

    /// Modify [`Self::host_ip`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_ip_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.host_ip.is_none() { self.host_ip = Some(Default::default()) };
        func(self.host_ip.as_mut().unwrap()); self
    }


    /// Set [`Self::init_container_statuses`]
    pub  fn init_container_statuses_set(&mut self, init_container_statuses: impl Into<Option<Vec<crate::api::core::v1::ContainerStatus>>>) -> &mut Self {
        self.init_container_statuses = init_container_statuses.into(); self
    }

    pub  fn init_container_statuses(&mut self) -> &mut Vec<crate::api::core::v1::ContainerStatus> {
        if self.init_container_statuses.is_none() { self.init_container_statuses = Some(Default::default()) }
        self.init_container_statuses.as_mut().unwrap()
    }

    /// Modify [`Self::init_container_statuses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn init_container_statuses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::ContainerStatus>)) -> &mut Self {
        if self.init_container_statuses.is_none() { self.init_container_statuses = Some(Default::default()) };
        func(self.init_container_statuses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::init_container_statuses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn init_container_statuses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerStatus)) -> &mut Self {
        if self.init_container_statuses.is_none() {
            self.init_container_statuses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.init_container_statuses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::init_container_statuses`]
    pub  fn init_container_statuses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::ContainerStatus]>) -> &mut Self {
         if self.init_container_statuses.is_none() { self.init_container_statuses = Some(Vec::new()); }
         let init_container_statuses = &mut self.init_container_statuses.as_mut().unwrap();
         for item in other.borrow() {
             init_container_statuses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::message`]
    pub  fn message_set(&mut self, message: impl Into<Option<String>>) -> &mut Self {
        self.message = message.into(); self
    }

    pub  fn message(&mut self) -> &mut String {
        if self.message.is_none() { self.message = Some(Default::default()) }
        self.message.as_mut().unwrap()
    }

    /// Modify [`Self::message`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn message_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.message.is_none() { self.message = Some(Default::default()) };
        func(self.message.as_mut().unwrap()); self
    }


    /// Set [`Self::nominated_node_name`]
    pub  fn nominated_node_name_set(&mut self, nominated_node_name: impl Into<Option<String>>) -> &mut Self {
        self.nominated_node_name = nominated_node_name.into(); self
    }

    pub  fn nominated_node_name(&mut self) -> &mut String {
        if self.nominated_node_name.is_none() { self.nominated_node_name = Some(Default::default()) }
        self.nominated_node_name.as_mut().unwrap()
    }

    /// Modify [`Self::nominated_node_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn nominated_node_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.nominated_node_name.is_none() { self.nominated_node_name = Some(Default::default()) };
        func(self.nominated_node_name.as_mut().unwrap()); self
    }


    /// Set [`Self::phase`]
    pub  fn phase_set(&mut self, phase: impl Into<Option<String>>) -> &mut Self {
        self.phase = phase.into(); self
    }

    pub  fn phase(&mut self) -> &mut String {
        if self.phase.is_none() { self.phase = Some(Default::default()) }
        self.phase.as_mut().unwrap()
    }

    /// Modify [`Self::phase`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn phase_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.phase.is_none() { self.phase = Some(Default::default()) };
        func(self.phase.as_mut().unwrap()); self
    }


    /// Set [`Self::pod_ip`]
    pub  fn pod_ip_set(&mut self, pod_ip: impl Into<Option<String>>) -> &mut Self {
        self.pod_ip = pod_ip.into(); self
    }

    pub  fn pod_ip(&mut self) -> &mut String {
        if self.pod_ip.is_none() { self.pod_ip = Some(Default::default()) }
        self.pod_ip.as_mut().unwrap()
    }

    /// Modify [`Self::pod_ip`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pod_ip_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.pod_ip.is_none() { self.pod_ip = Some(Default::default()) };
        func(self.pod_ip.as_mut().unwrap()); self
    }


    /// Set [`Self::pod_ips`]
    pub  fn pod_ips_set(&mut self, pod_ips: impl Into<Option<Vec<crate::api::core::v1::PodIP>>>) -> &mut Self {
        self.pod_ips = pod_ips.into(); self
    }

    pub  fn pod_ips(&mut self) -> &mut Vec<crate::api::core::v1::PodIP> {
        if self.pod_ips.is_none() { self.pod_ips = Some(Default::default()) }
        self.pod_ips.as_mut().unwrap()
    }

    /// Modify [`Self::pod_ips`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pod_ips_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PodIP>)) -> &mut Self {
        if self.pod_ips.is_none() { self.pod_ips = Some(Default::default()) };
        func(self.pod_ips.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::pod_ips`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn pod_ips_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodIP)) -> &mut Self {
        if self.pod_ips.is_none() {
            self.pod_ips = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.pod_ips.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::pod_ips`]
    pub  fn pod_ips_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PodIP]>) -> &mut Self {
         if self.pod_ips.is_none() { self.pod_ips = Some(Vec::new()); }
         let pod_ips = &mut self.pod_ips.as_mut().unwrap();
         for item in other.borrow() {
             pod_ips.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::qos_class`]
    pub  fn qos_class_set(&mut self, qos_class: impl Into<Option<String>>) -> &mut Self {
        self.qos_class = qos_class.into(); self
    }

    pub  fn qos_class(&mut self) -> &mut String {
        if self.qos_class.is_none() { self.qos_class = Some(Default::default()) }
        self.qos_class.as_mut().unwrap()
    }

    /// Modify [`Self::qos_class`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn qos_class_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.qos_class.is_none() { self.qos_class = Some(Default::default()) };
        func(self.qos_class.as_mut().unwrap()); self
    }


    /// Set [`Self::reason`]
    pub  fn reason_set(&mut self, reason: impl Into<Option<String>>) -> &mut Self {
        self.reason = reason.into(); self
    }

    pub  fn reason(&mut self) -> &mut String {
        if self.reason.is_none() { self.reason = Some(Default::default()) }
        self.reason.as_mut().unwrap()
    }

    /// Modify [`Self::reason`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn reason_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.reason.is_none() { self.reason = Some(Default::default()) };
        func(self.reason.as_mut().unwrap()); self
    }


    /// Set [`Self::start_time`]
    pub  fn start_time_set(&mut self, start_time: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.start_time = start_time.into(); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PodStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_container_statuses,
            Key_ephemeral_container_statuses,
            Key_host_ip,
            Key_init_container_statuses,
            Key_message,
            Key_nominated_node_name,
            Key_phase,
            Key_pod_ip,
            Key_pod_ips,
            Key_qos_class,
            Key_reason,
            Key_start_time,
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
                            "conditions" => Field::Key_conditions,
                            "containerStatuses" => Field::Key_container_statuses,
                            "ephemeralContainerStatuses" => Field::Key_ephemeral_container_statuses,
                            "hostIP" => Field::Key_host_ip,
                            "initContainerStatuses" => Field::Key_init_container_statuses,
                            "message" => Field::Key_message,
                            "nominatedNodeName" => Field::Key_nominated_node_name,
                            "phase" => Field::Key_phase,
                            "podIP" => Field::Key_pod_ip,
                            "podIPs" => Field::Key_pod_ips,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::core::v1::PodCondition>> = None;
                let mut value_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_ephemeral_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_host_ip: Option<String> = None;
                let mut value_init_container_statuses: Option<Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_message: Option<String> = None;
                let mut value_nominated_node_name: Option<String> = None;
                let mut value_phase: Option<String> = None;
                let mut value_pod_ip: Option<String> = None;
                let mut value_pod_ips: Option<Vec<crate::api::core::v1::PodIP>> = None;
                let mut value_qos_class: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_statuses => value_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ephemeral_container_statuses => value_ephemeral_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ip => value_host_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_container_statuses => value_init_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nominated_node_name => value_nominated_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_ip => value_pod_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_ips => value_pod_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_qos_class => value_qos_class = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodStatus {
                    conditions: value_conditions,
                    container_statuses: value_container_statuses,
                    ephemeral_container_statuses: value_ephemeral_container_statuses,
                    host_ip: value_host_ip,
                    init_container_statuses: value_init_container_statuses,
                    message: value_message,
                    nominated_node_name: value_nominated_node_name,
                    phase: value_phase,
                    pod_ip: value_pod_ip,
                    pod_ips: value_pod_ips,
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
                "ephemeralContainerStatuses",
                "hostIP",
                "initContainerStatuses",
                "message",
                "nominatedNodeName",
                "phase",
                "podIP",
                "podIPs",
                "qosClass",
                "reason",
                "startTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.container_statuses.as_ref().map_or(0, |_| 1) +
            self.ephemeral_container_statuses.as_ref().map_or(0, |_| 1) +
            self.host_ip.as_ref().map_or(0, |_| 1) +
            self.init_container_statuses.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.nominated_node_name.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.pod_ip.as_ref().map_or(0, |_| 1) +
            self.pod_ips.as_ref().map_or(0, |_| 1) +
            self.qos_class.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.container_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerStatuses", value)?;
        }
        if let Some(value) = &self.ephemeral_container_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ephemeralContainerStatuses", value)?;
        }
        if let Some(value) = &self.host_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIP", value)?;
        }
        if let Some(value) = &self.init_container_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initContainerStatuses", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.nominated_node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nominatedNodeName", value)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.pod_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podIP", value)?;
        }
        if let Some(value) = &self.pod_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podIPs", value)?;
        }
        if let Some(value) = &self.qos_class {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "qosClass", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.start_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PodStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PodCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "containerStatuses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The list has one entry per container in the manifest. Each entry is currently the output of `docker inspect`. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ephemeralContainerStatuses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status for any ephemeral containers that have run in this pod. This field is alpha-level and is only populated by servers that enable the EphemeralContainers feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address of the host to which the pod is assigned. Empty if not yet scheduled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "initContainerStatuses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "message".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A human readable message indicating details about why the pod is in this condition.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nominatedNodeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "phase".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:\n\nPending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.\n\nMore info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podIPs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PodIP>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "qosClass".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://git.k8s.io/community/contributors/design-proposals/node/resource-qos.md".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
