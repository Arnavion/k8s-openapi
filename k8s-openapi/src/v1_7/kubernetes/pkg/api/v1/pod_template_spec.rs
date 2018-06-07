// Generated from definition io.k8s.kubernetes.pkg.api.v1.PodTemplateSpec

/// PodTemplateSpec describes the data a pod should have when created from a template
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodTemplateSpec {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::PodSpec>,
}
