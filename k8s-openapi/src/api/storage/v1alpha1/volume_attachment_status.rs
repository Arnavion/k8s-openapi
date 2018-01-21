// Generated from definition io.k8s.api.storage.v1alpha1.VolumeAttachmentStatus

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeAttachmentStatus {
    /// The last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    #[serde(rename = "attachError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<::api::storage::v1alpha1::VolumeError>,

    /// Indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attached: bool,

    /// Upon successful attach, this field is populated with any information returned by the attach operation that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    #[serde(rename = "attachmentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_metadata: Option<::std::collections::BTreeMap<String, String>>,

    /// The last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.
    #[serde(rename = "detachError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<::api::storage::v1alpha1::VolumeError>,
}
