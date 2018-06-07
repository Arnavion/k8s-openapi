// Generated from definition io.k8s.api.storage.v1beta1.VolumeAttachmentSource

/// VolumeAttachmentSource represents a volume that should be attached. Right now only PersistenVolumes can be attached via external attacher, in future we may allow also inline volumes in pods. Exactly one member can be set.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeAttachmentSource {
    /// Name of the persistent volume to attach.
    #[serde(rename = "persistentVolumeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<String>,
}
