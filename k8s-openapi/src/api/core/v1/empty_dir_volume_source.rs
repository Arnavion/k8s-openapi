// Generated from definition io.k8s.api.core.v1.EmptyDirVolumeSource

/// Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EmptyDirVolumeSource {
    /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,

    /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
    #[serde(rename = "sizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<::apimachinery::pkg::api::resource::Quantity>,
}
