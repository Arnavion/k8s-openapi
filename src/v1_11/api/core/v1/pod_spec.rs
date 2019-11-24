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

    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.
    pub host_aliases: Option<Vec<crate::api::core::v1::HostAlias>>,

    /// Use the host's ipc namespace. Optional: Default to false.
    pub host_ipc: Option<bool>,

    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
    pub host_network: Option<bool>,

    /// Use the host's pid namespace. Optional: Default to false.
    pub host_pid: Option<bool>,

    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    pub hostname: Option<String>,

    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    pub image_pull_secrets: Option<Vec<crate::api::core::v1::LocalObjectReference>>,

    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, or Liveness probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    pub init_containers: Option<Vec<crate::api::core::v1::Container>>,

    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
    pub node_name: Option<String>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    pub node_selector: Option<std::collections::BTreeMap<String, String>>,

    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    pub priority: Option<i32>,

    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    pub priority_class_name: Option<String>,

    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://github.com/kubernetes/community/blob/master/keps/sig-network/0007-pod-ready%2B%2B.md
    pub readiness_gates: Option<Vec<crate::api::core::v1::PodReadinessGate>>,

    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    pub restart_policy: Option<String>,

    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    pub scheduler_name: Option<String>,

    /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
    pub security_context: Option<crate::api::core::v1::PodSecurityContext>,

    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    pub service_account: Option<String>,

    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    pub service_account_name: Option<String>,

    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false. This field is alpha-level and is honored only by servers that enable the PodShareProcessNamespace feature.
    pub share_process_namespace: Option<bool>,

    /// If specified, the fully qualified Pod hostname will be "\<hostname\>.\<subdomain\>.\<pod namespace\>.svc.\<cluster domain\>". If not specified, the pod will not have a domainname at all.
    pub subdomain: Option<String>,

    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    pub termination_grace_period_seconds: Option<i64>,

    /// If specified, the pod's tolerations.
    pub tolerations: Option<Vec<crate::api::core::v1::Toleration>>,

    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    pub volumes: Option<Vec<crate::api::core::v1::Volume>>,
}

impl<'de> serde::Deserialize<'de> for PodSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active_deadline_seconds,
            Key_affinity,
            Key_automount_service_account_token,
            Key_containers,
            Key_dns_config,
            Key_dns_policy,
            Key_host_aliases,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_hostname,
            Key_image_pull_secrets,
            Key_init_containers,
            Key_node_name,
            Key_node_selector,
            Key_priority,
            Key_priority_class_name,
            Key_readiness_gates,
            Key_restart_policy,
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
                            "activeDeadlineSeconds" => Field::Key_active_deadline_seconds,
                            "affinity" => Field::Key_affinity,
                            "automountServiceAccountToken" => Field::Key_automount_service_account_token,
                            "containers" => Field::Key_containers,
                            "dnsConfig" => Field::Key_dns_config,
                            "dnsPolicy" => Field::Key_dns_policy,
                            "hostAliases" => Field::Key_host_aliases,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostname" => Field::Key_hostname,
                            "imagePullSecrets" => Field::Key_image_pull_secrets,
                            "initContainers" => Field::Key_init_containers,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "readinessGates" => Field::Key_readiness_gates,
                            "restartPolicy" => Field::Key_restart_policy,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_active_deadline_seconds: Option<i64> = None;
                let mut value_affinity: Option<crate::api::core::v1::Affinity> = None;
                let mut value_automount_service_account_token: Option<bool> = None;
                let mut value_containers: Option<Vec<crate::api::core::v1::Container>> = None;
                let mut value_dns_config: Option<crate::api::core::v1::PodDNSConfig> = None;
                let mut value_dns_policy: Option<String> = None;
                let mut value_host_aliases: Option<Vec<crate::api::core::v1::HostAlias>> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_hostname: Option<String> = None;
                let mut value_image_pull_secrets: Option<Vec<crate::api::core::v1::LocalObjectReference>> = None;
                let mut value_init_containers: Option<Vec<crate::api::core::v1::Container>> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<String> = None;
                let mut value_readiness_gates: Option<Vec<crate::api::core::v1::PodReadinessGate>> = None;
                let mut value_restart_policy: Option<String> = None;
                let mut value_scheduler_name: Option<String> = None;
                let mut value_security_context: Option<crate::api::core::v1::PodSecurityContext> = None;
                let mut value_service_account: Option<String> = None;
                let mut value_service_account_name: Option<String> = None;
                let mut value_share_process_namespace: Option<bool> = None;
                let mut value_subdomain: Option<String> = None;
                let mut value_termination_grace_period_seconds: Option<i64> = None;
                let mut value_tolerations: Option<Vec<crate::api::core::v1::Toleration>> = None;
                let mut value_volumes: Option<Vec<crate::api::core::v1::Volume>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_affinity => value_affinity = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_automount_service_account_token => value_automount_service_account_token = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_containers => value_containers = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_dns_config => value_dns_config = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_policy => value_dns_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_aliases => value_host_aliases = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ipc => value_host_ipc = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_secrets => value_image_pull_secrets = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_containers => value_init_containers = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_gates => value_readiness_gates = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_policy => value_restart_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduler_name => value_scheduler_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account => value_service_account = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_name => value_service_account_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_process_namespace => value_share_process_namespace = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subdomain => value_subdomain = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_grace_period_seconds => value_termination_grace_period_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    affinity: value_affinity,
                    automount_service_account_token: value_automount_service_account_token,
                    containers: value_containers.ok_or_else(|| serde::de::Error::missing_field("containers"))?,
                    dns_config: value_dns_config,
                    dns_policy: value_dns_policy,
                    host_aliases: value_host_aliases,
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    hostname: value_hostname,
                    image_pull_secrets: value_image_pull_secrets,
                    init_containers: value_init_containers,
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    readiness_gates: value_readiness_gates,
                    restart_policy: value_restart_policy,
                    scheduler_name: value_scheduler_name,
                    security_context: value_security_context,
                    service_account: value_service_account,
                    service_account_name: value_service_account_name,
                    share_process_namespace: value_share_process_namespace,
                    subdomain: value_subdomain,
                    termination_grace_period_seconds: value_termination_grace_period_seconds,
                    tolerations: value_tolerations,
                    volumes: value_volumes,
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
                "hostAliases",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostname",
                "imagePullSecrets",
                "initContainers",
                "nodeName",
                "nodeSelector",
                "priority",
                "priorityClassName",
                "readinessGates",
                "restartPolicy",
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

impl serde::Serialize for PodSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSpec",
            1 +
            self.active_deadline_seconds.as_ref().map_or(0, |_| 1) +
            self.affinity.as_ref().map_or(0, |_| 1) +
            self.automount_service_account_token.as_ref().map_or(0, |_| 1) +
            self.dns_config.as_ref().map_or(0, |_| 1) +
            self.dns_policy.as_ref().map_or(0, |_| 1) +
            self.host_aliases.as_ref().map_or(0, |_| 1) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.image_pull_secrets.as_ref().map_or(0, |_| 1) +
            self.init_containers.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.readiness_gates.as_ref().map_or(0, |_| 1) +
            self.restart_policy.as_ref().map_or(0, |_| 1) +
            self.scheduler_name.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.service_account_name.as_ref().map_or(0, |_| 1) +
            self.share_process_namespace.as_ref().map_or(0, |_| 1) +
            self.subdomain.as_ref().map_or(0, |_| 1) +
            self.termination_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active_deadline_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "activeDeadlineSeconds", value)?;
        }
        if let Some(value) = &self.affinity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "affinity", value)?;
        }
        if let Some(value) = &self.automount_service_account_token {
            serde::ser::SerializeStruct::serialize_field(&mut state, "automountServiceAccountToken", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "containers", &self.containers)?;
        if let Some(value) = &self.dns_config {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dnsConfig", value)?;
        }
        if let Some(value) = &self.dns_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dnsPolicy", value)?;
        }
        if let Some(value) = &self.host_aliases {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostAliases", value)?;
        }
        if let Some(value) = &self.host_ipc {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPC", value)?;
        }
        if let Some(value) = &self.host_network {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostNetwork", value)?;
        }
        if let Some(value) = &self.host_pid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostPID", value)?;
        }
        if let Some(value) = &self.hostname {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.image_pull_secrets {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullSecrets", value)?;
        }
        if let Some(value) = &self.init_containers {
            serde::ser::SerializeStruct::serialize_field(&mut state, "initContainers", value)?;
        }
        if let Some(value) = &self.node_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.node_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.priority {
            serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        if let Some(value) = &self.priority_class_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "priorityClassName", value)?;
        }
        if let Some(value) = &self.readiness_gates {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readinessGates", value)?;
        }
        if let Some(value) = &self.restart_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "restartPolicy", value)?;
        }
        if let Some(value) = &self.scheduler_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "schedulerName", value)?;
        }
        if let Some(value) = &self.security_context {
            serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.service_account {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccount", value)?;
        }
        if let Some(value) = &self.service_account_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountName", value)?;
        }
        if let Some(value) = &self.share_process_namespace {
            serde::ser::SerializeStruct::serialize_field(&mut state, "shareProcessNamespace", value)?;
        }
        if let Some(value) = &self.subdomain {
            serde::ser::SerializeStruct::serialize_field(&mut state, "subdomain", value)?;
        }
        if let Some(value) = &self.termination_grace_period_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "terminationGracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.tolerations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        if let Some(value) = &self.volumes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
