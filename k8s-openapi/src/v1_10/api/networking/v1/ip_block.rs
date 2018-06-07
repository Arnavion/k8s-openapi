// Generated from definition io.k8s.api.networking.v1.IPBlock

/// IPBlock describes a particular CIDR (Ex. "192.168.1.1/24") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IPBlock {
    /// CIDR is a string representing the IP Block Valid examples are "192.168.1.1/24"
    pub cidr: String,

    /// Except is a slice of CIDRs that should not be included within an IP Block Valid examples are "192.168.1.1/24" Except values will be rejected if they are outside the CIDR range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}
