// Generated from definition io.k8s.api.authentication.v1beta1.UserInfo

/// UserInfo holds the information about the user needed to implement the user.Info interface.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserInfo {
    /// Any additional information provided by the authenticator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<::std::collections::BTreeMap<String, Vec<String>>>,

    /// The names of groups this user is a part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,

    /// A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The name that uniquely identifies this user among all active users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
