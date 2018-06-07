// Generated from definition io.k8s.api.core.v1.ContainerImage

/// Describe a container image
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContainerImage {
    /// Names by which this image is known. e.g. ["k8s.gcr.io/hyperkube:v1.0.7", "dockerhub.io/google_containers/hyperkube:v1.0.7"]
    pub names: Vec<String>,

    /// The size of the image in bytes.
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}
