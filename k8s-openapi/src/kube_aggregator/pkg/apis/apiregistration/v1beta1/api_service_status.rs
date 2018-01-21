// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceStatus

/// APIServiceStatus contains derived information about an API server
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIServiceStatus {
    /// Current service state of apiService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceCondition>>,
}
