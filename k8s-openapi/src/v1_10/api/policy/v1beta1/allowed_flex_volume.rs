// Generated from definition io.k8s.api.policy.v1beta1.AllowedFlexVolume

/// AllowedFlexVolume represents a single Flexvolume that is allowed to be used.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AllowedFlexVolume {
    /// Driver is the name of the Flexvolume driver.
    pub driver: String,
}
