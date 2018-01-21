// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionStatus

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomResourceDefinitionStatus {
    /// AcceptedNames are the names that are actually being used to serve discovery They may be different than the names in spec.
    #[serde(rename = "acceptedNames")]
    pub accepted_names: ::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames,

    /// Conditions indicate state for particular aspects of a CustomResourceDefinition
    pub conditions: Vec<::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition>,
}
