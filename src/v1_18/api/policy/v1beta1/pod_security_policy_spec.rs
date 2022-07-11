// Generated from definition io.k8s.api.policy.v1beta1.PodSecurityPolicySpec

/// PodSecurityPolicySpec defines the policy enforced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicySpec {
    /// allowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
    pub allow_privilege_escalation: Option<bool>,

    /// AllowedCSIDrivers is a whitelist of inline CSI drivers that must be explicitly set to be embedded within a pod spec. An empty value indicates that any CSI driver can be used for inline ephemeral volumes. This is an alpha field, and is only honored if the API server enables the CSIInlineVolume feature gate.
    pub allowed_csi_drivers: Option<Vec<crate::api::policy::v1beta1::AllowedCSIDriver>>,

    /// allowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both allowedCapabilities and requiredDropCapabilities.
    pub allowed_capabilities: Option<Vec<String>>,

    /// allowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the "volumes" field.
    pub allowed_flex_volumes: Option<Vec<crate::api::policy::v1beta1::AllowedFlexVolume>>,

    /// allowedHostPaths is a white list of allowed host paths. Empty indicates that all host paths may be used.
    pub allowed_host_paths: Option<Vec<crate::api::policy::v1beta1::AllowedHostPath>>,

    /// AllowedProcMountTypes is a whitelist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled.
    pub allowed_proc_mount_types: Option<Vec<String>>,

    /// allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in "*" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to whitelist all allowed unsafe sysctls explicitly to avoid rejection.
    ///
    /// Examples: e.g. "foo/*" allows "foo/bar", "foo/baz", etc. e.g. "foo.*" allows "foo.bar", "foo.baz", etc.
    pub allowed_unsafe_sysctls: Option<Vec<String>>,

    /// defaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both defaultAddCapabilities and requiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the allowedCapabilities list.
    pub default_add_capabilities: Option<Vec<String>>,

    /// defaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.
    pub default_allow_privilege_escalation: Option<bool>,

    /// forbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none. Each entry is either a plain sysctl name or ends in "*" in which case it is considered as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.
    ///
    /// Examples: e.g. "foo/*" forbids "foo/bar", "foo/baz", etc. e.g. "foo.*" forbids "foo.bar", "foo.baz", etc.
    pub forbidden_sysctls: Option<Vec<String>>,

    /// fsGroup is the strategy that will dictate what fs group is used by the SecurityContext.
    pub fs_group: crate::api::policy::v1beta1::FSGroupStrategyOptions,

    /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
    pub host_ipc: Option<bool>,

    /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
    pub host_network: Option<bool>,

    /// hostPID determines if the policy allows the use of HostPID in the pod spec.
    pub host_pid: Option<bool>,

    /// hostPorts determines which host port ranges are allowed to be exposed.
    pub host_ports: Option<Vec<crate::api::policy::v1beta1::HostPortRange>>,

    /// privileged determines if a pod can request to be run as privileged.
    pub privileged: Option<bool>,

    /// readOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
    pub read_only_root_filesystem: Option<bool>,

    /// requiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
    pub required_drop_capabilities: Option<Vec<String>>,

    /// RunAsGroup is the strategy that will dictate the allowable RunAsGroup values that may be set. If this field is omitted, the pod's RunAsGroup can take any value. This field requires the RunAsGroup feature gate to be enabled.
    pub run_as_group: Option<crate::api::policy::v1beta1::RunAsGroupStrategyOptions>,

    /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
    pub run_as_user: crate::api::policy::v1beta1::RunAsUserStrategyOptions,

    /// runtimeClass is the strategy that will dictate the allowable RuntimeClasses for a pod. If this field is omitted, the pod's runtimeClassName field is unrestricted. Enforcement of this field depends on the RuntimeClass feature gate being enabled.
    pub runtime_class: Option<crate::api::policy::v1beta1::RuntimeClassStrategyOptions>,

    /// seLinux is the strategy that will dictate the allowable labels that may be set.
    pub se_linux: crate::api::policy::v1beta1::SELinuxStrategyOptions,

    /// supplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
    pub supplemental_groups: crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions,

    /// volumes is a white list of allowed volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'.
    pub volumes: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl PodSecurityPolicySpec  {
    /// Set [`Self::allow_privilege_escalation`]
    pub  fn allow_privilege_escalation_set(&mut self, allow_privilege_escalation: impl Into<Option<bool>>) -> &mut Self {
        self.allow_privilege_escalation = allow_privilege_escalation.into(); self
    }

    pub  fn allow_privilege_escalation(&mut self) -> &mut bool {
        if self.allow_privilege_escalation.is_none() { self.allow_privilege_escalation = Some(Default::default()) }
        self.allow_privilege_escalation.as_mut().unwrap()
    }

    /// Modify [`Self::allow_privilege_escalation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allow_privilege_escalation_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.allow_privilege_escalation.is_none() { self.allow_privilege_escalation = Some(Default::default()) };
        func(self.allow_privilege_escalation.as_mut().unwrap()); self
    }


    /// Set [`Self::allowed_csi_drivers`]
    pub  fn allowed_csi_drivers_set(&mut self, allowed_csi_drivers: impl Into<Option<Vec<crate::api::policy::v1beta1::AllowedCSIDriver>>>) -> &mut Self {
        self.allowed_csi_drivers = allowed_csi_drivers.into(); self
    }

    pub  fn allowed_csi_drivers(&mut self) -> &mut Vec<crate::api::policy::v1beta1::AllowedCSIDriver> {
        if self.allowed_csi_drivers.is_none() { self.allowed_csi_drivers = Some(Default::default()) }
        self.allowed_csi_drivers.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_csi_drivers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_csi_drivers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::policy::v1beta1::AllowedCSIDriver>)) -> &mut Self {
        if self.allowed_csi_drivers.is_none() { self.allowed_csi_drivers = Some(Default::default()) };
        func(self.allowed_csi_drivers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_csi_drivers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_csi_drivers_push_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::AllowedCSIDriver)) -> &mut Self {
        if self.allowed_csi_drivers.is_none() {
            self.allowed_csi_drivers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_csi_drivers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_csi_drivers`]
    pub  fn allowed_csi_drivers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::policy::v1beta1::AllowedCSIDriver]>) -> &mut Self {
         if self.allowed_csi_drivers.is_none() { self.allowed_csi_drivers = Some(Vec::new()); }
         let allowed_csi_drivers = &mut self.allowed_csi_drivers.as_mut().unwrap();
         for item in other.borrow() {
             allowed_csi_drivers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allowed_capabilities`]
    pub  fn allowed_capabilities_set(&mut self, allowed_capabilities: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.allowed_capabilities = allowed_capabilities.into(); self
    }

    pub  fn allowed_capabilities(&mut self) -> &mut Vec<String> {
        if self.allowed_capabilities.is_none() { self.allowed_capabilities = Some(Default::default()) }
        self.allowed_capabilities.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_capabilities`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_capabilities_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.allowed_capabilities.is_none() { self.allowed_capabilities = Some(Default::default()) };
        func(self.allowed_capabilities.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_capabilities`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_capabilities_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.allowed_capabilities.is_none() {
            self.allowed_capabilities = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_capabilities.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_capabilities`]
    pub  fn allowed_capabilities_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.allowed_capabilities.is_none() { self.allowed_capabilities = Some(Vec::new()); }
         let allowed_capabilities = &mut self.allowed_capabilities.as_mut().unwrap();
         for item in other.borrow() {
             allowed_capabilities.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allowed_flex_volumes`]
    pub  fn allowed_flex_volumes_set(&mut self, allowed_flex_volumes: impl Into<Option<Vec<crate::api::policy::v1beta1::AllowedFlexVolume>>>) -> &mut Self {
        self.allowed_flex_volumes = allowed_flex_volumes.into(); self
    }

    pub  fn allowed_flex_volumes(&mut self) -> &mut Vec<crate::api::policy::v1beta1::AllowedFlexVolume> {
        if self.allowed_flex_volumes.is_none() { self.allowed_flex_volumes = Some(Default::default()) }
        self.allowed_flex_volumes.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_flex_volumes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_flex_volumes_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::policy::v1beta1::AllowedFlexVolume>)) -> &mut Self {
        if self.allowed_flex_volumes.is_none() { self.allowed_flex_volumes = Some(Default::default()) };
        func(self.allowed_flex_volumes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_flex_volumes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_flex_volumes_push_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::AllowedFlexVolume)) -> &mut Self {
        if self.allowed_flex_volumes.is_none() {
            self.allowed_flex_volumes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_flex_volumes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_flex_volumes`]
    pub  fn allowed_flex_volumes_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::policy::v1beta1::AllowedFlexVolume]>) -> &mut Self {
         if self.allowed_flex_volumes.is_none() { self.allowed_flex_volumes = Some(Vec::new()); }
         let allowed_flex_volumes = &mut self.allowed_flex_volumes.as_mut().unwrap();
         for item in other.borrow() {
             allowed_flex_volumes.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allowed_host_paths`]
    pub  fn allowed_host_paths_set(&mut self, allowed_host_paths: impl Into<Option<Vec<crate::api::policy::v1beta1::AllowedHostPath>>>) -> &mut Self {
        self.allowed_host_paths = allowed_host_paths.into(); self
    }

    pub  fn allowed_host_paths(&mut self) -> &mut Vec<crate::api::policy::v1beta1::AllowedHostPath> {
        if self.allowed_host_paths.is_none() { self.allowed_host_paths = Some(Default::default()) }
        self.allowed_host_paths.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_host_paths`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_host_paths_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::policy::v1beta1::AllowedHostPath>)) -> &mut Self {
        if self.allowed_host_paths.is_none() { self.allowed_host_paths = Some(Default::default()) };
        func(self.allowed_host_paths.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_host_paths`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_host_paths_push_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::AllowedHostPath)) -> &mut Self {
        if self.allowed_host_paths.is_none() {
            self.allowed_host_paths = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_host_paths.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_host_paths`]
    pub  fn allowed_host_paths_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::policy::v1beta1::AllowedHostPath]>) -> &mut Self {
         if self.allowed_host_paths.is_none() { self.allowed_host_paths = Some(Vec::new()); }
         let allowed_host_paths = &mut self.allowed_host_paths.as_mut().unwrap();
         for item in other.borrow() {
             allowed_host_paths.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allowed_proc_mount_types`]
    pub  fn allowed_proc_mount_types_set(&mut self, allowed_proc_mount_types: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.allowed_proc_mount_types = allowed_proc_mount_types.into(); self
    }

    pub  fn allowed_proc_mount_types(&mut self) -> &mut Vec<String> {
        if self.allowed_proc_mount_types.is_none() { self.allowed_proc_mount_types = Some(Default::default()) }
        self.allowed_proc_mount_types.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_proc_mount_types`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_proc_mount_types_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.allowed_proc_mount_types.is_none() { self.allowed_proc_mount_types = Some(Default::default()) };
        func(self.allowed_proc_mount_types.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_proc_mount_types`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_proc_mount_types_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.allowed_proc_mount_types.is_none() {
            self.allowed_proc_mount_types = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_proc_mount_types.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_proc_mount_types`]
    pub  fn allowed_proc_mount_types_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.allowed_proc_mount_types.is_none() { self.allowed_proc_mount_types = Some(Vec::new()); }
         let allowed_proc_mount_types = &mut self.allowed_proc_mount_types.as_mut().unwrap();
         for item in other.borrow() {
             allowed_proc_mount_types.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allowed_unsafe_sysctls`]
    pub  fn allowed_unsafe_sysctls_set(&mut self, allowed_unsafe_sysctls: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.allowed_unsafe_sysctls = allowed_unsafe_sysctls.into(); self
    }

    pub  fn allowed_unsafe_sysctls(&mut self) -> &mut Vec<String> {
        if self.allowed_unsafe_sysctls.is_none() { self.allowed_unsafe_sysctls = Some(Default::default()) }
        self.allowed_unsafe_sysctls.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_unsafe_sysctls`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_unsafe_sysctls_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.allowed_unsafe_sysctls.is_none() { self.allowed_unsafe_sysctls = Some(Default::default()) };
        func(self.allowed_unsafe_sysctls.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_unsafe_sysctls`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_unsafe_sysctls_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.allowed_unsafe_sysctls.is_none() {
            self.allowed_unsafe_sysctls = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_unsafe_sysctls.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_unsafe_sysctls`]
    pub  fn allowed_unsafe_sysctls_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.allowed_unsafe_sysctls.is_none() { self.allowed_unsafe_sysctls = Some(Vec::new()); }
         let allowed_unsafe_sysctls = &mut self.allowed_unsafe_sysctls.as_mut().unwrap();
         for item in other.borrow() {
             allowed_unsafe_sysctls.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::default_add_capabilities`]
    pub  fn default_add_capabilities_set(&mut self, default_add_capabilities: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.default_add_capabilities = default_add_capabilities.into(); self
    }

    pub  fn default_add_capabilities(&mut self) -> &mut Vec<String> {
        if self.default_add_capabilities.is_none() { self.default_add_capabilities = Some(Default::default()) }
        self.default_add_capabilities.as_mut().unwrap()
    }

    /// Modify [`Self::default_add_capabilities`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_add_capabilities_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.default_add_capabilities.is_none() { self.default_add_capabilities = Some(Default::default()) };
        func(self.default_add_capabilities.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::default_add_capabilities`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn default_add_capabilities_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.default_add_capabilities.is_none() {
            self.default_add_capabilities = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.default_add_capabilities.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::default_add_capabilities`]
    pub  fn default_add_capabilities_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.default_add_capabilities.is_none() { self.default_add_capabilities = Some(Vec::new()); }
         let default_add_capabilities = &mut self.default_add_capabilities.as_mut().unwrap();
         for item in other.borrow() {
             default_add_capabilities.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::default_allow_privilege_escalation`]
    pub  fn default_allow_privilege_escalation_set(&mut self, default_allow_privilege_escalation: impl Into<Option<bool>>) -> &mut Self {
        self.default_allow_privilege_escalation = default_allow_privilege_escalation.into(); self
    }

    pub  fn default_allow_privilege_escalation(&mut self) -> &mut bool {
        if self.default_allow_privilege_escalation.is_none() { self.default_allow_privilege_escalation = Some(Default::default()) }
        self.default_allow_privilege_escalation.as_mut().unwrap()
    }

    /// Modify [`Self::default_allow_privilege_escalation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_allow_privilege_escalation_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.default_allow_privilege_escalation.is_none() { self.default_allow_privilege_escalation = Some(Default::default()) };
        func(self.default_allow_privilege_escalation.as_mut().unwrap()); self
    }


    /// Set [`Self::forbidden_sysctls`]
    pub  fn forbidden_sysctls_set(&mut self, forbidden_sysctls: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.forbidden_sysctls = forbidden_sysctls.into(); self
    }

    pub  fn forbidden_sysctls(&mut self) -> &mut Vec<String> {
        if self.forbidden_sysctls.is_none() { self.forbidden_sysctls = Some(Default::default()) }
        self.forbidden_sysctls.as_mut().unwrap()
    }

    /// Modify [`Self::forbidden_sysctls`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn forbidden_sysctls_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.forbidden_sysctls.is_none() { self.forbidden_sysctls = Some(Default::default()) };
        func(self.forbidden_sysctls.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::forbidden_sysctls`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn forbidden_sysctls_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.forbidden_sysctls.is_none() {
            self.forbidden_sysctls = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.forbidden_sysctls.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::forbidden_sysctls`]
    pub  fn forbidden_sysctls_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.forbidden_sysctls.is_none() { self.forbidden_sysctls = Some(Vec::new()); }
         let forbidden_sysctls = &mut self.forbidden_sysctls.as_mut().unwrap();
         for item in other.borrow() {
             forbidden_sysctls.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::fs_group`]
    pub  fn fs_group_set(&mut self, fs_group: impl Into<crate::api::policy::v1beta1::FSGroupStrategyOptions>) -> &mut Self {
        self.fs_group = fs_group.into(); self
    }

    pub  fn fs_group(&mut self) -> &mut crate::api::policy::v1beta1::FSGroupStrategyOptions {
        &mut self.fs_group
    }

    /// Modify [`Self::fs_group`] with a `func`
    pub  fn fs_group_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::FSGroupStrategyOptions)) -> &mut Self {
        func(&mut self.fs_group); self
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


    /// Set [`Self::host_ports`]
    pub  fn host_ports_set(&mut self, host_ports: impl Into<Option<Vec<crate::api::policy::v1beta1::HostPortRange>>>) -> &mut Self {
        self.host_ports = host_ports.into(); self
    }

    pub  fn host_ports(&mut self) -> &mut Vec<crate::api::policy::v1beta1::HostPortRange> {
        if self.host_ports.is_none() { self.host_ports = Some(Default::default()) }
        self.host_ports.as_mut().unwrap()
    }

    /// Modify [`Self::host_ports`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_ports_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::policy::v1beta1::HostPortRange>)) -> &mut Self {
        if self.host_ports.is_none() { self.host_ports = Some(Default::default()) };
        func(self.host_ports.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::host_ports`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn host_ports_push_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::HostPortRange)) -> &mut Self {
        if self.host_ports.is_none() {
            self.host_ports = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.host_ports.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::host_ports`]
    pub  fn host_ports_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::policy::v1beta1::HostPortRange]>) -> &mut Self {
         if self.host_ports.is_none() { self.host_ports = Some(Vec::new()); }
         let host_ports = &mut self.host_ports.as_mut().unwrap();
         for item in other.borrow() {
             host_ports.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::privileged`]
    pub  fn privileged_set(&mut self, privileged: impl Into<Option<bool>>) -> &mut Self {
        self.privileged = privileged.into(); self
    }

    pub  fn privileged(&mut self) -> &mut bool {
        if self.privileged.is_none() { self.privileged = Some(Default::default()) }
        self.privileged.as_mut().unwrap()
    }

    /// Modify [`Self::privileged`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn privileged_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.privileged.is_none() { self.privileged = Some(Default::default()) };
        func(self.privileged.as_mut().unwrap()); self
    }


    /// Set [`Self::read_only_root_filesystem`]
    pub  fn read_only_root_filesystem_set(&mut self, read_only_root_filesystem: impl Into<Option<bool>>) -> &mut Self {
        self.read_only_root_filesystem = read_only_root_filesystem.into(); self
    }

    pub  fn read_only_root_filesystem(&mut self) -> &mut bool {
        if self.read_only_root_filesystem.is_none() { self.read_only_root_filesystem = Some(Default::default()) }
        self.read_only_root_filesystem.as_mut().unwrap()
    }

    /// Modify [`Self::read_only_root_filesystem`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn read_only_root_filesystem_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.read_only_root_filesystem.is_none() { self.read_only_root_filesystem = Some(Default::default()) };
        func(self.read_only_root_filesystem.as_mut().unwrap()); self
    }


    /// Set [`Self::required_drop_capabilities`]
    pub  fn required_drop_capabilities_set(&mut self, required_drop_capabilities: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.required_drop_capabilities = required_drop_capabilities.into(); self
    }

    pub  fn required_drop_capabilities(&mut self) -> &mut Vec<String> {
        if self.required_drop_capabilities.is_none() { self.required_drop_capabilities = Some(Default::default()) }
        self.required_drop_capabilities.as_mut().unwrap()
    }

    /// Modify [`Self::required_drop_capabilities`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn required_drop_capabilities_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.required_drop_capabilities.is_none() { self.required_drop_capabilities = Some(Default::default()) };
        func(self.required_drop_capabilities.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::required_drop_capabilities`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn required_drop_capabilities_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.required_drop_capabilities.is_none() {
            self.required_drop_capabilities = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.required_drop_capabilities.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::required_drop_capabilities`]
    pub  fn required_drop_capabilities_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.required_drop_capabilities.is_none() { self.required_drop_capabilities = Some(Vec::new()); }
         let required_drop_capabilities = &mut self.required_drop_capabilities.as_mut().unwrap();
         for item in other.borrow() {
             required_drop_capabilities.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::run_as_group`]
    pub  fn run_as_group_set(&mut self, run_as_group: impl Into<Option<crate::api::policy::v1beta1::RunAsGroupStrategyOptions>>) -> &mut Self {
        self.run_as_group = run_as_group.into(); self
    }

    pub  fn run_as_group(&mut self) -> &mut crate::api::policy::v1beta1::RunAsGroupStrategyOptions {
        if self.run_as_group.is_none() { self.run_as_group = Some(Default::default()) }
        self.run_as_group.as_mut().unwrap()
    }

    /// Modify [`Self::run_as_group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn run_as_group_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::RunAsGroupStrategyOptions)) -> &mut Self {
        if self.run_as_group.is_none() { self.run_as_group = Some(Default::default()) };
        func(self.run_as_group.as_mut().unwrap()); self
    }


    /// Set [`Self::run_as_user`]
    pub  fn run_as_user_set(&mut self, run_as_user: impl Into<crate::api::policy::v1beta1::RunAsUserStrategyOptions>) -> &mut Self {
        self.run_as_user = run_as_user.into(); self
    }

    pub  fn run_as_user(&mut self) -> &mut crate::api::policy::v1beta1::RunAsUserStrategyOptions {
        &mut self.run_as_user
    }

    /// Modify [`Self::run_as_user`] with a `func`
    pub  fn run_as_user_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::RunAsUserStrategyOptions)) -> &mut Self {
        func(&mut self.run_as_user); self
    }


    /// Set [`Self::runtime_class`]
    pub  fn runtime_class_set(&mut self, runtime_class: impl Into<Option<crate::api::policy::v1beta1::RuntimeClassStrategyOptions>>) -> &mut Self {
        self.runtime_class = runtime_class.into(); self
    }

    pub  fn runtime_class(&mut self) -> &mut crate::api::policy::v1beta1::RuntimeClassStrategyOptions {
        if self.runtime_class.is_none() { self.runtime_class = Some(Default::default()) }
        self.runtime_class.as_mut().unwrap()
    }

    /// Modify [`Self::runtime_class`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn runtime_class_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::RuntimeClassStrategyOptions)) -> &mut Self {
        if self.runtime_class.is_none() { self.runtime_class = Some(Default::default()) };
        func(self.runtime_class.as_mut().unwrap()); self
    }


    /// Set [`Self::se_linux`]
    pub  fn se_linux_set(&mut self, se_linux: impl Into<crate::api::policy::v1beta1::SELinuxStrategyOptions>) -> &mut Self {
        self.se_linux = se_linux.into(); self
    }

    pub  fn se_linux(&mut self) -> &mut crate::api::policy::v1beta1::SELinuxStrategyOptions {
        &mut self.se_linux
    }

    /// Modify [`Self::se_linux`] with a `func`
    pub  fn se_linux_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::SELinuxStrategyOptions)) -> &mut Self {
        func(&mut self.se_linux); self
    }


    /// Set [`Self::supplemental_groups`]
    pub  fn supplemental_groups_set(&mut self, supplemental_groups: impl Into<crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions>) -> &mut Self {
        self.supplemental_groups = supplemental_groups.into(); self
    }

    pub  fn supplemental_groups(&mut self) -> &mut crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions {
        &mut self.supplemental_groups
    }

    /// Modify [`Self::supplemental_groups`] with a `func`
    pub  fn supplemental_groups_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions)) -> &mut Self {
        func(&mut self.supplemental_groups); self
    }


    /// Set [`Self::volumes`]
    pub  fn volumes_set(&mut self, volumes: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.volumes = volumes.into(); self
    }

    pub  fn volumes(&mut self) -> &mut Vec<String> {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) }
        self.volumes.as_mut().unwrap()
    }

    /// Modify [`Self::volumes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volumes_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) };
        func(self.volumes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volumes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volumes_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volumes.is_none() {
            self.volumes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volumes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volumes`]
    pub  fn volumes_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.volumes.is_none() { self.volumes = Some(Vec::new()); }
         let volumes = &mut self.volumes.as_mut().unwrap();
         for item in other.borrow() {
             volumes.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PodSecurityPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_privilege_escalation,
            Key_allowed_csi_drivers,
            Key_allowed_capabilities,
            Key_allowed_flex_volumes,
            Key_allowed_host_paths,
            Key_allowed_proc_mount_types,
            Key_allowed_unsafe_sysctls,
            Key_default_add_capabilities,
            Key_default_allow_privilege_escalation,
            Key_forbidden_sysctls,
            Key_fs_group,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_host_ports,
            Key_privileged,
            Key_read_only_root_filesystem,
            Key_required_drop_capabilities,
            Key_run_as_group,
            Key_run_as_user,
            Key_runtime_class,
            Key_se_linux,
            Key_supplemental_groups,
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
                            "allowPrivilegeEscalation" => Field::Key_allow_privilege_escalation,
                            "allowedCSIDrivers" => Field::Key_allowed_csi_drivers,
                            "allowedCapabilities" => Field::Key_allowed_capabilities,
                            "allowedFlexVolumes" => Field::Key_allowed_flex_volumes,
                            "allowedHostPaths" => Field::Key_allowed_host_paths,
                            "allowedProcMountTypes" => Field::Key_allowed_proc_mount_types,
                            "allowedUnsafeSysctls" => Field::Key_allowed_unsafe_sysctls,
                            "defaultAddCapabilities" => Field::Key_default_add_capabilities,
                            "defaultAllowPrivilegeEscalation" => Field::Key_default_allow_privilege_escalation,
                            "forbiddenSysctls" => Field::Key_forbidden_sysctls,
                            "fsGroup" => Field::Key_fs_group,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostPorts" => Field::Key_host_ports,
                            "privileged" => Field::Key_privileged,
                            "readOnlyRootFilesystem" => Field::Key_read_only_root_filesystem,
                            "requiredDropCapabilities" => Field::Key_required_drop_capabilities,
                            "runAsGroup" => Field::Key_run_as_group,
                            "runAsUser" => Field::Key_run_as_user,
                            "runtimeClass" => Field::Key_runtime_class,
                            "seLinux" => Field::Key_se_linux,
                            "supplementalGroups" => Field::Key_supplemental_groups,
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
            type Value = PodSecurityPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_allowed_csi_drivers: Option<Vec<crate::api::policy::v1beta1::AllowedCSIDriver>> = None;
                let mut value_allowed_capabilities: Option<Vec<String>> = None;
                let mut value_allowed_flex_volumes: Option<Vec<crate::api::policy::v1beta1::AllowedFlexVolume>> = None;
                let mut value_allowed_host_paths: Option<Vec<crate::api::policy::v1beta1::AllowedHostPath>> = None;
                let mut value_allowed_proc_mount_types: Option<Vec<String>> = None;
                let mut value_allowed_unsafe_sysctls: Option<Vec<String>> = None;
                let mut value_default_add_capabilities: Option<Vec<String>> = None;
                let mut value_default_allow_privilege_escalation: Option<bool> = None;
                let mut value_forbidden_sysctls: Option<Vec<String>> = None;
                let mut value_fs_group: Option<crate::api::policy::v1beta1::FSGroupStrategyOptions> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_host_ports: Option<Vec<crate::api::policy::v1beta1::HostPortRange>> = None;
                let mut value_privileged: Option<bool> = None;
                let mut value_read_only_root_filesystem: Option<bool> = None;
                let mut value_required_drop_capabilities: Option<Vec<String>> = None;
                let mut value_run_as_group: Option<crate::api::policy::v1beta1::RunAsGroupStrategyOptions> = None;
                let mut value_run_as_user: Option<crate::api::policy::v1beta1::RunAsUserStrategyOptions> = None;
                let mut value_runtime_class: Option<crate::api::policy::v1beta1::RuntimeClassStrategyOptions> = None;
                let mut value_se_linux: Option<crate::api::policy::v1beta1::SELinuxStrategyOptions> = None;
                let mut value_supplemental_groups: Option<crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions> = None;
                let mut value_volumes: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_privilege_escalation => value_allow_privilege_escalation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_csi_drivers => value_allowed_csi_drivers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_capabilities => value_allowed_capabilities = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_flex_volumes => value_allowed_flex_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_host_paths => value_allowed_host_paths = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_proc_mount_types => value_allowed_proc_mount_types = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_unsafe_sysctls => value_allowed_unsafe_sysctls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_add_capabilities => value_default_add_capabilities = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_allow_privilege_escalation => value_default_allow_privilege_escalation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_forbidden_sysctls => value_forbidden_sysctls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group => value_fs_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ipc => value_host_ipc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ports => value_host_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_privileged => value_privileged = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only_root_filesystem => value_read_only_root_filesystem = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required_drop_capabilities => value_required_drop_capabilities = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_group => value_run_as_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_runtime_class => value_runtime_class = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux => value_se_linux = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups => value_supplemental_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicySpec {
                    allow_privilege_escalation: value_allow_privilege_escalation,
                    allowed_csi_drivers: value_allowed_csi_drivers,
                    allowed_capabilities: value_allowed_capabilities,
                    allowed_flex_volumes: value_allowed_flex_volumes,
                    allowed_host_paths: value_allowed_host_paths,
                    allowed_proc_mount_types: value_allowed_proc_mount_types,
                    allowed_unsafe_sysctls: value_allowed_unsafe_sysctls,
                    default_add_capabilities: value_default_add_capabilities,
                    default_allow_privilege_escalation: value_default_allow_privilege_escalation,
                    forbidden_sysctls: value_forbidden_sysctls,
                    fs_group: value_fs_group.unwrap_or_default(),
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    host_ports: value_host_ports,
                    privileged: value_privileged,
                    read_only_root_filesystem: value_read_only_root_filesystem,
                    required_drop_capabilities: value_required_drop_capabilities,
                    run_as_group: value_run_as_group,
                    run_as_user: value_run_as_user.unwrap_or_default(),
                    runtime_class: value_runtime_class,
                    se_linux: value_se_linux.unwrap_or_default(),
                    supplemental_groups: value_supplemental_groups.unwrap_or_default(),
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicySpec",
            &[
                "allowPrivilegeEscalation",
                "allowedCSIDrivers",
                "allowedCapabilities",
                "allowedFlexVolumes",
                "allowedHostPaths",
                "allowedProcMountTypes",
                "allowedUnsafeSysctls",
                "defaultAddCapabilities",
                "defaultAllowPrivilegeEscalation",
                "forbiddenSysctls",
                "fsGroup",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostPorts",
                "privileged",
                "readOnlyRootFilesystem",
                "requiredDropCapabilities",
                "runAsGroup",
                "runAsUser",
                "runtimeClass",
                "seLinux",
                "supplementalGroups",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSecurityPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicySpec",
            4 +
            self.allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.allowed_csi_drivers.as_ref().map_or(0, |_| 1) +
            self.allowed_capabilities.as_ref().map_or(0, |_| 1) +
            self.allowed_flex_volumes.as_ref().map_or(0, |_| 1) +
            self.allowed_host_paths.as_ref().map_or(0, |_| 1) +
            self.allowed_proc_mount_types.as_ref().map_or(0, |_| 1) +
            self.allowed_unsafe_sysctls.as_ref().map_or(0, |_| 1) +
            self.default_add_capabilities.as_ref().map_or(0, |_| 1) +
            self.default_allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.forbidden_sysctls.as_ref().map_or(0, |_| 1) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.host_ports.as_ref().map_or(0, |_| 1) +
            self.privileged.as_ref().map_or(0, |_| 1) +
            self.read_only_root_filesystem.as_ref().map_or(0, |_| 1) +
            self.required_drop_capabilities.as_ref().map_or(0, |_| 1) +
            self.run_as_group.as_ref().map_or(0, |_| 1) +
            self.runtime_class.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_privilege_escalation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.allowed_csi_drivers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedCSIDrivers", value)?;
        }
        if let Some(value) = &self.allowed_capabilities {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedCapabilities", value)?;
        }
        if let Some(value) = &self.allowed_flex_volumes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedFlexVolumes", value)?;
        }
        if let Some(value) = &self.allowed_host_paths {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedHostPaths", value)?;
        }
        if let Some(value) = &self.allowed_proc_mount_types {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedProcMountTypes", value)?;
        }
        if let Some(value) = &self.allowed_unsafe_sysctls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedUnsafeSysctls", value)?;
        }
        if let Some(value) = &self.default_add_capabilities {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAddCapabilities", value)?;
        }
        if let Some(value) = &self.default_allow_privilege_escalation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAllowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.forbidden_sysctls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "forbiddenSysctls", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroup", &self.fs_group)?;
        if let Some(value) = &self.host_ipc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPC", value)?;
        }
        if let Some(value) = &self.host_network {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostNetwork", value)?;
        }
        if let Some(value) = &self.host_pid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPID", value)?;
        }
        if let Some(value) = &self.host_ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPorts", value)?;
        }
        if let Some(value) = &self.privileged {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "privileged", value)?;
        }
        if let Some(value) = &self.read_only_root_filesystem {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnlyRootFilesystem", value)?;
        }
        if let Some(value) = &self.required_drop_capabilities {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDropCapabilities", value)?;
        }
        if let Some(value) = &self.run_as_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsGroup", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUser", &self.run_as_user)?;
        if let Some(value) = &self.runtime_class {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runtimeClass", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "seLinux", &self.se_linux)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", &self.supplemental_groups)?;
        if let Some(value) = &self.volumes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSecurityPolicySpec {
    fn schema_name() -> String {
        "io.k8s.api.policy.v1beta1.PodSecurityPolicySpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodSecurityPolicySpec defines the policy enforced.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowPrivilegeEscalation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedCSIDrivers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllowedCSIDrivers is a whitelist of inline CSI drivers that must be explicitly set to be embedded within a pod spec. An empty value indicates that any CSI driver can be used for inline ephemeral volumes. This is an alpha field, and is only honored if the API server enables the CSIInlineVolume feature gate.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::policy::v1beta1::AllowedCSIDriver>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedCapabilities".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both allowedCapabilities and requiredDropCapabilities.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedFlexVolumes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the \"volumes\" field.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::policy::v1beta1::AllowedFlexVolume>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedHostPaths".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowedHostPaths is a white list of allowed host paths. Empty indicates that all host paths may be used.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::policy::v1beta1::AllowedHostPath>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedProcMountTypes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllowedProcMountTypes is a whitelist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedUnsafeSysctls".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to whitelist all allowed unsafe sysctls explicitly to avoid rejection.\n\nExamples: e.g. \"foo/*\" allows \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" allows \"foo.bar\", \"foo.baz\", etc.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "defaultAddCapabilities".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("defaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both defaultAddCapabilities and requiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the allowedCapabilities list.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "defaultAllowPrivilegeEscalation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("defaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "forbiddenSysctls".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("forbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.\n\nExamples: e.g. \"foo/*\" forbids \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" forbids \"foo.bar\", \"foo.baz\", etc.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsGroup".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::FSGroupStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsGroup is the strategy that will dictate what fs group is used by the SecurityContext.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "hostIPC".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostIPC determines if the policy allows the use of HostIPC in the pod spec.".to_owned()),
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
                                description: Some("hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.".to_owned()),
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
                                description: Some("hostPID determines if the policy allows the use of HostPID in the pod spec.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostPorts".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostPorts determines which host port ranges are allowed to be exposed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::policy::v1beta1::HostPortRange>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "privileged".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("privileged determines if a pod can request to be run as privileged.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnlyRootFilesystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requiredDropCapabilities".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("requiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsGroup".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::RunAsGroupStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RunAsGroup is the strategy that will dictate the allowable RunAsGroup values that may be set. If this field is omitted, the pod's RunAsGroup can take any value. This field requires the RunAsGroup feature gate to be enabled.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "runAsUser".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::RunAsUserStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "runtimeClass".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::RuntimeClassStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("runtimeClass is the strategy that will dictate the allowable RuntimeClasses for a pod. If this field is omitted, the pod's runtimeClassName field is unrestricted. Enforcement of this field depends on the RuntimeClass feature gate being enabled.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "seLinux".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::SELinuxStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("seLinux is the strategy that will dictate the allowable labels that may be set.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "supplementalGroups".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::policy::v1beta1::SupplementalGroupsStrategyOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("supplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumes is a white list of allowed volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "fsGroup".to_owned(),
                    "runAsUser".to_owned(),
                    "seLinux".to_owned(),
                    "supplementalGroups".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
