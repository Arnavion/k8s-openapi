// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta

/// ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListMeta {
    /// String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency
    #[serde(rename = "resourceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// SelfLink is a URL representing this object. Populated by the system. Read-only.
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}
