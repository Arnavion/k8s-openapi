// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails

/// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StatusDetails {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<Vec<::apimachinery::pkg::apis::meta::v1::StatusCause>>,

    /// The group attribute of the resource associated with the status StatusReason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    #[serde(rename = "retryAfterSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,

    /// UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
