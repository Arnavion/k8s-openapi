// Generated from definition io.k8s.api.core.v1.SecurityContext

/// SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
    pub allow_privilege_escalation: Option<bool>,

    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
    pub capabilities: Option<crate::api::core::v1::Capabilities>,

    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
    pub privileged: Option<bool>,

    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
    pub proc_mount: Option<String>,

    /// Whether this container has a read-only root filesystem. Default is false.
    pub read_only_root_filesystem: Option<bool>,

    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub se_linux_options: Option<crate::api::core::v1::SELinuxOptions>,

    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options.
    pub seccomp_profile: Option<crate::api::core::v1::SeccompProfile>,

    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,

}

#[cfg(feature = "dsl")]
impl SecurityContext  {
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


    /// Set [`Self::capabilities`]
    pub  fn capabilities_set(&mut self, capabilities: impl Into<Option<crate::api::core::v1::Capabilities>>) -> &mut Self {
        self.capabilities = capabilities.into(); self
    }

    pub  fn capabilities(&mut self) -> &mut crate::api::core::v1::Capabilities {
        if self.capabilities.is_none() { self.capabilities = Some(Default::default()) }
        self.capabilities.as_mut().unwrap()
    }

    /// Modify [`Self::capabilities`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn capabilities_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Capabilities)) -> &mut Self {
        if self.capabilities.is_none() { self.capabilities = Some(Default::default()) };
        func(self.capabilities.as_mut().unwrap()); self
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


    /// Set [`Self::proc_mount`]
    pub  fn proc_mount_set(&mut self, proc_mount: impl Into<Option<String>>) -> &mut Self {
        self.proc_mount = proc_mount.into(); self
    }

    pub  fn proc_mount(&mut self) -> &mut String {
        if self.proc_mount.is_none() { self.proc_mount = Some(Default::default()) }
        self.proc_mount.as_mut().unwrap()
    }

    /// Modify [`Self::proc_mount`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn proc_mount_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.proc_mount.is_none() { self.proc_mount = Some(Default::default()) };
        func(self.proc_mount.as_mut().unwrap()); self
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


    /// Set [`Self::run_as_group`]
    pub  fn run_as_group_set(&mut self, run_as_group: impl Into<Option<i64>>) -> &mut Self {
        self.run_as_group = run_as_group.into(); self
    }

    pub  fn run_as_group(&mut self) -> &mut i64 {
        if self.run_as_group.is_none() { self.run_as_group = Some(Default::default()) }
        self.run_as_group.as_mut().unwrap()
    }

    /// Modify [`Self::run_as_group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn run_as_group_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.run_as_group.is_none() { self.run_as_group = Some(Default::default()) };
        func(self.run_as_group.as_mut().unwrap()); self
    }


    /// Set [`Self::run_as_non_root`]
    pub  fn run_as_non_root_set(&mut self, run_as_non_root: impl Into<Option<bool>>) -> &mut Self {
        self.run_as_non_root = run_as_non_root.into(); self
    }

    pub  fn run_as_non_root(&mut self) -> &mut bool {
        if self.run_as_non_root.is_none() { self.run_as_non_root = Some(Default::default()) }
        self.run_as_non_root.as_mut().unwrap()
    }

    /// Modify [`Self::run_as_non_root`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn run_as_non_root_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.run_as_non_root.is_none() { self.run_as_non_root = Some(Default::default()) };
        func(self.run_as_non_root.as_mut().unwrap()); self
    }


    /// Set [`Self::run_as_user`]
    pub  fn run_as_user_set(&mut self, run_as_user: impl Into<Option<i64>>) -> &mut Self {
        self.run_as_user = run_as_user.into(); self
    }

    pub  fn run_as_user(&mut self) -> &mut i64 {
        if self.run_as_user.is_none() { self.run_as_user = Some(Default::default()) }
        self.run_as_user.as_mut().unwrap()
    }

    /// Modify [`Self::run_as_user`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn run_as_user_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.run_as_user.is_none() { self.run_as_user = Some(Default::default()) };
        func(self.run_as_user.as_mut().unwrap()); self
    }


    /// Set [`Self::se_linux_options`]
    pub  fn se_linux_options_set(&mut self, se_linux_options: impl Into<Option<crate::api::core::v1::SELinuxOptions>>) -> &mut Self {
        self.se_linux_options = se_linux_options.into(); self
    }

    pub  fn se_linux_options(&mut self) -> &mut crate::api::core::v1::SELinuxOptions {
        if self.se_linux_options.is_none() { self.se_linux_options = Some(Default::default()) }
        self.se_linux_options.as_mut().unwrap()
    }

    /// Modify [`Self::se_linux_options`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn se_linux_options_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::SELinuxOptions)) -> &mut Self {
        if self.se_linux_options.is_none() { self.se_linux_options = Some(Default::default()) };
        func(self.se_linux_options.as_mut().unwrap()); self
    }


    /// Set [`Self::seccomp_profile`]
    pub  fn seccomp_profile_set(&mut self, seccomp_profile: impl Into<Option<crate::api::core::v1::SeccompProfile>>) -> &mut Self {
        self.seccomp_profile = seccomp_profile.into(); self
    }

    pub  fn seccomp_profile(&mut self) -> &mut crate::api::core::v1::SeccompProfile {
        if self.seccomp_profile.is_none() { self.seccomp_profile = Some(Default::default()) }
        self.seccomp_profile.as_mut().unwrap()
    }

    /// Modify [`Self::seccomp_profile`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn seccomp_profile_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::SeccompProfile)) -> &mut Self {
        if self.seccomp_profile.is_none() { self.seccomp_profile = Some(Default::default()) };
        func(self.seccomp_profile.as_mut().unwrap()); self
    }


    /// Set [`Self::windows_options`]
    pub  fn windows_options_set(&mut self, windows_options: impl Into<Option<crate::api::core::v1::WindowsSecurityContextOptions>>) -> &mut Self {
        self.windows_options = windows_options.into(); self
    }

    pub  fn windows_options(&mut self) -> &mut crate::api::core::v1::WindowsSecurityContextOptions {
        if self.windows_options.is_none() { self.windows_options = Some(Default::default()) }
        self.windows_options.as_mut().unwrap()
    }

    /// Modify [`Self::windows_options`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn windows_options_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::WindowsSecurityContextOptions)) -> &mut Self {
        if self.windows_options.is_none() { self.windows_options = Some(Default::default()) };
        func(self.windows_options.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for SecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_privilege_escalation,
            Key_capabilities,
            Key_privileged,
            Key_proc_mount,
            Key_read_only_root_filesystem,
            Key_run_as_group,
            Key_run_as_non_root,
            Key_run_as_user,
            Key_se_linux_options,
            Key_seccomp_profile,
            Key_windows_options,
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
                            "capabilities" => Field::Key_capabilities,
                            "privileged" => Field::Key_privileged,
                            "procMount" => Field::Key_proc_mount,
                            "readOnlyRootFilesystem" => Field::Key_read_only_root_filesystem,
                            "runAsGroup" => Field::Key_run_as_group,
                            "runAsNonRoot" => Field::Key_run_as_non_root,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            "seccompProfile" => Field::Key_seccomp_profile,
                            "windowsOptions" => Field::Key_windows_options,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityContext;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecurityContext")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_capabilities: Option<crate::api::core::v1::Capabilities> = None;
                let mut value_privileged: Option<bool> = None;
                let mut value_proc_mount: Option<String> = None;
                let mut value_read_only_root_filesystem: Option<bool> = None;
                let mut value_run_as_group: Option<i64> = None;
                let mut value_run_as_non_root: Option<bool> = None;
                let mut value_run_as_user: Option<i64> = None;
                let mut value_se_linux_options: Option<crate::api::core::v1::SELinuxOptions> = None;
                let mut value_seccomp_profile: Option<crate::api::core::v1::SeccompProfile> = None;
                let mut value_windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_privilege_escalation => value_allow_privilege_escalation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capabilities => value_capabilities = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_privileged => value_privileged = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_proc_mount => value_proc_mount = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only_root_filesystem => value_read_only_root_filesystem = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_group => value_run_as_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_non_root => value_run_as_non_root = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux_options => value_se_linux_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_seccomp_profile => value_seccomp_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_windows_options => value_windows_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecurityContext {
                    allow_privilege_escalation: value_allow_privilege_escalation,
                    capabilities: value_capabilities,
                    privileged: value_privileged,
                    proc_mount: value_proc_mount,
                    read_only_root_filesystem: value_read_only_root_filesystem,
                    run_as_group: value_run_as_group,
                    run_as_non_root: value_run_as_non_root,
                    run_as_user: value_run_as_user,
                    se_linux_options: value_se_linux_options,
                    seccomp_profile: value_seccomp_profile,
                    windows_options: value_windows_options,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecurityContext",
            &[
                "allowPrivilegeEscalation",
                "capabilities",
                "privileged",
                "procMount",
                "readOnlyRootFilesystem",
                "runAsGroup",
                "runAsNonRoot",
                "runAsUser",
                "seLinuxOptions",
                "seccompProfile",
                "windowsOptions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SecurityContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecurityContext",
            self.allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.capabilities.as_ref().map_or(0, |_| 1) +
            self.privileged.as_ref().map_or(0, |_| 1) +
            self.proc_mount.as_ref().map_or(0, |_| 1) +
            self.read_only_root_filesystem.as_ref().map_or(0, |_| 1) +
            self.run_as_group.as_ref().map_or(0, |_| 1) +
            self.run_as_non_root.as_ref().map_or(0, |_| 1) +
            self.run_as_user.as_ref().map_or(0, |_| 1) +
            self.se_linux_options.as_ref().map_or(0, |_| 1) +
            self.seccomp_profile.as_ref().map_or(0, |_| 1) +
            self.windows_options.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_privilege_escalation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.capabilities {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capabilities", value)?;
        }
        if let Some(value) = &self.privileged {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "privileged", value)?;
        }
        if let Some(value) = &self.proc_mount {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "procMount", value)?;
        }
        if let Some(value) = &self.read_only_root_filesystem {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnlyRootFilesystem", value)?;
        }
        if let Some(value) = &self.run_as_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsGroup", value)?;
        }
        if let Some(value) = &self.run_as_non_root {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsNonRoot", value)?;
        }
        if let Some(value) = &self.run_as_user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUser", value)?;
        }
        if let Some(value) = &self.se_linux_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "seLinuxOptions", value)?;
        }
        if let Some(value) = &self.seccomp_profile {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "seccompProfile", value)?;
        }
        if let Some(value) = &self.windows_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "windowsOptions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SecurityContext {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.SecurityContext".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowPrivilegeEscalation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capabilities".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Capabilities>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "privileged".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "procMount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnlyRootFilesystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Whether this container has a read-only root filesystem. Default is false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsGroup".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsNonRoot".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsUser".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "seLinuxOptions".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SELinuxOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "seccompProfile".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SeccompProfile>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "windowsOptions".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::WindowsSecurityContextOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
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
