// Generated from definition io.k8s.api.autoscaling.v2beta1.CrossVersionObjectReference

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CrossVersionObjectReference {
    /// API version of the referent
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds"
    pub kind: String,

    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}
