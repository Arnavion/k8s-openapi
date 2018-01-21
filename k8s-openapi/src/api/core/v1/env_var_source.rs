// Generated from definition io.k8s.api.core.v1.EnvVarSource

/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnvVarSource {
    /// Selects a key of a ConfigMap.
    #[serde(rename = "configMapKeyRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<::api::core::v1::ConfigMapKeySelector>,

    /// Selects a field of the pod: supports metadata.name, metadata.namespace, metadata.labels, metadata.annotations, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP.
    #[serde(rename = "fieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<::api::core::v1::ObjectFieldSelector>,

    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(rename = "resourceFieldRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<::api::core::v1::ResourceFieldSelector>,

    /// Selects a key of a secret in the pod's namespace
    #[serde(rename = "secretKeyRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<::api::core::v1::SecretKeySelector>,
}
