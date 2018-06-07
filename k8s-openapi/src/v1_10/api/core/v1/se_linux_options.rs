// Generated from definition io.k8s.api.core.v1.SELinuxOptions

/// SELinuxOptions are the labels to be applied to the container
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SELinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// Role is a SELinux role label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Type is a SELinux type label that applies to the container.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// User is a SELinux user label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
