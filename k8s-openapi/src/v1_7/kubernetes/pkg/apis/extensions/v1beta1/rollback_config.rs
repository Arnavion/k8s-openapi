// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.RollbackConfig

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
