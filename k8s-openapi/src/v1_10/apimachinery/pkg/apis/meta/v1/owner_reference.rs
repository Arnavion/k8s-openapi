// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference

/// OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OwnerReference {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    #[serde(rename = "blockOwnerDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,

    /// If true, this reference points to the managing controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: String,

    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,

    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: String,
}
