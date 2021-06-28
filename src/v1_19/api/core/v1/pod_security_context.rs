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
    pub supplemental_groups: Vec<i64>,

    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
    pub sysctls: Vec<crate::api::core::v1::Sysctl>,

    /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,
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
                    supplemental_groups: value_supplemental_groups.unwrap_or_default(),
                    sysctls: value_sysctls.unwrap_or_default(),
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
            usize::from(!self.supplemental_groups.is_empty()) +
            usize::from(!self.sysctls.is_empty()) +
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
        if !self.supplemental_groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", &self.supplemental_groups)?;
        }
        if !self.sysctls.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sysctls", &self.sysctls)?;
        }
        if let Some(value) = &self.windows_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "windowsOptions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PodSecurityContext {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.",
          "properties": {
            "fsGroup": {
              "description": "A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:\n\n1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----\n\nIf unset, the Kubelet will not modify the ownership and permissions of any volume.",
              "format": "int64",
              "type": "integer"
            },
            "fsGroupChangePolicy": {
              "description": "fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are \"OnRootMismatch\" and \"Always\". If not specified defaults to \"Always\".",
              "type": "string"
            },
            "runAsGroup": {
              "description": "The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.",
              "format": "int64",
              "type": "integer"
            },
            "runAsNonRoot": {
              "description": "Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.",
              "type": "boolean"
            },
            "runAsUser": {
              "description": "The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.",
              "format": "int64",
              "type": "integer"
            },
            "seLinuxOptions": crate::schema_ref_with_description(crate::api::core::v1::SELinuxOptions::schema(), "The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container."),
            "seccompProfile": crate::schema_ref_with_description(crate::api::core::v1::SeccompProfile::schema(), "The seccomp options to use by the containers in this pod."),
            "supplementalGroups": {
              "description": "A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.",
              "items": {
                "format": "int64",
                "type": "integer"
              },
              "type": "array"
            },
            "sysctls": {
              "description": "Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.",
              "items": crate::api::core::v1::Sysctl::schema(),
              "type": "array"
            },
            "windowsOptions": crate::schema_ref_with_description(crate::api::core::v1::WindowsSecurityContextOptions::schema(), "The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.")
          },
          "type": "object"
        })
    }
}
