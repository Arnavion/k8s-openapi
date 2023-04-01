// Generated from definition io.k8s.api.policy.v1beta1.PodSecurityPolicySpec

/// PodSecurityPolicySpec defines the policy enforced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicySpec {
    /// allowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
    pub allow_privilege_escalation: Option<bool>,

    /// AllowedCSIDrivers is an allowlist of inline CSI drivers that must be explicitly set to be embedded within a pod spec. An empty value indicates that any CSI driver can be used for inline ephemeral volumes. This is a beta field, and is only honored if the API server enables the CSIInlineVolume feature gate.
    pub allowed_csi_drivers: Option<Vec<crate::api::policy::v1beta1::AllowedCSIDriver>>,

    /// allowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both allowedCapabilities and requiredDropCapabilities.
    pub allowed_capabilities: Option<Vec<String>>,

    /// allowedFlexVolumes is an allowlist of Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the "volumes" field.
    pub allowed_flex_volumes: Option<Vec<crate::api::policy::v1beta1::AllowedFlexVolume>>,

    /// allowedHostPaths is an allowlist of host paths. Empty indicates that all host paths may be used.
    pub allowed_host_paths: Option<Vec<crate::api::policy::v1beta1::AllowedHostPath>>,

    /// AllowedProcMountTypes is an allowlist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled.
    pub allowed_proc_mount_types: Option<Vec<String>>,

    /// allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in "*" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to allowlist all allowed unsafe sysctls explicitly to avoid rejection.
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

    /// volumes is an allowlist of volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'.
    pub volumes: Option<Vec<String>>,
}

impl crate::DeepMerge for PodSecurityPolicySpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allow_privilege_escalation, other.allow_privilege_escalation);
        crate::merge_strategies::list::atomic(&mut self.allowed_csi_drivers, other.allowed_csi_drivers);
        crate::merge_strategies::list::atomic(&mut self.allowed_capabilities, other.allowed_capabilities);
        crate::merge_strategies::list::atomic(&mut self.allowed_flex_volumes, other.allowed_flex_volumes);
        crate::merge_strategies::list::atomic(&mut self.allowed_host_paths, other.allowed_host_paths);
        crate::merge_strategies::list::atomic(&mut self.allowed_proc_mount_types, other.allowed_proc_mount_types);
        crate::merge_strategies::list::atomic(&mut self.allowed_unsafe_sysctls, other.allowed_unsafe_sysctls);
        crate::merge_strategies::list::atomic(&mut self.default_add_capabilities, other.default_add_capabilities);
        crate::DeepMerge::merge_from(&mut self.default_allow_privilege_escalation, other.default_allow_privilege_escalation);
        crate::merge_strategies::list::atomic(&mut self.forbidden_sysctls, other.forbidden_sysctls);
        crate::DeepMerge::merge_from(&mut self.fs_group, other.fs_group);
        crate::DeepMerge::merge_from(&mut self.host_ipc, other.host_ipc);
        crate::DeepMerge::merge_from(&mut self.host_network, other.host_network);
        crate::DeepMerge::merge_from(&mut self.host_pid, other.host_pid);
        crate::merge_strategies::list::atomic(&mut self.host_ports, other.host_ports);
        crate::DeepMerge::merge_from(&mut self.privileged, other.privileged);
        crate::DeepMerge::merge_from(&mut self.read_only_root_filesystem, other.read_only_root_filesystem);
        crate::merge_strategies::list::atomic(&mut self.required_drop_capabilities, other.required_drop_capabilities);
        crate::DeepMerge::merge_from(&mut self.run_as_group, other.run_as_group);
        crate::DeepMerge::merge_from(&mut self.run_as_user, other.run_as_user);
        crate::DeepMerge::merge_from(&mut self.runtime_class, other.runtime_class);
        crate::DeepMerge::merge_from(&mut self.se_linux, other.se_linux);
        crate::DeepMerge::merge_from(&mut self.supplemental_groups, other.supplemental_groups);
        crate::merge_strategies::list::atomic(&mut self.volumes, other.volumes);
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
                                description: Some("AllowedCSIDrivers is an allowlist of inline CSI drivers that must be explicitly set to be embedded within a pod spec. An empty value indicates that any CSI driver can be used for inline ephemeral volumes. This is a beta field, and is only honored if the API server enables the CSIInlineVolume feature gate.".to_owned()),
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
                                description: Some("allowedFlexVolumes is an allowlist of Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the \"volumes\" field.".to_owned()),
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
                                description: Some("allowedHostPaths is an allowlist of host paths. Empty indicates that all host paths may be used.".to_owned()),
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
                                description: Some("AllowedProcMountTypes is an allowlist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled.".to_owned()),
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
                                description: Some("allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to allowlist all allowed unsafe sysctls explicitly to avoid rejection.\n\nExamples: e.g. \"foo/*\" allows \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" allows \"foo.bar\", \"foo.baz\", etc.".to_owned()),
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
                                description: Some("volumes is an allowlist of volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'.".to_owned()),
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
