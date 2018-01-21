// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_expressions: Option<Vec<::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>>,

    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_labels: Option<::std::collections::BTreeMap<String, String>>,
}
