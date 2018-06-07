// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,

    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
