// Generated from definition io.k8s.kubernetes.pkg.api.v1.VolumeProjection

/// Projection that may be projected along with other supported volume types
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VolumeProjection {
    /// information about the configMap data to project
    #[serde(rename = "configMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<::v1_7::kubernetes::pkg::api::v1::ConfigMapProjection>,

    /// information about the downwardAPI data to project
    #[serde(rename = "downwardAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<::v1_7::kubernetes::pkg::api::v1::DownwardAPIProjection>,

    /// information about the secret data to project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<::v1_7::kubernetes::pkg::api::v1::SecretProjection>,
}
