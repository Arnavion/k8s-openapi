// Generated from definition io.k8s.api.core.v1.EventSeries

/// EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Time of the last occurrence observed
    #[serde(rename = "lastObservedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_time: Option<::v1_10::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// State of this Series: Ongoing or Finished
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
