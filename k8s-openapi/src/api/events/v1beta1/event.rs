// Generated from definition io.k8s.api.events.v1beta1.Event

/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system.
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// What action was taken/failed regarding to the regarding object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    #[serde(rename = "deprecatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_count: Option<i32>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    #[serde(rename = "deprecatedFirstTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    #[serde(rename = "deprecatedLastTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    #[serde(rename = "deprecatedSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_source: Option<::api::core::v1::EventSource>,

    /// Required. Time when this Event was first observed.
    #[serde(rename = "eventTime")]
    pub event_time: ::apimachinery::pkg::apis::meta::v1::MicroTime,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Optional. A human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Why the action was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// The object this Event is about. In most cases it's an Object reporting controller implements. E.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regarding: Option<::api::core::v1::ObjectReference>,

    /// Optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<::api::core::v1::ObjectReference>,

    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    #[serde(rename = "reportingController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_controller: Option<String>,

    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    #[serde(rename = "reportingInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<String>,

    /// Data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<::api::events::v1beta1::EventSeries>,

    /// Type of this event (Normal, Warning), new types could be added in the future.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
