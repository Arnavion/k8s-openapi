// Generated from definition io.k8s.api.apps.v1beta2.StatefulSet

/// DEPRECATED - This group version of StatefulSet is deprecated by apps/v1/StatefulSet. See the release notes for more information. StatefulSet represents a set of pods with consistent identities. Identities are defined as:
///  - Network: A single stable DNS and hostname.
///  - Storage: As many VolumeClaims as requested.
/// The StatefulSet guarantees that a given network identity will always map to the same storage identity.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatefulSet {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired identities of pods in this set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::api::apps::v1beta2::StatefulSetSpec>,

    /// Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::apps::v1beta2::StatefulSetStatus>,
}
