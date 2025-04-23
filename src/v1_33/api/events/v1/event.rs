// Generated from definition io.k8s.api.events.v1.Event

/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system. Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    /// action is what action was taken/failed regarding to the regarding object. It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters.
    pub action: Option<std::string::String>,

    /// deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.
    pub deprecated_count: Option<i32>,

    /// deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    pub deprecated_first_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    pub deprecated_last_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
    pub deprecated_source: Option<crate::api::core::v1::EventSource>,

    /// eventTime is the time when this Event was first observed. It is required.
    pub event_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// note is a human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    pub note: Option<std::string::String>,

    /// reason is why the action was taken. It is human-readable. This field cannot be empty for new Events and it can have at most 128 characters.
    pub reason: Option<std::string::String>,

    /// regarding contains the object this Event is about. In most cases it's an Object reporting controller implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
    pub regarding: Option<crate::api::core::v1::ObjectReference>,

    /// related is the optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
    pub related: Option<crate::api::core::v1::ObjectReference>,

    /// reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events.
    pub reporting_controller: Option<std::string::String>,

    /// reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`. This field cannot be empty for new Events and it can have at most 128 characters.
    pub reporting_instance: Option<std::string::String>,

    /// series is data about the Event series this event represents or nil if it's a singleton Event.
    pub series: Option<crate::api::events::v1::EventSeries>,

    /// type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events.
    pub type_: Option<std::string::String>,
}

impl crate::Resource for Event {
    const API_VERSION: &'static str = "events.k8s.io/v1";
    const GROUP: &'static str = "events.k8s.io";
    const KIND: &'static str = "Event";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "events";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for Event {
    const LIST_KIND: &'static str = "EventList";
}

impl crate::Metadata for Event {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for Event {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.action, other.action);
        crate::DeepMerge::merge_from(&mut self.deprecated_count, other.deprecated_count);
        crate::DeepMerge::merge_from(&mut self.deprecated_first_timestamp, other.deprecated_first_timestamp);
        crate::DeepMerge::merge_from(&mut self.deprecated_last_timestamp, other.deprecated_last_timestamp);
        crate::DeepMerge::merge_from(&mut self.deprecated_source, other.deprecated_source);
        crate::DeepMerge::merge_from(&mut self.event_time, other.event_time);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.note, other.note);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.regarding, other.regarding);
        crate::DeepMerge::merge_from(&mut self.related, other.related);
        crate::DeepMerge::merge_from(&mut self.reporting_controller, other.reporting_controller);
        crate::DeepMerge::merge_from(&mut self.reporting_instance, other.reporting_instance);
        crate::DeepMerge::merge_from(&mut self.series, other.series);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_action,
            Key_deprecated_count,
            Key_deprecated_first_timestamp,
            Key_deprecated_last_timestamp,
            Key_deprecated_source,
            Key_event_time,
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

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "action" => Field::Key_action,
                            "deprecatedCount" => Field::Key_deprecated_count,
                            "deprecatedFirstTimestamp" => Field::Key_deprecated_first_timestamp,
                            "deprecatedLastTimestamp" => Field::Key_deprecated_last_timestamp,
                            "deprecatedSource" => Field::Key_deprecated_source,
                            "eventTime" => Field::Key_event_time,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Event;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_action: Option<std::string::String> = None;
                let mut value_deprecated_count: Option<i32> = None;
                let mut value_deprecated_first_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_last_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_source: Option<crate::api::core::v1::EventSource> = None;
                let mut value_event_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_note: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_regarding: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_related: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_reporting_controller: Option<std::string::String> = None;
                let mut value_reporting_instance: Option<std::string::String> = None;
                let mut value_series: Option<crate::api::events::v1::EventSeries> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_action => value_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_count => value_deprecated_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_first_timestamp => value_deprecated_first_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_last_timestamp => value_deprecated_last_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_source => value_deprecated_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_event_time => value_event_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_note => value_note = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_regarding => value_regarding = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_related => value_related = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reporting_controller => value_reporting_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reporting_instance => value_reporting_instance = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_series => value_series = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Event {
                    action: value_action,
                    deprecated_count: value_deprecated_count,
                    deprecated_first_timestamp: value_deprecated_first_timestamp,
                    deprecated_last_timestamp: value_deprecated_last_timestamp,
                    deprecated_source: value_deprecated_source,
                    event_time: value_event_time,
                    metadata: value_metadata.unwrap_or_default(),
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
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "action",
                "deprecatedCount",
                "deprecatedFirstTimestamp",
                "deprecatedLastTimestamp",
                "deprecatedSource",
                "eventTime",
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

impl crate::serde::Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.action.as_ref().map_or(0, |_| 1) +
            self.deprecated_count.as_ref().map_or(0, |_| 1) +
            self.deprecated_first_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_last_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_source.as_ref().map_or(0, |_| 1) +
            self.event_time.as_ref().map_or(0, |_| 1) +
            self.note.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.regarding.as_ref().map_or(0, |_| 1) +
            self.related.as_ref().map_or(0, |_| 1) +
            self.reporting_controller.as_ref().map_or(0, |_| 1) +
            self.reporting_instance.as_ref().map_or(0, |_| 1) +
            self.series.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.action {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "action", value)?;
        }
        if let Some(value) = &self.deprecated_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedCount", value)?;
        }
        if let Some(value) = &self.deprecated_first_timestamp {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedFirstTimestamp", value)?;
        }
        if let Some(value) = &self.deprecated_last_timestamp {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedLastTimestamp", value)?;
        }
        if let Some(value) = &self.deprecated_source {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedSource", value)?;
        }
        if let Some(value) = &self.event_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "eventTime", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.note {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "note", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.regarding {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "regarding", value)?;
        }
        if let Some(value) = &self.related {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "related", value)?;
        }
        if let Some(value) = &self.reporting_controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reportingController", value)?;
        }
        if let Some(value) = &self.reporting_instance {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reportingInstance", value)?;
        }
        if let Some(value) = &self.series {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "series", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Event {
    fn schema_name() -> std::string::String {
        "io.k8s.api.events.v1.Event".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system. Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "action".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("action is what action was taken/failed regarding to the regarding object. It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "apiVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deprecatedCount".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deprecatedFirstTimestamp".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deprecatedLastTimestamp".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deprecatedSource".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EventSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "eventTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("eventTime is the time when this Event was first observed. It is required.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "kind".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "note".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("note is a human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reason is why the action was taken. It is human-readable. This field cannot be empty for new Events and it can have at most 128 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "regarding".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("regarding contains the object this Event is about. In most cases it's an Object reporting controller implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "related".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("related is the optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "reportingController".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reportingInstance".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`. This field cannot be empty for new Events and it can have at most 128 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "series".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::events::v1::EventSeries>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("series is data about the Event series this event represents or nil if it's a singleton Event.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "metadata".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
