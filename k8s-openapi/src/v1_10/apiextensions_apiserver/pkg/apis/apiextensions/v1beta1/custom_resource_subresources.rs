// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceSubresources

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomResourceSubresources {
    /// Scale denotes the scale subresource for CustomResources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale>,

    /// Status denotes the status subresource for CustomResources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus>,
}
