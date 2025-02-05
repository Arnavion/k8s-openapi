// Generated from definition io.k8s.api.core.v1.PodStatus

/// PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodStatus {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    pub conditions: Option<std::vec::Vec<crate::api::core::v1::PodCondition>>,

    /// The list has one entry per container in the manifest. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>>,

    /// Status for any ephemeral containers that have run in this pod.
    pub ephemeral_container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>>,

    /// hostIP holds the IP address of the host to which the pod is assigned. Empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns mean that HostIP will not be updated even if there is a node is assigned to pod
    pub host_ip: Option<std::string::String>,

    /// hostIPs holds the IP addresses allocated to the host. If this field is specified, the first entry must match the hostIP field. This list is empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns means that HostIPs will not be updated even if there is a node is assigned to this pod.
    pub host_ips: Option<std::vec::Vec<crate::api::core::v1::HostIP>>,

    /// The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    pub init_container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>>,

    /// A human readable message indicating details about why the pod is in this condition.
    pub message: Option<std::string::String>,

    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
    pub nominated_node_name: Option<std::string::String>,

    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:
    ///
    /// Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.
    ///
    /// More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    pub phase: Option<std::string::String>,

    /// podIP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    pub pod_ip: Option<std::string::String>,

    /// podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.
    pub pod_ips: Option<std::vec::Vec<crate::api::core::v1::PodIP>>,

    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/#quality-of-service-classes
    pub qos_class: Option<std::string::String>,

    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    pub reason: Option<std::string::String>,

    /// Status of resources resize desired for pod's containers. It is empty if no resources resize is pending. Any changes to container resources will automatically set this to "Proposed"
    pub resize: Option<std::string::String>,

    /// Status of resource claims.
    pub resource_claim_statuses: Option<std::vec::Vec<crate::api::core::v1::PodResourceClaimStatus>>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for PodStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::atomic(&mut self.container_statuses, other.container_statuses);
        crate::merge_strategies::list::atomic(&mut self.ephemeral_container_statuses, other.ephemeral_container_statuses);
        crate::DeepMerge::merge_from(&mut self.host_ip, other.host_ip);
        crate::merge_strategies::list::map(
            &mut self.host_ips,
            other.host_ips,
            &[|lhs, rhs| lhs.ip == rhs.ip],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::atomic(&mut self.init_container_statuses, other.init_container_statuses);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.nominated_node_name, other.nominated_node_name);
        crate::DeepMerge::merge_from(&mut self.phase, other.phase);
        crate::DeepMerge::merge_from(&mut self.pod_ip, other.pod_ip);
        crate::merge_strategies::list::map(
            &mut self.pod_ips,
            other.pod_ips,
            &[|lhs, rhs| lhs.ip == rhs.ip],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.qos_class, other.qos_class);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.resize, other.resize);
        crate::merge_strategies::list::map(
            &mut self.resource_claim_statuses,
            other.resource_claim_statuses,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.start_time, other.start_time);
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
            Key_host_ips,
            Key_init_container_statuses,
            Key_message,
            Key_nominated_node_name,
            Key_phase,
            Key_pod_ip,
            Key_pod_ips,
            Key_qos_class,
            Key_reason,
            Key_resize,
            Key_resource_claim_statuses,
            Key_start_time,
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
                            "conditions" => Field::Key_conditions,
                            "containerStatuses" => Field::Key_container_statuses,
                            "ephemeralContainerStatuses" => Field::Key_ephemeral_container_statuses,
                            "hostIP" => Field::Key_host_ip,
                            "hostIPs" => Field::Key_host_ips,
                            "initContainerStatuses" => Field::Key_init_container_statuses,
                            "message" => Field::Key_message,
                            "nominatedNodeName" => Field::Key_nominated_node_name,
                            "phase" => Field::Key_phase,
                            "podIP" => Field::Key_pod_ip,
                            "podIPs" => Field::Key_pod_ips,
                            "qosClass" => Field::Key_qos_class,
                            "reason" => Field::Key_reason,
                            "resize" => Field::Key_resize,
                            "resourceClaimStatuses" => Field::Key_resource_claim_statuses,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::api::core::v1::PodCondition>> = None;
                let mut value_container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_ephemeral_container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_host_ip: Option<std::string::String> = None;
                let mut value_host_ips: Option<std::vec::Vec<crate::api::core::v1::HostIP>> = None;
                let mut value_init_container_statuses: Option<std::vec::Vec<crate::api::core::v1::ContainerStatus>> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_nominated_node_name: Option<std::string::String> = None;
                let mut value_phase: Option<std::string::String> = None;
                let mut value_pod_ip: Option<std::string::String> = None;
                let mut value_pod_ips: Option<std::vec::Vec<crate::api::core::v1::PodIP>> = None;
                let mut value_qos_class: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_resize: Option<std::string::String> = None;
                let mut value_resource_claim_statuses: Option<std::vec::Vec<crate::api::core::v1::PodResourceClaimStatus>> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_statuses => value_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ephemeral_container_statuses => value_ephemeral_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ip => value_host_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ips => value_host_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_container_statuses => value_init_container_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nominated_node_name => value_nominated_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_ip => value_pod_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_ips => value_pod_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_qos_class => value_qos_class = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resize => value_resize = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claim_statuses => value_resource_claim_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodStatus {
                    conditions: value_conditions,
                    container_statuses: value_container_statuses,
                    ephemeral_container_statuses: value_ephemeral_container_statuses,
                    host_ip: value_host_ip,
                    host_ips: value_host_ips,
                    init_container_statuses: value_init_container_statuses,
                    message: value_message,
                    nominated_node_name: value_nominated_node_name,
                    phase: value_phase,
                    pod_ip: value_pod_ip,
                    pod_ips: value_pod_ips,
                    qos_class: value_qos_class,
                    reason: value_reason,
                    resize: value_resize,
                    resource_claim_statuses: value_resource_claim_statuses,
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
                "hostIPs",
                "initContainerStatuses",
                "message",
                "nominatedNodeName",
                "phase",
                "podIP",
                "podIPs",
                "qosClass",
                "reason",
                "resize",
                "resourceClaimStatuses",
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
            self.host_ips.as_ref().map_or(0, |_| 1) +
            self.init_container_statuses.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.nominated_node_name.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.pod_ip.as_ref().map_or(0, |_| 1) +
            self.pod_ips.as_ref().map_or(0, |_| 1) +
            self.qos_class.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.resize.as_ref().map_or(0, |_| 1) +
            self.resource_claim_statuses.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.host_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPs", value)?;
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
        if let Some(value) = &self.resize {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resize", value)?;
        }
        if let Some(value) = &self.resource_claim_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaimStatuses", value)?;
        }
        if let Some(value) = &self.start_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PodStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::PodCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "containerStatuses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The list has one entry per container in the manifest. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ephemeralContainerStatuses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status for any ephemeral containers that have run in this pod.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostIP".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostIP holds the IP address of the host to which the pod is assigned. Empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns mean that HostIP will not be updated even if there is a node is assigned to pod".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostIPs".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostIPs holds the IP addresses allocated to the host. If this field is specified, the first entry must match the hostIP field. This list is empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns means that HostIPs will not be updated even if there is a node is assigned to this pod.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::HostIP>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "initContainerStatuses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::ContainerStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "message".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A human readable message indicating details about why the pod is in this condition.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nominatedNodeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "phase".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:\n\nPending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.\n\nMore info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podIP".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("podIP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podIPs".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::PodIP>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "qosClass".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/#quality-of-service-classes".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resize".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status of resources resize desired for pod's containers. It is empty if no resources resize is pending. Any changes to container resources will automatically set this to \"Proposed\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaimStatuses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status of resource claims.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::PodResourceClaimStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.".into()),
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
