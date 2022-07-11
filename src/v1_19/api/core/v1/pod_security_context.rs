// Generated from definition io.k8s.api.core.v1.PodSecurityContext

/// PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityContext {
    /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
    ///
    /// 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
    ///
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume.
    pub fs_group: Option<i64>,

    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified defaults to "Always".
    pub fs_group_change_policy: Option<String>,

    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
    pub se_linux_options: Option<crate::api::core::v1::SELinuxOptions>,

    /// The seccomp options to use by the containers in this pod.
    pub seccomp_profile: Option<crate::api::core::v1::SeccompProfile>,

    /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
    pub supplemental_groups: Option<Vec<i64>>,

    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
    pub sysctls: Option<Vec<crate::api::core::v1::Sysctl>>,

    /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,

}

#[cfg(feature = "dsl")]
impl PodSecurityContext  {
    /// Set [`Self::fs_group`]
    pub  fn fs_group_set(&mut self, fs_group: impl Into<Option<i64>>) -> &mut Self {
        self.fs_group = fs_group.into(); self
    }

    pub  fn fs_group(&mut self) -> &mut i64 {
        if self.fs_group.is_none() { self.fs_group = Some(Default::default()) }
        self.fs_group.as_mut().unwrap()
    }

    /// Modify [`Self::fs_group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fs_group_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.fs_group.is_none() { self.fs_group = Some(Default::default()) };
        func(self.fs_group.as_mut().unwrap()); self
    }


    /// Set [`Self::fs_group_change_policy`]
    pub  fn fs_group_change_policy_set(&mut self, fs_group_change_policy: impl Into<Option<String>>) -> &mut Self {
        self.fs_group_change_policy = fs_group_change_policy.into(); self
    }

    pub  fn fs_group_change_policy(&mut self) -> &mut String {
        if self.fs_group_change_policy.is_none() { self.fs_group_change_policy = Some(Default::default()) }
        self.fs_group_change_policy.as_mut().unwrap()
    }

    /// Modify [`Self::fs_group_change_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fs_group_change_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.fs_group_change_policy.is_none() { self.fs_group_change_policy = Some(Default::default()) };
        func(self.fs_group_change_policy.as_mut().unwrap()); self
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


    /// Set [`Self::supplemental_groups`]
    pub  fn supplemental_groups_set(&mut self, supplemental_groups: impl Into<Option<Vec<i64>>>) -> &mut Self {
        self.supplemental_groups = supplemental_groups.into(); self
    }

    pub  fn supplemental_groups(&mut self) -> &mut Vec<i64> {
        if self.supplemental_groups.is_none() { self.supplemental_groups = Some(Default::default()) }
        self.supplemental_groups.as_mut().unwrap()
    }

    /// Modify [`Self::supplemental_groups`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn supplemental_groups_with(&mut self, func: impl FnOnce(&mut Vec<i64>)) -> &mut Self {
        if self.supplemental_groups.is_none() { self.supplemental_groups = Some(Default::default()) };
        func(self.supplemental_groups.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::supplemental_groups`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn supplemental_groups_push_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.supplemental_groups.is_none() {
            self.supplemental_groups = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.supplemental_groups.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::supplemental_groups`]
    pub  fn supplemental_groups_append_from(&mut self, other: impl std::borrow::Borrow<[i64]>) -> &mut Self {
         if self.supplemental_groups.is_none() { self.supplemental_groups = Some(Vec::new()); }
         let supplemental_groups = &mut self.supplemental_groups.as_mut().unwrap();
         for item in other.borrow() {
             supplemental_groups.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::sysctls`]
    pub  fn sysctls_set(&mut self, sysctls: impl Into<Option<Vec<crate::api::core::v1::Sysctl>>>) -> &mut Self {
        self.sysctls = sysctls.into(); self
    }

    pub  fn sysctls(&mut self) -> &mut Vec<crate::api::core::v1::Sysctl> {
        if self.sysctls.is_none() { self.sysctls = Some(Default::default()) }
        self.sysctls.as_mut().unwrap()
    }

    /// Modify [`Self::sysctls`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn sysctls_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Sysctl>)) -> &mut Self {
        if self.sysctls.is_none() { self.sysctls = Some(Default::default()) };
        func(self.sysctls.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::sysctls`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn sysctls_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Sysctl)) -> &mut Self {
        if self.sysctls.is_none() {
            self.sysctls = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.sysctls.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::sysctls`]
    pub  fn sysctls_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Sysctl]>) -> &mut Self {
         if self.sysctls.is_none() { self.sysctls = Some(Vec::new()); }
         let sysctls = &mut self.sysctls.as_mut().unwrap();
         for item in other.borrow() {
             sysctls.push(item.to_owned());
         }
         self
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


impl<'de> crate::serde::Deserialize<'de> for PodSecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_group,
            Key_fs_group_change_policy,
            Key_run_as_group,
            Key_run_as_non_root,
            Key_run_as_user,
            Key_se_linux_options,
            Key_seccomp_profile,
            Key_supplemental_groups,
            Key_sysctls,
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
                            "fsGroup" => Field::Key_fs_group,
                            "fsGroupChangePolicy" => Field::Key_fs_group_change_policy,
                            "runAsGroup" => Field::Key_run_as_group,
                            "runAsNonRoot" => Field::Key_run_as_non_root,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            "seccompProfile" => Field::Key_seccomp_profile,
                            "supplementalGroups" => Field::Key_supplemental_groups,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityContext")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_group: Option<i64> = None;
                let mut value_fs_group_change_policy: Option<String> = None;
                let mut value_run_as_group: Option<i64> = None;
                let mut value_run_as_non_root: Option<bool> = None;
                let mut value_run_as_user: Option<i64> = None;
                let mut value_se_linux_options: Option<crate::api::core::v1::SELinuxOptions> = None;
                let mut value_seccomp_profile: Option<crate::api::core::v1::SeccompProfile> = None;
                let mut value_supplemental_groups: Option<Vec<i64>> = None;
                let mut value_sysctls: Option<Vec<crate::api::core::v1::Sysctl>> = None;
                let mut value_windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_group => value_fs_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group_change_policy => value_fs_group_change_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_group => value_run_as_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_non_root => value_run_as_non_root = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux_options => value_se_linux_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_seccomp_profile => value_seccomp_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups => value_supplemental_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sysctls => value_sysctls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_windows_options => value_windows_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityContext {
                    fs_group: value_fs_group,
                    fs_group_change_policy: value_fs_group_change_policy,
                    run_as_group: value_run_as_group,
                    run_as_non_root: value_run_as_non_root,
                    run_as_user: value_run_as_user,
                    se_linux_options: value_se_linux_options,
                    seccomp_profile: value_seccomp_profile,
                    supplemental_groups: value_supplemental_groups,
                    sysctls: value_sysctls,
                    windows_options: value_windows_options,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityContext",
            &[
                "fsGroup",
                "fsGroupChangePolicy",
                "runAsGroup",
                "runAsNonRoot",
                "runAsUser",
                "seLinuxOptions",
                "seccompProfile",
                "supplementalGroups",
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
            self.fs_group.as_ref().map_or(0, |_| 1) +
            self.fs_group_change_policy.as_ref().map_or(0, |_| 1) +
            self.run_as_group.as_ref().map_or(0, |_| 1) +
            self.run_as_non_root.as_ref().map_or(0, |_| 1) +
            self.run_as_user.as_ref().map_or(0, |_| 1) +
            self.se_linux_options.as_ref().map_or(0, |_| 1) +
            self.seccomp_profile.as_ref().map_or(0, |_| 1) +
            self.supplemental_groups.as_ref().map_or(0, |_| 1) +
            self.sysctls.as_ref().map_or(0, |_| 1) +
            self.windows_options.as_ref().map_or(0, |_| 1),
        )?;
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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PodSecurityContext".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fsGroup".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:\n\n1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----\n\nIf unset, the Kubelet will not modify the ownership and permissions of any volume.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsGroupChangePolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are \"OnRootMismatch\" and \"Always\". If not specified defaults to \"Always\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsGroup".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.".to_owned()),
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
                                description: Some("Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
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
                                description: Some("The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.".to_owned()),
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
                                description: Some("The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.".to_owned()),
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
                                description: Some("The seccomp options to use by the containers in this pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "supplementalGroups".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".to_owned()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sysctls".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Sysctl>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "windowsOptions".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::WindowsSecurityContextOptions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
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
