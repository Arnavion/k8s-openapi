// Generated from definition io.k8s.api.extensions.v1beta1.PodSecurityPolicySpec

/// Pod Security Policy Spec defines the policy enforced.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodSecurityPolicySpec {
    /// AllowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
    #[serde(rename = "allowPrivilegeEscalation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,

    /// AllowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both AllowedCapabilities and RequiredDropCapabilities.
    #[serde(rename = "allowedCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_capabilities: Option<Vec<String>>,

    /// AllowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the "Volumes" field.
    #[serde(rename = "allowedFlexVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_flex_volumes: Option<Vec<::api::extensions::v1beta1::AllowedFlexVolume>>,

    /// is a white list of allowed host paths. Empty indicates that all host paths may be used.
    #[serde(rename = "allowedHostPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_host_paths: Option<Vec<::api::extensions::v1beta1::AllowedHostPath>>,

    /// DefaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both DefaultAddCapabilities and RequiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the AllowedCapabilities list.
    #[serde(rename = "defaultAddCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_add_capabilities: Option<Vec<String>>,

    /// DefaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.
    #[serde(rename = "defaultAllowPrivilegeEscalation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allow_privilege_escalation: Option<bool>,

    /// FSGroup is the strategy that will dictate what fs group is used by the SecurityContext.
    #[serde(rename = "fsGroup")]
    pub fs_group: ::api::extensions::v1beta1::FSGroupStrategyOptions,

    /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
    #[serde(rename = "hostIPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,

    /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
    #[serde(rename = "hostNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,

    /// hostPID determines if the policy allows the use of HostPID in the pod spec.
    #[serde(rename = "hostPID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,

    /// hostPorts determines which host port ranges are allowed to be exposed.
    #[serde(rename = "hostPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ports: Option<Vec<::api::extensions::v1beta1::HostPortRange>>,

    /// privileged determines if a pod can request to be run as privileged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// ReadOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
    #[serde(rename = "readOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,

    /// RequiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
    #[serde(rename = "requiredDropCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_drop_capabilities: Option<Vec<String>>,

    /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
    #[serde(rename = "runAsUser")]
    pub run_as_user: ::api::extensions::v1beta1::RunAsUserStrategyOptions,

    /// seLinux is the strategy that will dictate the allowable labels that may be set.
    #[serde(rename = "seLinux")]
    pub se_linux: ::api::extensions::v1beta1::SELinuxStrategyOptions,

    /// SupplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
    #[serde(rename = "supplementalGroups")]
    pub supplemental_groups: ::api::extensions::v1beta1::SupplementalGroupsStrategyOptions,

    /// volumes is a white list of allowed volume plugins.  Empty indicates that all plugins may be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}
