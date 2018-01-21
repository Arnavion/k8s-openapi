// Generated from definition io.k8s.api.storage.v1alpha1.VolumeAttachmentSpec

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
    pub attacher: String,

    /// The node that the volume should be attached to.
    #[serde(rename = "nodeName")]
    pub node_name: String,

    /// Source represents the volume that should be attached.
    pub source: ::api::storage::v1alpha1::VolumeAttachmentSource,
}
