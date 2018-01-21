// Generated from definition io.k8s.api.core.v1.EnvFromSource

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnvFromSource {
    /// The ConfigMap to select from
    #[serde(rename = "configMapRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<::api::core::v1::ConfigMapEnvSource>,

    /// An optional identifer to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The Secret to select from
    #[serde(rename = "secretRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<::api::core::v1::SecretEnvSource>,
}
