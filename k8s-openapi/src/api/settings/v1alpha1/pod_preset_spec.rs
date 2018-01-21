// Generated from definition io.k8s.api.settings.v1alpha1.PodPresetSpec

/// PodPresetSpec is a description of a pod preset.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodPresetSpec {
    /// Env defines the collection of EnvVar to inject into containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<::api::core::v1::EnvVar>>,

    /// EnvFrom defines the collection of EnvFromSource to inject into containers.
    #[serde(rename = "envFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_from: Option<Vec<::api::core::v1::EnvFromSource>>,

    /// Selector is a label query over a set of resources, in this case pods. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// VolumeMounts defines the collection of VolumeMount to inject into containers.
    #[serde(rename = "volumeMounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<::api::core::v1::VolumeMount>>,

    /// Volumes defines the collection of Volume to inject into the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<::api::core::v1::Volume>>,
}
