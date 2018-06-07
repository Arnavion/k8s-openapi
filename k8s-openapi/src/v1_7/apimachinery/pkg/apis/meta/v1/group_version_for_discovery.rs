// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery

/// GroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GroupVersionForDiscovery {
    /// groupVersion specifies the API group and version in the form "group/version"
    #[serde(rename = "groupVersion")]
    pub group_version: String,

    /// version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.
    pub version: String,
}
