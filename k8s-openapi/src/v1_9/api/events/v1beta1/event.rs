// Generated from definition io.k8s.api.events.v1beta1.Event

/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// What action was taken/failed regarding to the regarding object.
    pub action: Option<String>,

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_count: Option<i32>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_first_timestamp: Option<::v1_9::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_last_timestamp: Option<::v1_9::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_source: Option<::v1_9::api::core::v1::EventSource>,

    /// Required. Time when this Event was first observed.
    pub event_time: ::v1_9::apimachinery::pkg::apis::meta::v1::MicroTime,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Optional. A human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    pub note: Option<String>,

    /// Why the action was taken.
    pub reason: Option<String>,

    /// The object this Event is about. In most cases it's an Object reporting controller implements. E.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
    pub regarding: Option<::v1_9::api::core::v1::ObjectReference>,

    /// Optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
    pub related: Option<::v1_9::api::core::v1::ObjectReference>,

    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    pub reporting_controller: Option<String>,

    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    pub reporting_instance: Option<String>,

    /// Data about the Event series this event represents or nil if it's a singleton Event.
    pub series: Option<::v1_9::api::events::v1beta1::EventSeries>,

    /// Type of this event (Normal, Warning), new types could be added in the future.
    pub type_: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action,
            Key_api_version,
            Key_deprecated_count,
            Key_deprecated_first_timestamp,
            Key_deprecated_last_timestamp,
            Key_deprecated_source,
            Key_event_time,
            Key_kind,
            Key_metadata,
            Key_note,
            Key_reason,
            Key_regarding,
            Key_related,
            Key_reporting_controller,
            Key_reporting_instance,
            Key_series,
            Key_type_,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "action" => Field::Key_action,
                            "apiVersion" => Field::Key_api_version,
                            "deprecatedCount" => Field::Key_deprecated_count,
                            "deprecatedFirstTimestamp" => Field::Key_deprecated_first_timestamp,
                            "deprecatedLastTimestamp" => Field::Key_deprecated_last_timestamp,
                            "deprecatedSource" => Field::Key_deprecated_source,
                            "eventTime" => Field::Key_event_time,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "note" => Field::Key_note,
                            "reason" => Field::Key_reason,
                            "regarding" => Field::Key_regarding,
                            "related" => Field::Key_related,
                            "reportingController" => Field::Key_reporting_controller,
                            "reportingInstance" => Field::Key_reporting_instance,
                            "series" => Field::Key_series,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Event;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Event")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_action: Option<String> = None;
                let mut value_api_version: Option<String> = None;
                let mut value_deprecated_count: Option<i32> = None;
                let mut value_deprecated_first_timestamp: Option<::v1_9::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_last_timestamp: Option<::v1_9::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_source: Option<::v1_9::api::core::v1::EventSource> = None;
                let mut value_event_time: Option<::v1_9::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_note: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_regarding: Option<::v1_9::api::core::v1::ObjectReference> = None;
                let mut value_related: Option<::v1_9::api::core::v1::ObjectReference> = None;
                let mut value_reporting_controller: Option<String> = None;
                let mut value_reporting_instance: Option<String> = None;
                let mut value_series: Option<::v1_9::api::events::v1beta1::EventSeries> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action => value_action = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_count => value_deprecated_count = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_first_timestamp => value_deprecated_first_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_last_timestamp => value_deprecated_last_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_source => value_deprecated_source = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_event_time => value_event_time = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_note => value_note = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_regarding => value_regarding = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_related => value_related = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reporting_controller => value_reporting_controller = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reporting_instance => value_reporting_instance = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_series => value_series = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Event {
                    action: value_action,
                    api_version: value_api_version,
                    deprecated_count: value_deprecated_count,
                    deprecated_first_timestamp: value_deprecated_first_timestamp,
                    deprecated_last_timestamp: value_deprecated_last_timestamp,
                    deprecated_source: value_deprecated_source,
                    event_time: value_event_time.ok_or_else(|| ::serde::de::Error::missing_field("eventTime"))?,
                    kind: value_kind,
                    metadata: value_metadata,
                    note: value_note,
                    reason: value_reason,
                    regarding: value_regarding,
                    related: value_related,
                    reporting_controller: value_reporting_controller,
                    reporting_instance: value_reporting_instance,
                    series: value_series,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "Event",
            &[
                "action",
                "apiVersion",
                "deprecatedCount",
                "deprecatedFirstTimestamp",
                "deprecatedLastTimestamp",
                "deprecatedSource",
                "eventTime",
                "kind",
                "metadata",
                "note",
                "reason",
                "regarding",
                "related",
                "reportingController",
                "reportingInstance",
                "series",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Event",
            0 +
            self.action.as_ref().map_or(0, |_| 1) +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.deprecated_count.as_ref().map_or(0, |_| 1) +
            self.deprecated_first_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_last_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_source.as_ref().map_or(0, |_| 1) +
            1 +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.note.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.regarding.as_ref().map_or(0, |_| 1) +
            self.related.as_ref().map_or(0, |_| 1) +
            self.reporting_controller.as_ref().map_or(0, |_| 1) +
            self.reporting_instance.as_ref().map_or(0, |_| 1) +
            self.series.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.action {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "action", value)?;
        }
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.deprecated_count {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedCount", value)?;
        }
        if let Some(value) = &self.deprecated_first_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedFirstTimestamp", value)?;
        }
        if let Some(value) = &self.deprecated_last_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedLastTimestamp", value)?;
        }
        if let Some(value) = &self.deprecated_source {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedSource", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "eventTime", &self.event_time)?;
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.note {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "note", value)?;
        }
        if let Some(value) = &self.reason {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.regarding {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "regarding", value)?;
        }
        if let Some(value) = &self.related {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "related", value)?;
        }
        if let Some(value) = &self.reporting_controller {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reportingController", value)?;
        }
        if let Some(value) = &self.reporting_instance {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reportingInstance", value)?;
        }
        if let Some(value) = &self.series {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "series", value)?;
        }
        if let Some(value) = &self.type_ {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
