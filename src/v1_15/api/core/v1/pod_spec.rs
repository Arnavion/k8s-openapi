// Generated from definition io.k8s.api.core.v1.PodSpec

/// PodSpec is a description of a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSpec {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    pub active_deadline_seconds: Option<i64>,

    /// If specified, the pod's scheduling constraints
    pub affinity: Option<crate::api::core::v1::Affinity>,

    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    pub automount_service_account_token: Option<bool>,

    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    pub containers: Vec<crate::api::core::v1::Container>,

    /// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
    pub dns_config: Option<crate::api::core::v1::PodDNSConfig>,

    /// Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    pub dns_policy: Option<String>,

    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    pub enable_service_links: Option<bool>,

    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.
    pub host_aliases: Vec<crate::api::core::v1::HostAlias>,

    /// Use the host's ipc namespace. Optional: Default to false.
    pub host_ipc: Option<bool>,

    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
    pub host_network: Option<bool>,

    /// Use the host's pid namespace. Optional: Default to false.
    pub host_pid: Option<bool>,

    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    pub hostname: Option<String>,

    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    pub image_pull_secrets: Vec<crate::api::core::v1::LocalObjectReference>,

    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, or Liveness probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    pub init_containers: Vec<crate::api::core::v1::Container>,

    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
    pub node_name: Option<String>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    pub node_selector: std::collections::BTreeMap<String, String>,

    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is alpha-level and is only honored by servers that enable the NonPreemptingPriority feature.
    pub preemption_policy: Option<String>,

    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    pub priority: Option<i32>,

    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    pub priority_class_name: Option<String>,

    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/0007-pod-ready%2B%2B.md
    pub readiness_gates: Vec<crate::api::core::v1::PodReadinessGate>,

    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    pub restart_policy: Option<String>,

    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/runtime-class.md This is a beta feature as of Kubernetes v1.14.
    pub runtime_class_name: Option<String>,

    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    pub scheduler_name: Option<String>,

    /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
    pub security_context: Option<crate::api::core::v1::PodSecurityContext>,

    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    pub service_account: Option<String>,

    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    pub service_account_name: Option<String>,

    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false. This field is beta-level and may be disabled with the PodShareProcessNamespace feature.
    pub share_process_namespace: Option<bool>,

    /// If specified, the fully qualified Pod hostname will be "\<hostname\>.\<subdomain\>.\<pod namespace\>.svc.\<cluster domain\>". If not specified, the pod will not have a domainname at all.
    pub subdomain: Option<String>,

    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    pub termination_grace_period_seconds: Option<i64>,

    /// If specified, the pod's tolerations.
    pub tolerations: Vec<crate::api::core::v1::Toleration>,

    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    pub volumes: Vec<crate::api::core::v1::Volume>,
}

impl<'de> crate::serde::Deserialize<'de> for PodSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_affinity,
            Key_automount_service_account_token,
            Key_containers,
            Key_dns_config,
            Key_dns_policy,
            Key_enable_service_links,
            Key_host_aliases,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_hostname,
            Key_image_pull_secrets,
            Key_init_containers,
            Key_node_name,
            Key_node_selector,
            Key_preemption_policy,
            Key_priority,
            Key_priority_class_name,
            Key_readiness_gates,
            Key_restart_policy,
            Key_runtime_class_name,
            Key_scheduler_name,
            Key_security_context,
            Key_service_account,
            Key_service_account_name,
            Key_share_process_namespace,
            Key_subdomain,
            Key_termination_grace_period_seconds,
            Key_tolerations,
            Key_volumes,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "affinity" => Field::Key_affinity,
                            "automountServiceAccountToken" => Field::Key_automount_service_account_token,
                            "containers" => Field::Key_containers,
                            "dnsConfig" => Field::Key_dns_config,
                            "dnsPolicy" => Field::Key_dns_policy,
                            "enableServiceLinks" => Field::Key_enable_service_links,
                            "hostAliases" => Field::Key_host_aliases,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostname" => Field::Key_hostname,
                            "imagePullSecrets" => Field::Key_image_pull_secrets,
                            "initContainers" => Field::Key_init_containers,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "preemptionPolicy" => Field::Key_preemption_policy,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "readinessGates" => Field::Key_readiness_gates,
                            "restartPolicy" => Field::Key_restart_policy,
                            "runtimeClassName" => Field::Key_runtime_class_name,
                            "schedulerName" => Field::Key_scheduler_name,
                            "securityContext" => Field::Key_security_context,
                            "serviceAccount" => Field::Key_service_account,
                            "serviceAccountName" => Field::Key_service_account_name,
                            "shareProcessNamespace" => Field::Key_share_process_namespace,
                            "subdomain" => Field::Key_subdomain,
                            "terminationGracePeriodSeconds" => Field::Key_termination_grace_period_seconds,
                            "tolerations" => Field::Key_tolerations,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_affinity: Option<crate::api::core::v1::Affinity> = None;
                let mut value_automount_service_account_token: Option<bool> = None;
                let mut value_containers: Option<Vec<crate::api::core::v1::Container>> = None;
                let mut value_dns_config: Option<crate::api::core::v1::PodDNSConfig> = None;
                let mut value_dns_policy: Option<String> = None;
                let mut value_enable_service_links: Option<bool> = None;
                let mut value_host_aliases: Option<Vec<crate::api::core::v1::HostAlias>> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_hostname: Option<String> = None;
                let mut value_image_pull_secrets: Option<Vec<crate::api::core::v1::LocalObjectReference>> = None;
                let mut value_init_containers: Option<Vec<crate::api::core::v1::Container>> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_preemption_policy: Option<String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<String> = None;
                let mut value_readiness_gates: Option<Vec<crate::api::core::v1::PodReadinessGate>> = None;
                let mut value_restart_policy: Option<String> = None;
                let mut value_runtime_class_name: Option<String> = None;
                let mut value_scheduler_name: Option<String> = None;
                let mut value_security_context: Option<crate::api::core::v1::PodSecurityContext> = None;
                let mut value_service_account: Option<String> = None;
                let mut value_service_account_name: Option<String> = None;
                let mut value_share_process_namespace: Option<bool> = None;
                let mut value_subdomain: Option<String> = None;
                let mut value_termination_grace_period_seconds: Option<i64> = None;
                let mut value_tolerations: Option<Vec<crate::api::core::v1::Toleration>> = None;
                let mut value_volumes: Option<Vec<crate::api::core::v1::Volume>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_affinity => value_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_automount_service_account_token => value_automount_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_containers => value_containers = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_dns_config => value_dns_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_policy => value_dns_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enable_service_links => value_enable_service_links = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_aliases => value_host_aliases = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ipc => value_host_ipc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_secrets => value_image_pull_secrets = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_containers => value_init_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preemption_policy => value_preemption_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_gates => value_readiness_gates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_policy => value_restart_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_runtime_class_name => value_runtime_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduler_name => value_scheduler_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_name => value_service_account_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_process_namespace => value_share_process_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subdomain => value_subdomain = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_grace_period_seconds => value_termination_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    affinity: value_affinity,
                    automount_service_account_token: value_automount_service_account_token,
                    containers: value_containers.ok_or_else(|| crate::serde::de::Error::missing_field("containers"))?,
                    dns_config: value_dns_config,
                    dns_policy: value_dns_policy,
                    enable_service_links: value_enable_service_links,
                    host_aliases: value_host_aliases.unwrap_or_default(),
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    hostname: value_hostname,
                    image_pull_secrets: value_image_pull_secrets.unwrap_or_default(),
                    init_containers: value_init_containers.unwrap_or_default(),
                    node_name: value_node_name,
                    node_selector: value_node_selector.unwrap_or_default(),
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    readiness_gates: value_readiness_gates.unwrap_or_default(),
                    restart_policy: value_restart_policy,
                    runtime_class_name: value_runtime_class_name,
                    scheduler_name: value_scheduler_name,
                    security_context: value_security_context,
                    service_account: value_service_account,
                    service_account_name: value_service_account_name,
                    share_process_namespace: value_share_process_namespace,
                    subdomain: value_subdomain,
                    termination_grace_period_seconds: value_termination_grace_period_seconds,
                    tolerations: value_tolerations.unwrap_or_default(),
                    volumes: value_volumes.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSpec",
            &[
                "activeDeadlineSeconds",
                "affinity",
                "automountServiceAccountToken",
                "containers",
                "dnsConfig",
                "dnsPolicy",
                "enableServiceLinks",
                "hostAliases",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostname",
                "imagePullSecrets",
                "initContainers",
                "nodeName",
                "nodeSelector",
                "preemptionPolicy",
                "priority",
                "priorityClassName",
                "readinessGates",
                "restartPolicy",
                "runtimeClassName",
                "schedulerName",
                "securityContext",
                "serviceAccount",
                "serviceAccountName",
                "shareProcessNamespace",
                "subdomain",
                "terminationGracePeriodSeconds",
                "tolerations",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSpec",
            1 +
            self.active_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.affinity.as_ref().map_or(0, |_| 1) +
            self.automount_service_account_token.as_ref().map_or(0, |_| 1) +
            self.dns_config.as_ref().map_or(0, |_| 1) +
            self.dns_policy.as_ref().map_or(0, |_| 1) +
            self.enable_service_links.as_ref().map_or(0, |_| 1) +
            usize::from(!self.host_aliases.is_empty()) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            usize::from(!self.image_pull_secrets.is_empty()) +
            usize::from(!self.init_containers.is_empty()) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            usize::from(!self.node_selector.is_empty()) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            usize::from(!self.readiness_gates.is_empty()) +
            self.restart_policy.as_ref().map_or(0, |_| 1) +
            self.runtime_class_name.as_ref().map_or(0, |_| 1) +
            self.scheduler_name.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.service_account_name.as_ref().map_or(0, |_| 1) +
            self.share_process_namespace.as_ref().map_or(0, |_| 1) +
            self.subdomain.as_ref().map_or(0, |_| 1) +
            self.termination_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            usize::from(!self.tolerations.is_empty()) +
            usize::from(!self.volumes.is_empty()),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "affinity", value)?;
        }
        if let Some(value) = &self.automount_service_account_token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "automountServiceAccountToken", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containers", &self.containers)?;
        if let Some(value) = &self.dns_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dnsConfig", value)?;
        }
        if let Some(value) = &self.dns_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dnsPolicy", value)?;
        }
        if let Some(value) = &self.enable_service_links {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "enableServiceLinks", value)?;
        }
        if !self.host_aliases.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostAliases", &self.host_aliases)?;
        }
        if let Some(value) = &self.host_ipc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPC", value)?;
        }
        if let Some(value) = &self.host_network {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostNetwork", value)?;
        }
        if let Some(value) = &self.host_pid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPID", value)?;
        }
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if !self.image_pull_secrets.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullSecrets", &self.image_pull_secrets)?;
        }
        if !self.init_containers.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initContainers", &self.init_containers)?;
        }
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if !self.node_selector.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", &self.node_selector)?;
        }
        if let Some(value) = &self.preemption_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preemptionPolicy", value)?;
        }
        if let Some(value) = &self.priority {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        if let Some(value) = &self.priority_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priorityClassName", value)?;
        }
        if !self.readiness_gates.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readinessGates", &self.readiness_gates)?;
        }
        if let Some(value) = &self.restart_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartPolicy", value)?;
        }
        if let Some(value) = &self.runtime_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runtimeClassName", value)?;
        }
        if let Some(value) = &self.scheduler_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulerName", value)?;
        }
        if let Some(value) = &self.security_context {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.service_account {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.service_account_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountName", value)?;
        }
        if let Some(value) = &self.share_process_namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareProcessNamespace", value)?;
        }
        if let Some(value) = &self.subdomain {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subdomain", value)?;
        }
        if let Some(value) = &self.termination_grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationGracePeriodSeconds", value)?;
        }
        if !self.tolerations.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", &self.tolerations)?;
        }
        if !self.volumes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", &self.volumes)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PodSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PodSpec is a description of a pod.",
          "properties": {
            "activeDeadlineSeconds": {
              "description": "Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.",
              "format": "int64",
              "type": "integer"
            },
            "affinity": crate::schema_ref_with_description(crate::api::core::v1::Affinity::schema(), "If specified, the pod's scheduling constraints"),
            "automountServiceAccountToken": {
              "description": "AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.",
              "type": "boolean"
            },
            "containers": {
              "description": "List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.",
              "items": crate::api::core::v1::Container::schema(),
              "type": "array"
            },
            "dnsConfig": crate::schema_ref_with_description(crate::api::core::v1::PodDNSConfig::schema(), "Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy."),
            "dnsPolicy": {
              "description": "Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.",
              "type": "string"
            },
            "enableServiceLinks": {
              "description": "EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.",
              "type": "boolean"
            },
            "hostAliases": {
              "description": "HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.",
              "items": crate::api::core::v1::HostAlias::schema(),
              "type": "array"
            },
            "hostIPC": {
              "description": "Use the host's ipc namespace. Optional: Default to false.",
              "type": "boolean"
            },
            "hostNetwork": {
              "description": "Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.",
              "type": "boolean"
            },
            "hostPID": {
              "description": "Use the host's pid namespace. Optional: Default to false.",
              "type": "boolean"
            },
            "hostname": {
              "description": "Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.",
              "type": "string"
            },
            "imagePullSecrets": {
              "description": "ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod",
              "items": crate::api::core::v1::LocalObjectReference::schema(),
              "type": "array"
            },
            "initContainers": {
              "description": "List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, or Liveness probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/",
              "items": crate::api::core::v1::Container::schema(),
              "type": "array"
            },
            "nodeName": {
              "description": "NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.",
              "type": "string"
            },
            "nodeSelector": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/",
              "type": "object"
            },
            "preemptionPolicy": {
              "description": "PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is alpha-level and is only honored by servers that enable the NonPreemptingPriority feature.",
              "type": "string"
            },
            "priority": {
              "description": "The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.",
              "format": "int32",
              "type": "integer"
            },
            "priorityClassName": {
              "description": "If specified, indicates the pod's priority. \"system-node-critical\" and \"system-cluster-critical\" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.",
              "type": "string"
            },
            "readinessGates": {
              "description": "If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to \"True\" More info: https://git.k8s.io/enhancements/keps/sig-network/0007-pod-ready%2B%2B.md",
              "items": crate::api::core::v1::PodReadinessGate::schema(),
              "type": "array"
            },
            "restartPolicy": {
              "description": "Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy",
              "type": "string"
            },
            "runtimeClassName": {
              "description": "RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the \"legacy\" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/runtime-class.md This is a beta feature as of Kubernetes v1.14.",
              "type": "string"
            },
            "schedulerName": {
              "description": "If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.",
              "type": "string"
            },
            "securityContext": crate::schema_ref_with_description(crate::api::core::v1::PodSecurityContext::schema(), "SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field."),
            "serviceAccount": {
              "description": "DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.",
              "type": "string"
            },
            "serviceAccountName": {
              "description": "ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/",
              "type": "string"
            },
            "shareProcessNamespace": {
              "description": "Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false. This field is beta-level and may be disabled with the PodShareProcessNamespace feature.",
              "type": "boolean"
            },
            "subdomain": {
              "description": "If specified, the fully qualified Pod hostname will be \"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>\". If not specified, the pod will not have a domainname at all.",
              "type": "string"
            },
            "terminationGracePeriodSeconds": {
              "description": "Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.",
              "format": "int64",
              "type": "integer"
            },
            "tolerations": {
              "description": "If specified, the pod's tolerations.",
              "items": crate::api::core::v1::Toleration::schema(),
              "type": "array"
            },
            "volumes": {
              "description": "List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes",
              "items": crate::api::core::v1::Volume::schema(),
              "type": "array"
            }
          },
          "required": [
            "containers"
          ],
          "type": "object"
        })
    }
}
