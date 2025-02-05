// Generated from definition io.k8s.api.core.v1.PodSecurityContext

/// PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityContext {
    /// appArmorProfile is the AppArmor options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
    pub app_armor_profile: Option<crate::api::core::v1::AppArmorProfile>,

    /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
    ///
    /// 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
    ///
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
    pub fs_group: Option<i64>,

    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
    pub fs_group_change_policy: Option<std::string::String>,

    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    pub se_linux_options: Option<crate::api::core::v1::SELinuxOptions>,

    /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
    pub seccomp_profile: Option<crate::api::core::v1::SeccompProfile>,

    /// A list of groups applied to the first process run in each container, in addition to the container's primary GID and fsGroup (if specified).  If the SupplementalGroupsPolicy feature is enabled, the supplementalGroupsPolicy field determines whether these are in addition to or instead of any group memberships defined in the container image. If unspecified, no additional groups are added, though group memberships defined in the container image may still be used, depending on the supplementalGroupsPolicy field. Note that this field cannot be set when spec.os.name is windows.
    pub supplemental_groups: Option<std::vec::Vec<i64>>,

    /// Defines how supplemental groups of the first container processes are calculated. Valid values are "Merge" and "Strict". If not specified, "Merge" is used. (Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled and the container runtime must implement support for this feature. Note that this field cannot be set when spec.os.name is windows.
    pub supplemental_groups_policy: Option<std::string::String>,

    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
    pub sysctls: Option<std::vec::Vec<crate::api::core::v1::Sysctl>>,

    /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,
}

impl crate::DeepMerge for PodSecurityContext {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.app_armor_profile, other.app_armor_profile);
        crate::DeepMerge::merge_from(&mut self.fs_group, other.fs_group);
        crate::DeepMerge::merge_from(&mut self.fs_group_change_policy, other.fs_group_change_policy);
        crate::DeepMerge::merge_from(&mut self.run_as_group, other.run_as_group);
        crate::DeepMerge::merge_from(&mut self.run_as_non_root, other.run_as_non_root);
        crate::DeepMerge::merge_from(&mut self.run_as_user, other.run_as_user);
        crate::DeepMerge::merge_from(&mut self.se_linux_options, other.se_linux_options);
        crate::DeepMerge::merge_from(&mut self.seccomp_profile, other.seccomp_profile);
        crate::merge_strategies::list::atomic(&mut self.supplemental_groups, other.supplemental_groups);
        crate::DeepMerge::merge_from(&mut self.supplemental_groups_policy, other.supplemental_groups_policy);
        crate::merge_strategies::list::atomic(&mut self.sysctls, other.sysctls);
        crate::DeepMerge::merge_from(&mut self.windows_options, other.windows_options);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodSecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_app_armor_profile,
            Key_fs_group,
            Key_fs_group_change_policy,
            Key_run_as_group,
            Key_run_as_non_root,
            Key_run_as_user,
            Key_se_linux_options,
            Key_seccomp_profile,
            Key_supplemental_groups,
            Key_supplemental_groups_policy,
            Key_sysctls,
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
                            "appArmorProfile" => Field::Key_app_armor_profile,
                            "fsGroup" => Field::Key_fs_group,
                            "fsGroupChangePolicy" => Field::Key_fs_group_change_policy,
                            "runAsGroup" => Field::Key_run_as_group,
                            "runAsNonRoot" => Field::Key_run_as_non_root,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            "seccompProfile" => Field::Key_seccomp_profile,
                            "supplementalGroups" => Field::Key_supplemental_groups,
                            "supplementalGroupsPolicy" => Field::Key_supplemental_groups_policy,
                            "sysctls" => Field::Key_sysctls,
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
            type Value = PodSecurityContext;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodSecurityContext")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_app_armor_profile: Option<crate::api::core::v1::AppArmorProfile> = None;
                let mut value_fs_group: Option<i64> = None;
                let mut value_fs_group_change_policy: Option<std::string::String> = None;
                let mut value_run_as_group: Option<i64> = None;
                let mut value_run_as_non_root: Option<bool> = None;
                let mut value_run_as_user: Option<i64> = None;
                let mut value_se_linux_options: Option<crate::api::core::v1::SELinuxOptions> = None;
                let mut value_seccomp_profile: Option<crate::api::core::v1::SeccompProfile> = None;
                let mut value_supplemental_groups: Option<std::vec::Vec<i64>> = None;
                let mut value_supplemental_groups_policy: Option<std::string::String> = None;
                let mut value_sysctls: Option<std::vec::Vec<crate::api::core::v1::Sysctl>> = None;
                let mut value_windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_app_armor_profile => value_app_armor_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group => value_fs_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group_change_policy => value_fs_group_change_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_group => value_run_as_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_non_root => value_run_as_non_root = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux_options => value_se_linux_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_seccomp_profile => value_seccomp_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups => value_supplemental_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups_policy => value_supplemental_groups_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sysctls => value_sysctls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_windows_options => value_windows_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityContext {
                    app_armor_profile: value_app_armor_profile,
                    fs_group: value_fs_group,
                    fs_group_change_policy: value_fs_group_change_policy,
                    run_as_group: value_run_as_group,
                    run_as_non_root: value_run_as_non_root,
                    run_as_user: value_run_as_user,
                    se_linux_options: value_se_linux_options,
                    seccomp_profile: value_seccomp_profile,
                    supplemental_groups: value_supplemental_groups,
                    supplemental_groups_policy: value_supplemental_groups_policy,
                    sysctls: value_sysctls,
                    windows_options: value_windows_options,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityContext",
            &[
                "appArmorProfile",
                "fsGroup",
                "fsGroupChangePolicy",
                "runAsGroup",
                "runAsNonRoot",
                "runAsUser",
                "seLinuxOptions",
                "seccompProfile",
                "supplementalGroups",
                "supplementalGroupsPolicy",
                "sysctls",
                "windowsOptions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodSecurityContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityContext",
            self.app_armor_profile.as_ref().map_or(0, |_| 1) +
            self.fs_group.as_ref().map_or(0, |_| 1) +
            self.fs_group_change_policy.as_ref().map_or(0, |_| 1) +
            self.run_as_group.as_ref().map_or(0, |_| 1) +
            self.run_as_non_root.as_ref().map_or(0, |_| 1) +
            self.run_as_user.as_ref().map_or(0, |_| 1) +
            self.se_linux_options.as_ref().map_or(0, |_| 1) +
            self.seccomp_profile.as_ref().map_or(0, |_| 1) +
            self.supplemental_groups.as_ref().map_or(0, |_| 1) +
            self.supplemental_groups_policy.as_ref().map_or(0, |_| 1) +
            self.sysctls.as_ref().map_or(0, |_| 1) +
            self.windows_options.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.app_armor_profile {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "appArmorProfile", value)?;
        }
        if let Some(value) = &self.fs_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroup", value)?;
        }
        if let Some(value) = &self.fs_group_change_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroupChangePolicy", value)?;
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
        if let Some(value) = &self.supplemental_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", value)?;
        }
        if let Some(value) = &self.supplemental_groups_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroupsPolicy", value)?;
        }
        if let Some(value) = &self.sysctls {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sysctls", value)?;
        }
        if let Some(value) = &self.windows_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "windowsOptions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodSecurityContext {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PodSecurityContext".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "appArmorProfile".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AppArmorProfile>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("appArmorProfile is the AppArmor options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "fsGroup".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:\n\n1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----\n\nIf unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsGroupChangePolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are \"OnRootMismatch\" and \"Always\". If not specified, \"Always\" is used. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsGroup".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.".into()),
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
                                description: Some("Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".into()),
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
                                description: Some("The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.".into()),
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
                                description: Some("The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.".into()),
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
                                description: Some("The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "supplementalGroups".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of groups applied to the first process run in each container, in addition to the container's primary GID and fsGroup (if specified).  If the SupplementalGroupsPolicy feature is enabled, the supplementalGroupsPolicy field determines whether these are in addition to or instead of any group memberships defined in the container image. If unspecified, no additional groups are added, though group memberships defined in the container image may still be used, depending on the supplementalGroupsPolicy field. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".into()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "supplementalGroupsPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Defines how supplemental groups of the first container processes are calculated. Valid values are \"Merge\" and \"Strict\". If not specified, \"Merge\" is used. (Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled and the container runtime must implement support for this feature. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sysctls".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::Sysctl>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "windowsOptions".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::WindowsSecurityContextOptions>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.".into()),
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
