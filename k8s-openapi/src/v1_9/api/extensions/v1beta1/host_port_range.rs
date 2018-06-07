// Generated from definition io.k8s.api.extensions.v1beta1.HostPortRange

/// Host Port Range defines a range of host ports that will be enabled by a policy for pods to use.  It requires both the start and end to be defined.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HostPortRange {
    /// max is the end of the range, inclusive.
    pub max: i32,

    /// min is the start of the range, inclusive.
    pub min: i32,
}
