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

    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource. This field is alpha-level and is only honored by servers that enable the EphemeralContainers feature.
    pub ephemeral_containers: Option<Vec<crate::api::core::v1::EphemeralContainer>>,

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

    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    pub init_containers: Option<Vec<crate::api::core::v1::Container>>,

    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
    pub node_name: Option<String>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    pub node_selector: Option<std::collections::BTreeMap<String, String>>,

    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/20190226-pod-overhead.md This field is alpha-level as of Kubernetes v1.16, and is only honored by servers that enable the PodOverhead feature.
    pub overhead: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is alpha-level and is only honored by servers that enable the NonPreemptingPriority feature.
    pub preemption_policy: Option<String>,

    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    pub priority: Option<i32>,

    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    pub priority_class_name: Option<String>,

    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/0007-pod-ready%2B%2B.md
    pub readiness_gates: Option<Vec<crate::api::core::v1::PodReadinessGate>>,

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

    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
    pub share_process_namespace: Option<bool>,

    /// If specified, the fully qualified Pod hostname will be "\<hostname\>.\<subdomain\>.\<pod namespace\>.svc.\<cluster domain\>". If not specified, the pod will not have a domainname at all.
    pub subdomain: Option<String>,

    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    pub termination_grace_period_seconds: Option<i64>,

    /// If specified, the pod's tolerations.
    pub tolerations: Option<Vec<crate::api::core::v1::Toleration>>,

    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. This field is only honored by clusters that enable the EvenPodsSpread feature. All topologySpreadConstraints are ANDed.
    pub topology_spread_constraints: Option<Vec<crate::api::core::v1::TopologySpreadConstraint>>,

    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    pub volumes: Option<Vec<crate::api::core::v1::Volume>>,

}

#[cfg(feature = "dsl")]
impl PodSpec  {
    /// Set [`Self::active_deadline_seconds`]
    pub  fn active_deadline_seconds_set(&mut self, active_deadline_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.active_deadline_seconds = active_deadline_seconds.into(); self
    }

    pub  fn active_deadline_seconds(&mut self) -> &mut i64 {
        if self.active_deadline_seconds.is_none() { self.active_deadline_seconds = Some(Default::default()) }
        self.active_deadline_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::active_deadline_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn active_deadline_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.active_deadline_seconds.is_none() { self.active_deadline_seconds = Some(Default::default()) };
        func(self.active_deadline_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::affinity`]
    pub  fn affinity_set(&mut self, affinity: impl Into<Option<crate::api::core::v1::Affinity>>) -> &mut Self {
        self.affinity = affinity.into(); self
    }

    pub  fn affinity(&mut self) -> &mut crate::api::core::v1::Affinity {
        if self.affinity.is_none() { self.affinity = Some(Default::default()) }
        self.affinity.as_mut().unwrap()
    }

    /// Modify [`Self::affinity`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn affinity_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Affinity)) -> &mut Self {
        if self.affinity.is_none() { self.affinity = Some(Default::default()) };
        func(self.affinity.as_mut().unwrap()); self
    }


    /// Set [`Self::automount_service_account_token`]
    pub  fn automount_service_account_token_set(&mut self, automount_service_account_token: impl Into<Option<bool>>) -> &mut Self {
        self.automount_service_account_token = automount_service_account_token.into(); self
    }

    pub  fn automount_service_account_token(&mut self) -> &mut bool {
        if self.automount_service_account_token.is_none() { self.automount_service_account_token = Some(Default::default()) }
        self.automount_service_account_token.as_mut().unwrap()
    }

    /// Modify [`Self::automount_service_account_token`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn automount_service_account_token_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.automount_service_account_token.is_none() { self.automount_service_account_token = Some(Default::default()) };
        func(self.automount_service_account_token.as_mut().unwrap()); self
    }


    /// Set [`Self::containers`]
    pub  fn containers_set(&mut self, containers: impl Into<Vec<crate::api::core::v1::Container>>) -> &mut Self {
        self.containers = containers.into(); self
    }

    pub  fn containers(&mut self) -> &mut Vec<crate::api::core::v1::Container> {
        &mut self.containers
    }

    /// Modify [`Self::containers`] with a `func`
    pub  fn containers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Container>)) -> &mut Self {
        func(&mut self.containers); self
    }

    /// Push new element to [`Self::containers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn containers_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Container)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.containers.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::containers`]
    pub  fn containers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Container]>) -> &mut Self {
         for item in other.borrow() {
             self.containers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::dns_config`]
    pub  fn dns_config_set(&mut self, dns_config: impl Into<Option<crate::api::core::v1::PodDNSConfig>>) -> &mut Self {
        self.dns_config = dns_config.into(); self
    }

    pub  fn dns_config(&mut self) -> &mut crate::api::core::v1::PodDNSConfig {
        if self.dns_config.is_none() { self.dns_config = Some(Default::default()) }
        self.dns_config.as_mut().unwrap()
    }

    /// Modify [`Self::dns_config`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn dns_config_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodDNSConfig)) -> &mut Self {
        if self.dns_config.is_none() { self.dns_config = Some(Default::default()) };
        func(self.dns_config.as_mut().unwrap()); self
    }


    /// Set [`Self::dns_policy`]
    pub  fn dns_policy_set(&mut self, dns_policy: impl Into<Option<String>>) -> &mut Self {
        self.dns_policy = dns_policy.into(); self
    }

    pub  fn dns_policy(&mut self) -> &mut String {
        if self.dns_policy.is_none() { self.dns_policy = Some(Default::default()) }
        self.dns_policy.as_mut().unwrap()
    }

    /// Modify [`Self::dns_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn dns_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.dns_policy.is_none() { self.dns_policy = Some(Default::default()) };
        func(self.dns_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::enable_service_links`]
    pub  fn enable_service_links_set(&mut self, enable_service_links: impl Into<Option<bool>>) -> &mut Self {
        self.enable_service_links = enable_service_links.into(); self
    }

    pub  fn enable_service_links(&mut self) -> &mut bool {
        if self.enable_service_links.is_none() { self.enable_service_links = Some(Default::default()) }
        self.enable_service_links.as_mut().unwrap()
    }

    /// Modify [`Self::enable_service_links`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn enable_service_links_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.enable_service_links.is_none() { self.enable_service_links = Some(Default::default()) };
        func(self.enable_service_links.as_mut().unwrap()); self
    }


    /// Set [`Self::ephemeral_containers`]
    pub  fn ephemeral_containers_set(&mut self, ephemeral_containers: impl Into<Option<Vec<crate::api::core::v1::EphemeralContainer>>>) -> &mut Self {
        self.ephemeral_containers = ephemeral_containers.into(); self
    }

    pub  fn ephemeral_containers(&mut self) -> &mut Vec<crate::api::core::v1::EphemeralContainer> {
        if self.ephemeral_containers.is_none() { self.ephemeral_containers = Some(Default::default()) }
        self.ephemeral_containers.as_mut().unwrap()
    }

    /// Modify [`Self::ephemeral_containers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ephemeral_containers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EphemeralContainer>)) -> &mut Self {
        if self.ephemeral_containers.is_none() { self.ephemeral_containers = Some(Default::default()) };
        func(self.ephemeral_containers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ephemeral_containers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ephemeral_containers_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EphemeralContainer)) -> &mut Self {
        if self.ephemeral_containers.is_none() {
            self.ephemeral_containers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ephemeral_containers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ephemeral_containers`]
    pub  fn ephemeral_containers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EphemeralContainer]>) -> &mut Self {
         if self.ephemeral_containers.is_none() { self.ephemeral_containers = Some(Vec::new()); }
         let ephemeral_containers = &mut self.ephemeral_containers.as_mut().unwrap();
         for item in other.borrow() {
             ephemeral_containers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::host_aliases`]
    pub  fn host_aliases_set(&mut self, host_aliases: impl Into<Option<Vec<crate::api::core::v1::HostAlias>>>) -> &mut Self {
        self.host_aliases = host_aliases.into(); self
    }

    pub  fn host_aliases(&mut self) -> &mut Vec<crate::api::core::v1::HostAlias> {
        if self.host_aliases.is_none() { self.host_aliases = Some(Default::default()) }
        self.host_aliases.as_mut().unwrap()
    }

    /// Modify [`Self::host_aliases`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_aliases_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::HostAlias>)) -> &mut Self {
        if self.host_aliases.is_none() { self.host_aliases = Some(Default::default()) };
        func(self.host_aliases.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::host_aliases`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn host_aliases_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::HostAlias)) -> &mut Self {
        if self.host_aliases.is_none() {
            self.host_aliases = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.host_aliases.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::host_aliases`]
    pub  fn host_aliases_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::HostAlias]>) -> &mut Self {
         if self.host_aliases.is_none() { self.host_aliases = Some(Vec::new()); }
         let host_aliases = &mut self.host_aliases.as_mut().unwrap();
         for item in other.borrow() {
             host_aliases.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::host_ipc`]
    pub  fn host_ipc_set(&mut self, host_ipc: impl Into<Option<bool>>) -> &mut Self {
        self.host_ipc = host_ipc.into(); self
    }

    pub  fn host_ipc(&mut self) -> &mut bool {
        if self.host_ipc.is_none() { self.host_ipc = Some(Default::default()) }
        self.host_ipc.as_mut().unwrap()
    }

    /// Modify [`Self::host_ipc`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_ipc_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.host_ipc.is_none() { self.host_ipc = Some(Default::default()) };
        func(self.host_ipc.as_mut().unwrap()); self
    }


    /// Set [`Self::host_network`]
    pub  fn host_network_set(&mut self, host_network: impl Into<Option<bool>>) -> &mut Self {
        self.host_network = host_network.into(); self
    }

    pub  fn host_network(&mut self) -> &mut bool {
        if self.host_network.is_none() { self.host_network = Some(Default::default()) }
        self.host_network.as_mut().unwrap()
    }

    /// Modify [`Self::host_network`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_network_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.host_network.is_none() { self.host_network = Some(Default::default()) };
        func(self.host_network.as_mut().unwrap()); self
    }


    /// Set [`Self::host_pid`]
    pub  fn host_pid_set(&mut self, host_pid: impl Into<Option<bool>>) -> &mut Self {
        self.host_pid = host_pid.into(); self
    }

    pub  fn host_pid(&mut self) -> &mut bool {
        if self.host_pid.is_none() { self.host_pid = Some(Default::default()) }
        self.host_pid.as_mut().unwrap()
    }

    /// Modify [`Self::host_pid`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_pid_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.host_pid.is_none() { self.host_pid = Some(Default::default()) };
        func(self.host_pid.as_mut().unwrap()); self
    }


    /// Set [`Self::hostname`]
    pub  fn hostname_set(&mut self, hostname: impl Into<Option<String>>) -> &mut Self {
        self.hostname = hostname.into(); self
    }

    pub  fn hostname(&mut self) -> &mut String {
        if self.hostname.is_none() { self.hostname = Some(Default::default()) }
        self.hostname.as_mut().unwrap()
    }

    /// Modify [`Self::hostname`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn hostname_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.hostname.is_none() { self.hostname = Some(Default::default()) };
        func(self.hostname.as_mut().unwrap()); self
    }


    /// Set [`Self::image_pull_secrets`]
    pub  fn image_pull_secrets_set(&mut self, image_pull_secrets: impl Into<Option<Vec<crate::api::core::v1::LocalObjectReference>>>) -> &mut Self {
        self.image_pull_secrets = image_pull_secrets.into(); self
    }

    pub  fn image_pull_secrets(&mut self) -> &mut Vec<crate::api::core::v1::LocalObjectReference> {
        if self.image_pull_secrets.is_none() { self.image_pull_secrets = Some(Default::default()) }
        self.image_pull_secrets.as_mut().unwrap()
    }

    /// Modify [`Self::image_pull_secrets`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn image_pull_secrets_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::LocalObjectReference>)) -> &mut Self {
        if self.image_pull_secrets.is_none() { self.image_pull_secrets = Some(Default::default()) };
        func(self.image_pull_secrets.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::image_pull_secrets`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn image_pull_secrets_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LocalObjectReference)) -> &mut Self {
        if self.image_pull_secrets.is_none() {
            self.image_pull_secrets = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.image_pull_secrets.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::image_pull_secrets`]
    pub  fn image_pull_secrets_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::LocalObjectReference]>) -> &mut Self {
         if self.image_pull_secrets.is_none() { self.image_pull_secrets = Some(Vec::new()); }
         let image_pull_secrets = &mut self.image_pull_secrets.as_mut().unwrap();
         for item in other.borrow() {
             image_pull_secrets.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::init_containers`]
    pub  fn init_containers_set(&mut self, init_containers: impl Into<Option<Vec<crate::api::core::v1::Container>>>) -> &mut Self {
        self.init_containers = init_containers.into(); self
    }

    pub  fn init_containers(&mut self) -> &mut Vec<crate::api::core::v1::Container> {
        if self.init_containers.is_none() { self.init_containers = Some(Default::default()) }
        self.init_containers.as_mut().unwrap()
    }

    /// Modify [`Self::init_containers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn init_containers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Container>)) -> &mut Self {
        if self.init_containers.is_none() { self.init_containers = Some(Default::default()) };
        func(self.init_containers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::init_containers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn init_containers_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Container)) -> &mut Self {
        if self.init_containers.is_none() {
            self.init_containers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.init_containers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::init_containers`]
    pub  fn init_containers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Container]>) -> &mut Self {
         if self.init_containers.is_none() { self.init_containers = Some(Vec::new()); }
         let init_containers = &mut self.init_containers.as_mut().unwrap();
         for item in other.borrow() {
             init_containers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::node_name`]
    pub  fn node_name_set(&mut self, node_name: impl Into<Option<String>>) -> &mut Self {
        self.node_name = node_name.into(); self
    }

    pub  fn node_name(&mut self) -> &mut String {
        if self.node_name.is_none() { self.node_name = Some(Default::default()) }
        self.node_name.as_mut().unwrap()
    }

    /// Modify [`Self::node_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.node_name.is_none() { self.node_name = Some(Default::default()) };
        func(self.node_name.as_mut().unwrap()); self
    }


    /// Set [`Self::node_selector`]
    pub  fn node_selector_set(&mut self, node_selector: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.node_selector = node_selector.into(); self
    }

    pub  fn node_selector(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.node_selector.is_none() { self.node_selector = Some(Default::default()) }
        self.node_selector.as_mut().unwrap()
    }

    /// Modify [`Self::node_selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_selector_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.node_selector.is_none() { self.node_selector = Some(Default::default()) };
        func(self.node_selector.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::node_selector`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn node_selector_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.node_selector.is_none() {
            self.node_selector = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.node_selector.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::node_selector`]
    pub  fn node_selector_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.node_selector.is_none() { self.node_selector = Some(std::collections::BTreeMap::new()); }
         let node_selector = &mut self.node_selector.as_mut().unwrap();
         for (name, value) in other.borrow() {
             node_selector.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::overhead`]
    pub  fn overhead_set(&mut self, overhead: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.overhead = overhead.into(); self
    }

    pub  fn overhead(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.overhead.is_none() { self.overhead = Some(Default::default()) }
        self.overhead.as_mut().unwrap()
    }

    /// Modify [`Self::overhead`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn overhead_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.overhead.is_none() { self.overhead = Some(Default::default()) };
        func(self.overhead.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::overhead`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn overhead_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.overhead.is_none() {
            self.overhead = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.overhead.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::overhead`]
    pub  fn overhead_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.overhead.is_none() { self.overhead = Some(std::collections::BTreeMap::new()); }
         let overhead = &mut self.overhead.as_mut().unwrap();
         for (name, value) in other.borrow() {
             overhead.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::preemption_policy`]
    pub  fn preemption_policy_set(&mut self, preemption_policy: impl Into<Option<String>>) -> &mut Self {
        self.preemption_policy = preemption_policy.into(); self
    }

    pub  fn preemption_policy(&mut self) -> &mut String {
        if self.preemption_policy.is_none() { self.preemption_policy = Some(Default::default()) }
        self.preemption_policy.as_mut().unwrap()
    }

    /// Modify [`Self::preemption_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn preemption_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.preemption_policy.is_none() { self.preemption_policy = Some(Default::default()) };
        func(self.preemption_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::priority`]
    pub  fn priority_set(&mut self, priority: impl Into<Option<i32>>) -> &mut Self {
        self.priority = priority.into(); self
    }

    pub  fn priority(&mut self) -> &mut i32 {
        if self.priority.is_none() { self.priority = Some(Default::default()) }
        self.priority.as_mut().unwrap()
    }

    /// Modify [`Self::priority`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn priority_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.priority.is_none() { self.priority = Some(Default::default()) };
        func(self.priority.as_mut().unwrap()); self
    }


    /// Set [`Self::priority_class_name`]
    pub  fn priority_class_name_set(&mut self, priority_class_name: impl Into<Option<String>>) -> &mut Self {
        self.priority_class_name = priority_class_name.into(); self
    }

    pub  fn priority_class_name(&mut self) -> &mut String {
        if self.priority_class_name.is_none() { self.priority_class_name = Some(Default::default()) }
        self.priority_class_name.as_mut().unwrap()
    }

    /// Modify [`Self::priority_class_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn priority_class_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.priority_class_name.is_none() { self.priority_class_name = Some(Default::default()) };
        func(self.priority_class_name.as_mut().unwrap()); self
    }


    /// Set [`Self::readiness_gates`]
    pub  fn readiness_gates_set(&mut self, readiness_gates: impl Into<Option<Vec<crate::api::core::v1::PodReadinessGate>>>) -> &mut Self {
        self.readiness_gates = readiness_gates.into(); self
    }

    pub  fn readiness_gates(&mut self) -> &mut Vec<crate::api::core::v1::PodReadinessGate> {
        if self.readiness_gates.is_none() { self.readiness_gates = Some(Default::default()) }
        self.readiness_gates.as_mut().unwrap()
    }

    /// Modify [`Self::readiness_gates`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn readiness_gates_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PodReadinessGate>)) -> &mut Self {
        if self.readiness_gates.is_none() { self.readiness_gates = Some(Default::default()) };
        func(self.readiness_gates.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::readiness_gates`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn readiness_gates_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodReadinessGate)) -> &mut Self {
        if self.readiness_gates.is_none() {
            self.readiness_gates = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.readiness_gates.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::readiness_gates`]
    pub  fn readiness_gates_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PodReadinessGate]>) -> &mut Self {
         if self.readiness_gates.is_none() { self.readiness_gates = Some(Vec::new()); }
         let readiness_gates = &mut self.readiness_gates.as_mut().unwrap();
         for item in other.borrow() {
             readiness_gates.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::restart_policy`]
    pub  fn restart_policy_set(&mut self, restart_policy: impl Into<Option<String>>) -> &mut Self {
        self.restart_policy = restart_policy.into(); self
    }

    pub  fn restart_policy(&mut self) -> &mut String {
        if self.restart_policy.is_none() { self.restart_policy = Some(Default::default()) }
        self.restart_policy.as_mut().unwrap()
    }

    /// Modify [`Self::restart_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn restart_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.restart_policy.is_none() { self.restart_policy = Some(Default::default()) };
        func(self.restart_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::runtime_class_name`]
    pub  fn runtime_class_name_set(&mut self, runtime_class_name: impl Into<Option<String>>) -> &mut Self {
        self.runtime_class_name = runtime_class_name.into(); self
    }

    pub  fn runtime_class_name(&mut self) -> &mut String {
        if self.runtime_class_name.is_none() { self.runtime_class_name = Some(Default::default()) }
        self.runtime_class_name.as_mut().unwrap()
    }

    /// Modify [`Self::runtime_class_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn runtime_class_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.runtime_class_name.is_none() { self.runtime_class_name = Some(Default::default()) };
        func(self.runtime_class_name.as_mut().unwrap()); self
    }


    /// Set [`Self::scheduler_name`]
    pub  fn scheduler_name_set(&mut self, scheduler_name: impl Into<Option<String>>) -> &mut Self {
        self.scheduler_name = scheduler_name.into(); self
    }

    pub  fn scheduler_name(&mut self) -> &mut String {
        if self.scheduler_name.is_none() { self.scheduler_name = Some(Default::default()) }
        self.scheduler_name.as_mut().unwrap()
    }

    /// Modify [`Self::scheduler_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scheduler_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.scheduler_name.is_none() { self.scheduler_name = Some(Default::default()) };
        func(self.scheduler_name.as_mut().unwrap()); self
    }


    /// Set [`Self::security_context`]
    pub  fn security_context_set(&mut self, security_context: impl Into<Option<crate::api::core::v1::PodSecurityContext>>) -> &mut Self {
        self.security_context = security_context.into(); self
    }

    pub  fn security_context(&mut self) -> &mut crate::api::core::v1::PodSecurityContext {
        if self.security_context.is_none() { self.security_context = Some(Default::default()) }
        self.security_context.as_mut().unwrap()
    }

    /// Modify [`Self::security_context`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn security_context_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodSecurityContext)) -> &mut Self {
        if self.security_context.is_none() { self.security_context = Some(Default::default()) };
        func(self.security_context.as_mut().unwrap()); self
    }


    /// Set [`Self::service_account`]
    pub  fn service_account_set(&mut self, service_account: impl Into<Option<String>>) -> &mut Self {
        self.service_account = service_account.into(); self
    }

    pub  fn service_account(&mut self) -> &mut String {
        if self.service_account.is_none() { self.service_account = Some(Default::default()) }
        self.service_account.as_mut().unwrap()
    }

    /// Modify [`Self::service_account`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn service_account_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.service_account.is_none() { self.service_account = Some(Default::default()) };
        func(self.service_account.as_mut().unwrap()); self
    }


    /// Set [`Self::service_account_name`]
    pub  fn service_account_name_set(&mut self, service_account_name: impl Into<Option<String>>) -> &mut Self {
        self.service_account_name = service_account_name.into(); self
    }

    pub  fn service_account_name(&mut self) -> &mut String {
        if self.service_account_name.is_none() { self.service_account_name = Some(Default::default()) }
        self.service_account_name.as_mut().unwrap()
    }

    /// Modify [`Self::service_account_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn service_account_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.service_account_name.is_none() { self.service_account_name = Some(Default::default()) };
        func(self.service_account_name.as_mut().unwrap()); self
    }


    /// Set [`Self::share_process_namespace`]
    pub  fn share_process_namespace_set(&mut self, share_process_namespace: impl Into<Option<bool>>) -> &mut Self {
        self.share_process_namespace = share_process_namespace.into(); self
    }

    pub  fn share_process_namespace(&mut self) -> &mut bool {
        if self.share_process_namespace.is_none() { self.share_process_namespace = Some(Default::default()) }
        self.share_process_namespace.as_mut().unwrap()
    }

    /// Modify [`Self::share_process_namespace`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn share_process_namespace_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.share_process_namespace.is_none() { self.share_process_namespace = Some(Default::default()) };
        func(self.share_process_namespace.as_mut().unwrap()); self
    }


    /// Set [`Self::subdomain`]
    pub  fn subdomain_set(&mut self, subdomain: impl Into<Option<String>>) -> &mut Self {
        self.subdomain = subdomain.into(); self
    }

    pub  fn subdomain(&mut self) -> &mut String {
        if self.subdomain.is_none() { self.subdomain = Some(Default::default()) }
        self.subdomain.as_mut().unwrap()
    }

    /// Modify [`Self::subdomain`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn subdomain_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.subdomain.is_none() { self.subdomain = Some(Default::default()) };
        func(self.subdomain.as_mut().unwrap()); self
    }


    /// Set [`Self::termination_grace_period_seconds`]
    pub  fn termination_grace_period_seconds_set(&mut self, termination_grace_period_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.termination_grace_period_seconds = termination_grace_period_seconds.into(); self
    }

    pub  fn termination_grace_period_seconds(&mut self) -> &mut i64 {
        if self.termination_grace_period_seconds.is_none() { self.termination_grace_period_seconds = Some(Default::default()) }
        self.termination_grace_period_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::termination_grace_period_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn termination_grace_period_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.termination_grace_period_seconds.is_none() { self.termination_grace_period_seconds = Some(Default::default()) };
        func(self.termination_grace_period_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::tolerations`]
    pub  fn tolerations_set(&mut self, tolerations: impl Into<Option<Vec<crate::api::core::v1::Toleration>>>) -> &mut Self {
        self.tolerations = tolerations.into(); self
    }

    pub  fn tolerations(&mut self) -> &mut Vec<crate::api::core::v1::Toleration> {
        if self.tolerations.is_none() { self.tolerations = Some(Default::default()) }
        self.tolerations.as_mut().unwrap()
    }

    /// Modify [`Self::tolerations`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn tolerations_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Toleration>)) -> &mut Self {
        if self.tolerations.is_none() { self.tolerations = Some(Default::default()) };
        func(self.tolerations.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::tolerations`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn tolerations_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Toleration)) -> &mut Self {
        if self.tolerations.is_none() {
            self.tolerations = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.tolerations.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::tolerations`]
    pub  fn tolerations_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Toleration]>) -> &mut Self {
         if self.tolerations.is_none() { self.tolerations = Some(Vec::new()); }
         let tolerations = &mut self.tolerations.as_mut().unwrap();
         for item in other.borrow() {
             tolerations.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::topology_spread_constraints`]
    pub  fn topology_spread_constraints_set(&mut self, topology_spread_constraints: impl Into<Option<Vec<crate::api::core::v1::TopologySpreadConstraint>>>) -> &mut Self {
        self.topology_spread_constraints = topology_spread_constraints.into(); self
    }

    pub  fn topology_spread_constraints(&mut self) -> &mut Vec<crate::api::core::v1::TopologySpreadConstraint> {
        if self.topology_spread_constraints.is_none() { self.topology_spread_constraints = Some(Default::default()) }
        self.topology_spread_constraints.as_mut().unwrap()
    }

    /// Modify [`Self::topology_spread_constraints`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn topology_spread_constraints_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::TopologySpreadConstraint>)) -> &mut Self {
        if self.topology_spread_constraints.is_none() { self.topology_spread_constraints = Some(Default::default()) };
        func(self.topology_spread_constraints.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::topology_spread_constraints`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn topology_spread_constraints_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::TopologySpreadConstraint)) -> &mut Self {
        if self.topology_spread_constraints.is_none() {
            self.topology_spread_constraints = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.topology_spread_constraints.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::topology_spread_constraints`]
    pub  fn topology_spread_constraints_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::TopologySpreadConstraint]>) -> &mut Self {
         if self.topology_spread_constraints.is_none() { self.topology_spread_constraints = Some(Vec::new()); }
         let topology_spread_constraints = &mut self.topology_spread_constraints.as_mut().unwrap();
         for item in other.borrow() {
             topology_spread_constraints.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::volumes`]
    pub  fn volumes_set(&mut self, volumes: impl Into<Option<Vec<crate::api::core::v1::Volume>>>) -> &mut Self {
        self.volumes = volumes.into(); self
    }

    pub  fn volumes(&mut self) -> &mut Vec<crate::api::core::v1::Volume> {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) }
        self.volumes.as_mut().unwrap()
    }

    /// Modify [`Self::volumes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volumes_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Volume>)) -> &mut Self {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) };
        func(self.volumes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volumes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volumes_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Volume)) -> &mut Self {
        if self.volumes.is_none() {
            self.volumes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volumes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volumes`]
    pub  fn volumes_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Volume]>) -> &mut Self {
         if self.volumes.is_none() { self.volumes = Some(Vec::new()); }
         let volumes = &mut self.volumes.as_mut().unwrap();
         for item in other.borrow() {
             volumes.push(item.to_owned());
         }
         self
    }


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
            Key_ephemeral_containers,
            Key_host_aliases,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_hostname,
            Key_image_pull_secrets,
            Key_init_containers,
            Key_node_name,
            Key_node_selector,
            Key_overhead,
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
            Key_topology_spread_constraints,
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
                            "ephemeralContainers" => Field::Key_ephemeral_containers,
                            "hostAliases" => Field::Key_host_aliases,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostname" => Field::Key_hostname,
                            "imagePullSecrets" => Field::Key_image_pull_secrets,
                            "initContainers" => Field::Key_init_containers,
                            "nodeName" => Field::Key_node_name,
                            "nodeSelector" => Field::Key_node_selector,
                            "overhead" => Field::Key_overhead,
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
                            "topologySpreadConstraints" => Field::Key_topology_spread_constraints,
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
                let mut value_ephemeral_containers: Option<Vec<crate::api::core::v1::EphemeralContainer>> = None;
                let mut value_host_aliases: Option<Vec<crate::api::core::v1::HostAlias>> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_hostname: Option<String> = None;
                let mut value_image_pull_secrets: Option<Vec<crate::api::core::v1::LocalObjectReference>> = None;
                let mut value_init_containers: Option<Vec<crate::api::core::v1::Container>> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_overhead: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
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
                let mut value_topology_spread_constraints: Option<Vec<crate::api::core::v1::TopologySpreadConstraint>> = None;
                let mut value_volumes: Option<Vec<crate::api::core::v1::Volume>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active_deadline_seconds => value_active_deadline_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_affinity => value_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_automount_service_account_token => value_automount_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_containers => value_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_config => value_dns_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dns_policy => value_dns_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enable_service_links => value_enable_service_links = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ephemeral_containers => value_ephemeral_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_aliases => value_host_aliases = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ipc => value_host_ipc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_secrets => value_image_pull_secrets = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_init_containers => value_init_containers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_overhead => value_overhead = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                        Field::Key_topology_spread_constraints => value_topology_spread_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSpec {
                    active_deadline_seconds: value_active_deadline_seconds,
                    affinity: value_affinity,
                    automount_service_account_token: value_automount_service_account_token,
                    containers: value_containers.unwrap_or_default(),
                    dns_config: value_dns_config,
                    dns_policy: value_dns_policy,
                    enable_service_links: value_enable_service_links,
                    ephemeral_containers: value_ephemeral_containers,
                    host_aliases: value_host_aliases,
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    hostname: value_hostname,
                    image_pull_secrets: value_image_pull_secrets,
                    init_containers: value_init_containers,
                    node_name: value_node_name,
                    node_selector: value_node_selector,
                    overhead: value_overhead,
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    readiness_gates: value_readiness_gates,
                    restart_policy: value_restart_policy,
                    runtime_class_name: value_runtime_class_name,
                    scheduler_name: value_scheduler_name,
                    security_context: value_security_context,
                    service_account: value_service_account,
                    service_account_name: value_service_account_name,
                    share_process_namespace: value_share_process_namespace,
                    subdomain: value_subdomain,
                    termination_grace_period_seconds: value_termination_grace_period_seconds,
                    tolerations: value_tolerations,
                    topology_spread_constraints: value_topology_spread_constraints,
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
                "enableServiceLinks",
                "ephemeralContainers",
                "hostAliases",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostname",
                "imagePullSecrets",
                "initContainers",
                "nodeName",
                "nodeSelector",
                "overhead",
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
                "topologySpreadConstraints",
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
            self.ephemeral_containers.as_ref().map_or(0, |_| 1) +
            self.host_aliases.as_ref().map_or(0, |_| 1) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.image_pull_secrets.as_ref().map_or(0, |_| 1) +
            self.init_containers.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.overhead.as_ref().map_or(0, |_| 1) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.readiness_gates.as_ref().map_or(0, |_| 1) +
            self.restart_policy.as_ref().map_or(0, |_| 1) +
            self.runtime_class_name.as_ref().map_or(0, |_| 1) +
            self.scheduler_name.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.service_account.as_ref().map_or(0, |_| 1) +
            self.service_account_name.as_ref().map_or(0, |_| 1) +
            self.share_process_namespace.as_ref().map_or(0, |_| 1) +
            self.subdomain.as_ref().map_or(0, |_| 1) +
            self.termination_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1) +
            self.topology_spread_constraints.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.ephemeral_containers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ephemeralContainers", value)?;
        }
        if let Some(value) = &self.host_aliases {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostAliases", value)?;
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
        if let Some(value) = &self.image_pull_secrets {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullSecrets", value)?;
        }
        if let Some(value) = &self.init_containers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initContainers", value)?;
        }
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.overhead {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "overhead", value)?;
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
        if let Some(value) = &self.readiness_gates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readinessGates", value)?;
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
        if let Some(value) = &self.tolerations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        if let Some(value) = &self.topology_spread_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologySpreadConstraints", value)?;
        }
        if let Some(value) = &self.volumes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PodSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodSpec is a description of a pod.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "activeDeadlineSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "affinity".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Affinity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the pod's scheduling constraints".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "automountServiceAccountToken".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "containers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Container>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "dnsConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodDNSConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "dnsPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enableServiceLinks".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ephemeralContainers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource. This field is alpha-level and is only honored by servers that enable the EphemeralContainers feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EphemeralContainer>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostAliases".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::HostAlias>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostIPC".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Use the host's ipc namespace. Optional: Default to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostNetwork".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostPID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Use the host's pid namespace. Optional: Default to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostname".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "imagePullSecrets".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::LocalObjectReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "initContainers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Container>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeSelector".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "overhead".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/20190226-pod-overhead.md This field is alpha-level as of Kubernetes v1.16, and is only honored by servers that enable the PodOverhead feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "preemptionPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is alpha-level and is only honored by servers that enable the NonPreemptingPriority feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priority".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priorityClassName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, indicates the pod's priority. \"system-node-critical\" and \"system-cluster-critical\" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readinessGates".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to \"True\" More info: https://git.k8s.io/enhancements/keps/sig-network/0007-pod-ready%2B%2B.md".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PodReadinessGate>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "restartPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runtimeClassName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the \"legacy\" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/runtime-class.md This is a beta feature as of Kubernetes v1.14.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "schedulerName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "securityContext".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodSecurityContext>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "serviceAccount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serviceAccountName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shareProcessNamespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subdomain".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the fully qualified Pod hostname will be \"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>\". If not specified, the pod will not have a domainname at all.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "terminationGracePeriodSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tolerations".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the pod's tolerations.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Toleration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "topologySpreadConstraints".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. This field is only honored by clusters that enable the EvenPodsSpread feature. All topologySpreadConstraints are ANDed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::TopologySpreadConstraint>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Volume>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "containers".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
