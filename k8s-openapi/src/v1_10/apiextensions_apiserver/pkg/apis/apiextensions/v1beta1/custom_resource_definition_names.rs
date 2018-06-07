// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionNames

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomResourceDefinitionNames {
    /// Categories is a list of grouped resources custom resources belong to (e.g. 'all')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// Kind is the serialized kind of the resource.  It is normally CamelCase and singular.
    pub kind: String,

    /// ListKind is the serialized kind of the list for this resource.  Defaults to <kind>List.
    #[serde(rename = "listKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kind: Option<String>,

    /// Plural is the plural name of the resource to serve.  It must match the name of the CustomResourceDefinition-registration too: plural.group and it must be all lowercase.
    pub plural: String,

    /// ShortNames are short names for the resource.  It must be all lowercase.
    #[serde(rename = "shortNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_names: Option<Vec<String>>,

    /// Singular is the singular name of the resource.  It must be all lowercase  Defaults to lowercased <kind>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<String>,
}
