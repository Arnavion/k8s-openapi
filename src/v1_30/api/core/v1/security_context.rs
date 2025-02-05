// Generated from definition io.k8s.api.core.v1.SecurityContext

/// SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
    pub allow_privilege_escalation: Option<bool>,

    /// appArmorProfile is the AppArmor options to use by this container. If set, this profile overrides the pod's appArmorProfile. Note that this field cannot be set when spec.os.name is windows.
    pub app_armor_profile: Option<crate::api::core::v1::AppArmorProfile>,

    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
    pub capabilities: Option<crate::api::core::v1::Capabilities>,

    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
    pub privileged: Option<bool>,

    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
    pub proc_mount: Option<std::string::String>,

    /// Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
    pub read_only_root_filesystem: Option<bool>,

    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    pub se_linux_options: Option<crate::api::core::v1::SELinuxOptions>,

    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
    pub seccomp_profile: Option<crate::api::core::v1::SeccompProfile>,

    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,
}

impl crate::DeepMerge for SecurityContext {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allow_privilege_escalation, other.allow_privilege_escalation);
        crate::DeepMerge::merge_from(&mut self.app_armor_profile, other.app_armor_profile);
        crate::DeepMerge::merge_from(&mut self.capabilities, other.capabilities);
        crate::DeepMerge::merge_from(&mut self.privileged, other.privileged);
        crate::DeepMerge::merge_from(&mut self.proc_mount, other.proc_mount);
        crate::DeepMerge::merge_from(&mut self.read_only_root_filesystem, other.read_only_root_filesystem);
        crate::DeepMerge::merge_from(&mut self.run_as_group, other.run_as_group);
        crate::DeepMerge::merge_from(&mut self.run_as_non_root, other.run_as_non_root);
        crate::DeepMerge::merge_from(&mut self.run_as_user, other.run_as_user);
        crate::DeepMerge::merge_from(&mut self.se_linux_options, other.se_linux_options);
        crate::DeepMerge::merge_from(&mut self.seccomp_profile, other.seccomp_profile);
        crate::DeepMerge::merge_from(&mut self.windows_options, other.windows_options);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_privilege_escalation,
            Key_app_armor_profile,
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "allowPrivilegeEscalation" => Field::Key_allow_privilege_escalation,
                            "appArmorProfile" => Field::Key_app_armor_profile,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SecurityContext")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_app_armor_profile: Option<crate::api::core::v1::AppArmorProfile> = None;
                let mut value_capabilities: Option<crate::api::core::v1::Capabilities> = None;
                let mut value_privileged: Option<bool> = None;
                let mut value_proc_mount: Option<std::string::String> = None;
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
                        Field::Key_app_armor_profile => value_app_armor_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    app_armor_profile: value_app_armor_profile,
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
                "appArmorProfile",
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
            self.app_armor_profile.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.app_armor_profile {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "appArmorProfile", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.SecurityContext".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowPrivilegeEscalation".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "appArmorProfile".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AppArmorProfile>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("appArmorProfile is the AppArmor options to use by this container. If set, this profile overrides the pod's appArmorProfile. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "capabilities".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Capabilities>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "privileged".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "procMount".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnlyRootFilesystem".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsGroup".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsNonRoot".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsUser".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "seLinuxOptions".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SELinuxOptions>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "seccompProfile".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SeccompProfile>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "windowsOptions".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::WindowsSecurityContextOptions>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.".into()),
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
