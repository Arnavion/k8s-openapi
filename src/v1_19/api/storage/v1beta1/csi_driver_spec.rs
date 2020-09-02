// Generated from definition io.k8s.api.storage.v1beta1.CSIDriverSpec

/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach operation (because it implements the CSI ControllerPublishVolume() method), and that the Kubernetes attach detach controller should call the attach volume interface which checks the volumeattachment status and waits until the volume is attached before proceeding to mounting. The CSI external-attacher coordinates with CSI volume driver and updates the volumeattachment status when the attach operation is complete. If the CSIDriverRegistry feature gate is enabled and the value is specified to false, the attach operation will be skipped. Otherwise the attach operation will be called.
    pub attach_required: Option<bool>,

    /// Defines if the underlying volume supports changing ownership and permission of the volume before being mounted. Refer to the specific FSGroupPolicy values for additional details. This field is alpha-level, and is only honored by servers that enable the CSIVolumeFSGroupPolicy feature gate.
    pub fs_group_policy: Option<String>,

    /// If set to true, podInfoOnMount indicates this CSI volume driver requires additional pod information (like podName, podUID, etc.) during mount operations. If set to false, pod information will not be passed on mount. Default is false. The CSI driver specifies podInfoOnMount as part of driver deployment. If true, Kubelet will pass pod information as VolumeContext in the CSI NodePublishVolume() calls. The CSI driver is responsible for parsing and validating the information passed in as VolumeContext. The following VolumeConext will be passed if podInfoOnMount is set to true. This list might grow, but the prefix will be used. "csi.storage.k8s.io/pod.name": pod.Name "csi.storage.k8s.io/pod.namespace": pod.Namespace "csi.storage.k8s.io/pod.uid": string(pod.UID) "csi.storage.k8s.io/ephemeral": "true" iff the volume is an ephemeral inline volume
    ///                                 defined by a CSIVolumeSource, otherwise "false"
    ///
    /// "csi.storage.k8s.io/ephemeral" is a new feature in Kubernetes 1.16. It is only required for drivers which support both the "Persistent" and "Ephemeral" VolumeLifecycleMode. Other drivers can leave pod info disabled and/or ignore this field. As Kubernetes 1.15 doesn't support this field, drivers can only support one mode when deployed on such a cluster and the deployment determines which mode that is, for example via a command line parameter of the driver.
    pub pod_info_on_mount: Option<bool>,

    /// If set to true, storageCapacity indicates that the CSI volume driver wants pod scheduling to consider the storage capacity that the driver deployment will report by creating CSIStorageCapacity objects with capacity information.
    ///
    /// The check can be enabled immediately when deploying a driver. In that case, provisioning new volumes with late binding will pause until the driver deployment has published some suitable CSIStorageCapacity object.
    ///
    /// Alternatively, the driver can be deployed with the field unset or false and it can be flipped later when storage capacity information has been published.
    ///
    /// This is an alpha field and only available when the CSIStorageCapacity feature is enabled. The default is false.
    pub storage_capacity: Option<bool>,

    /// VolumeLifecycleModes defines what kind of volumes this CSI volume driver supports. The default if the list is empty is "Persistent", which is the usage defined by the CSI specification and implemented in Kubernetes via the usual PV/PVC mechanism. The other mode is "Ephemeral". In this mode, volumes are defined inline inside the pod spec with CSIVolumeSource and their lifecycle is tied to the lifecycle of that pod. A driver has to be aware of this because it is only going to get a NodePublishVolume call for such a volume. For more information about implementing this mode, see https://kubernetes-csi.github.io/docs/ephemeral-local-volumes.html A driver can support one or more of these modes and more modes may be added in the future.
    pub volume_lifecycle_modes: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for CSIDriverSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attach_required,
            Key_fs_group_policy,
            Key_pod_info_on_mount,
            Key_storage_capacity,
            Key_volume_lifecycle_modes,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "attachRequired" => Field::Key_attach_required,
                            "fsGroupPolicy" => Field::Key_fs_group_policy,
                            "podInfoOnMount" => Field::Key_pod_info_on_mount,
                            "storageCapacity" => Field::Key_storage_capacity,
                            "volumeLifecycleModes" => Field::Key_volume_lifecycle_modes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CSIDriverSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSIDriverSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_attach_required: Option<bool> = None;
                let mut value_fs_group_policy: Option<String> = None;
                let mut value_pod_info_on_mount: Option<bool> = None;
                let mut value_storage_capacity: Option<bool> = None;
                let mut value_volume_lifecycle_modes: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attach_required => value_attach_required = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group_policy => value_fs_group_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_info_on_mount => value_pod_info_on_mount = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_capacity => value_storage_capacity = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_lifecycle_modes => value_volume_lifecycle_modes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIDriverSpec {
                    attach_required: value_attach_required,
                    fs_group_policy: value_fs_group_policy,
                    pod_info_on_mount: value_pod_info_on_mount,
                    storage_capacity: value_storage_capacity,
                    volume_lifecycle_modes: value_volume_lifecycle_modes,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIDriverSpec",
            &[
                "attachRequired",
                "fsGroupPolicy",
                "podInfoOnMount",
                "storageCapacity",
                "volumeLifecycleModes",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CSIDriverSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIDriverSpec",
            self.attach_required.as_ref().map_or(0, |_| 1) +
            self.fs_group_policy.as_ref().map_or(0, |_| 1) +
            self.pod_info_on_mount.as_ref().map_or(0, |_| 1) +
            self.storage_capacity.as_ref().map_or(0, |_| 1) +
            self.volume_lifecycle_modes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.attach_required {
            serde::ser::SerializeStruct::serialize_field(&mut state, "attachRequired", value)?;
        }
        if let Some(value) = &self.fs_group_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroupPolicy", value)?;
        }
        if let Some(value) = &self.pod_info_on_mount {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podInfoOnMount", value)?;
        }
        if let Some(value) = &self.storage_capacity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "storageCapacity", value)?;
        }
        if let Some(value) = &self.volume_lifecycle_modes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeLifecycleModes", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
