// Generated from definition io.k8s.api.storage.v1beta1.CSIDriverSpec

/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach operation (because it implements the CSI ControllerPublishVolume() method), and that the Kubernetes attach detach controller should call the attach volume interface which checks the volumeattachment status and waits until the volume is attached before proceeding to mounting. The CSI external-attacher coordinates with CSI volume driver and updates the volumeattachment status when the attach operation is complete. If the CSIDriverRegistry feature gate is enabled and the value is specified to false, the attach operation will be skipped. Otherwise the attach operation will be called.
    pub attach_required: Option<bool>,

    /// If set to true, podInfoOnMount indicates this CSI volume driver requires additional pod information (like podName, podUID, etc.) during mount operations. If set to false, pod information will not be passed on mount. Default is false. The CSI driver specifies podInfoOnMount as part of driver deployment. If true, Kubelet will pass pod information as VolumeContext in the CSI NodePublishVolume() calls. The CSI driver is responsible for parsing and validating the information passed in as VolumeContext. The following VolumeConext will be passed if podInfoOnMount is set to true. This list might grow, but the prefix will be used. "csi.storage.k8s.io/pod.name": pod.Name "csi.storage.k8s.io/pod.namespace": pod.Namespace "csi.storage.k8s.io/pod.uid": string(pod.UID)
    pub pod_info_on_mount: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for CSIDriverSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attach_required,
            Key_pod_info_on_mount,
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
                            "podInfoOnMount" => Field::Key_pod_info_on_mount,
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
                let mut value_pod_info_on_mount: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attach_required => value_attach_required = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_info_on_mount => value_pod_info_on_mount = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIDriverSpec {
                    attach_required: value_attach_required,
                    pod_info_on_mount: value_pod_info_on_mount,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIDriverSpec",
            &[
                "attachRequired",
                "podInfoOnMount",
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
            self.pod_info_on_mount.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.attach_required {
            serde::ser::SerializeStruct::serialize_field(&mut state, "attachRequired", value)?;
        }
        if let Some(value) = &self.pod_info_on_mount {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podInfoOnMount", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
