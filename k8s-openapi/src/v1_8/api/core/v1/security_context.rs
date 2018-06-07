// Generated from definition io.k8s.api.core.v1.SecurityContext

/// SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
    #[serde(rename = "allowPrivilegeEscalation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,

    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<::v1_8::api::core::v1::Capabilities>,

    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// Whether this container has a read-only root filesystem. Default is false.
    #[serde(rename = "readOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsNonRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "seLinuxOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<::v1_8::api::core::v1::SELinuxOptions>,
}
