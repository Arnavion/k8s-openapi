// Generated from definition io.k8s.api.events.v1beta1.EventSeries

/// EventSeries contain information on series of events, i.e. thing that was/is happening continously for some time.
#[derive(Debug, Deserialize, Serialize)]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time
    pub count: i32,

    /// Time when last Event from the series was seen before last heartbeat.
    #[serde(rename = "lastObservedTime")]
    pub last_observed_time: ::v1_9::apimachinery::pkg::apis::meta::v1::MicroTime,

    /// Information whether this series is ongoing or finished.
    pub state: String,
}
