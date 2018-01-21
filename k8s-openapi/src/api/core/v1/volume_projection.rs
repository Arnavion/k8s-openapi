// Generated from definition io.k8s.api.core.v1.VolumeProjection

/// Projection that may be projected along with other supported volume types
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeProjection {
    /// information about the configMap data to project
    #[serde(rename = "configMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<::api::core::v1::ConfigMapProjection>,

    /// information about the downwardAPI data to project
    #[serde(rename = "downwardAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<::api::core::v1::DownwardAPIProjection>,

    /// information about the secret data to project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<::api::core::v1::SecretProjection>,
}
