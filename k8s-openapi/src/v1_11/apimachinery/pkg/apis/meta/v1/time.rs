// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Time

/// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
pub type Time = ::chrono::DateTime<::chrono::Utc>;
