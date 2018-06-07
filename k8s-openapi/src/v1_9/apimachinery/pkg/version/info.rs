// Generated from definition io.k8s.apimachinery.pkg.version.Info

/// Info contains versioning information. how we'll want to distribute that information.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Info {
    #[serde(rename = "buildDate")]
    pub build_date: String,

    pub compiler: String,

    #[serde(rename = "gitCommit")]
    pub git_commit: String,

    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,

    #[serde(rename = "gitVersion")]
    pub git_version: String,

    #[serde(rename = "goVersion")]
    pub go_version: String,

    pub major: String,

    pub minor: String,

    pub platform: String,
}
