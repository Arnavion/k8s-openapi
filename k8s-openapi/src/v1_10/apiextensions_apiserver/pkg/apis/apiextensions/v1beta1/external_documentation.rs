// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.ExternalDocumentation

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExternalDocumentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
