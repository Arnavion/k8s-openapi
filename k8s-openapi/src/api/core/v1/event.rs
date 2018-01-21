// Generated from definition io.k8s.api.core.v1.Event

/// Event is a report of an event somewhere in the cluster.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Event {
    /// What action was taken/failed regarding to the Regarding object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// The number of times this event has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Time when this Event was first observed.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
    #[serde(rename = "firstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// The object that this event is about.
    #[serde(rename = "involvedObject")]
    pub involved_object: ::api::core::v1::ObjectReference,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The time at which the most recent occurrence of this event was recorded.
    #[serde(rename = "lastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human-readable description of the status of this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: ::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Optional secondary object for more complex actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<::api::core::v1::ObjectReference>,

    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    #[serde(rename = "reportingComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_component: Option<String>,

    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    #[serde(rename = "reportingInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<String>,

    /// Data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<::api::core::v1::EventSeries>,

    /// The component reporting this event. Should be a short machine understandable string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<::api::core::v1::EventSource>,

    /// Type of this event (Normal, Warning), new types could be added in the future
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
