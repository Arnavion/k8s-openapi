// Generated from definition io.k8s.api.events.v1beta1.Event

/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// What action was taken/failed regarding to the regarding object.
    pub action: Option<String>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_count: Option<i32>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_first_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_last_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Deprecated field assuring backward compatibility with core.v1 Event type
    pub deprecated_source: Option<crate::api::core::v1::EventSource>,

    /// Required. Time when this Event was first observed.
    pub event_time: crate::apimachinery::pkg::apis::meta::v1::MicroTime,

    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Optional. A human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    pub note: Option<String>,

    /// Why the action was taken.
    pub reason: Option<String>,

    /// The object this Event is about. In most cases it's an Object reporting controller implements. E.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
    pub regarding: Option<crate::api::core::v1::ObjectReference>,

    /// Optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
    pub related: Option<crate::api::core::v1::ObjectReference>,

    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    pub reporting_controller: Option<String>,

    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    pub reporting_instance: Option<String>,

    /// Data about the Event series this event represents or nil if it's a singleton Event.
    pub series: Option<crate::api::events::v1beta1::EventSeries>,

    /// Type of this event (Normal, Warning), new types could be added in the future.
    pub type_: Option<String>,
}

// Begin events.k8s.io/v1beta1/Event

// Generated from operation createEventsV1beta1NamespacedEvent

impl Event {
    /// create an Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::CreateResponse`]`<Self>>` constructor, or [`crate::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_namespaced_event(
        namespace: &str,
        body: &crate::api::events::v1beta1::Event,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteEventsV1beta1CollectionNamespacedEvent

impl Event {
    /// delete collection of Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>>` constructor, or [`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection_namespaced_event(
        namespace: &str,
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<crate::List<Self>>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteEventsV1beta1NamespacedEvent

impl Event {
    /// delete an Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<Self>>` constructor, or [`crate::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Event
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_namespaced_event(
        name: &str,
        namespace: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listEventsV1beta1EventForAllNamespaces

impl Event {
    /// list or watch objects of kind Event
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ListResponse`]`<Self>>` constructor, or [`crate::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_event_for_all_namespaces(
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = "/apis/events.k8s.io/v1beta1/events?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listEventsV1beta1NamespacedEvent

impl Event {
    /// list or watch objects of kind Event
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ListResponse`]`<Self>>` constructor, or [`crate::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_namespaced_event(
        namespace: &str,
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchEventsV1beta1NamespacedEvent

impl Event {
    /// partially update the specified Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Event
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_namespaced_event(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation readEventsV1beta1NamespacedEvent

impl Event {
    /// read the specified Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadNamespacedEventResponse`]`>` constructor, or [`ReadNamespacedEventResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Event
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_namespaced_event(
        name: &str,
        namespace: &str,
        optional: ReadNamespacedEventOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadNamespacedEventResponse>), crate::RequestError> {
        let ReadNamespacedEventOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Event::read_namespaced_event`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadNamespacedEventOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadNamespacedEventResponse as Response>::try_from_parts` to parse the HTTP response body of [`Event::read_namespaced_event`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadNamespacedEventResponse {
    Ok(crate::api::events::v1beta1::Event),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadNamespacedEventResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadNamespacedEventResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadNamespacedEventResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceEventsV1beta1NamespacedEvent

impl Event {
    /// replace the specified Event
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Event
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_namespaced_event(
        name: &str,
        namespace: &str,
        body: &crate::api::events::v1beta1::Event,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchEventsV1beta1EventForAllNamespaces

impl Event {
    /// list or watch objects of kind Event
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::WatchResponse`]`<Self>>` constructor, or [`crate::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_event_for_all_namespaces(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = "/apis/events.k8s.io/v1beta1/events?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchEventsV1beta1NamespacedEvent

impl Event {
    /// list or watch objects of kind Event
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::WatchResponse`]`<Self>>` constructor, or [`crate::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_namespaced_event(
        namespace: &str,
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/events.k8s.io/v1beta1/namespaces/{namespace}/events?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End events.k8s.io/v1beta1/Event

impl crate::Resource for Event {
    const API_VERSION: &'static str = "events.k8s.io/v1beta1";
    const GROUP: &'static str = "events.k8s.io";
    const KIND: &'static str = "Event";
    const VERSION: &'static str = "v1beta1";
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_action: Option<String> = None;
                let mut value_deprecated_count: Option<i32> = None;
                let mut value_deprecated_first_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_last_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deprecated_source: Option<crate::api::core::v1::EventSource> = None;
                let mut value_event_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_note: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_regarding: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_related: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_reporting_controller: Option<String> = None;
                let mut value_reporting_instance: Option<String> = None;
                let mut value_series: Option<crate::api::events::v1beta1::EventSeries> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_action => value_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_count => value_deprecated_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_first_timestamp => value_deprecated_first_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_last_timestamp => value_deprecated_last_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_source => value_deprecated_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_event_time => value_event_time = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metadata => value_metadata = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
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
                    event_time: value_event_time.ok_or_else(|| crate::serde::de::Error::missing_field("eventTime"))?,
                    metadata: value_metadata.ok_or_else(|| crate::serde::de::Error::missing_field("metadata"))?,
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
            4 +
            self.action.as_ref().map_or(0, |_| 1) +
            self.deprecated_count.as_ref().map_or(0, |_| 1) +
            self.deprecated_first_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_last_timestamp.as_ref().map_or(0, |_| 1) +
            self.deprecated_source.as_ref().map_or(0, |_| 1) +
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
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "eventTime", &self.event_time)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for Event {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system.",
          "x-kubernetes-group-version-kind": [
            {
              "group": "events.k8s.io",
              "kind": "Event",
              "version": "v1beta1"
            }
          ],
          "properties": {
            "action": {
              "description": "What action was taken/failed regarding to the regarding object.",
              "type": "string"
            },
            "apiVersion": {
              "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources",
              "type": "string"
            },
            "deprecatedCount": {
              "description": "Deprecated field assuring backward compatibility with core.v1 Event type",
              "format": "int32",
              "type": "integer"
            },
            "deprecatedFirstTimestamp": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Deprecated field assuring backward compatibility with core.v1 Event type"),
            "deprecatedLastTimestamp": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Deprecated field assuring backward compatibility with core.v1 Event type"),
            "deprecatedSource": crate::schema_ref_with_description(crate::api::core::v1::EventSource::schema(), "Deprecated field assuring backward compatibility with core.v1 Event type"),
            "eventTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::MicroTime::schema(), "Required. Time when this Event was first observed."),
            "kind": {
              "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
              "type": "string"
            },
            "metadata": crate::apimachinery::pkg::apis::meta::v1::ObjectMeta::schema(),
            "note": {
              "description": "Optional. A human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.",
              "type": "string"
            },
            "reason": {
              "description": "Why the action was taken.",
              "type": "string"
            },
            "regarding": crate::schema_ref_with_description(crate::api::core::v1::ObjectReference::schema(), "The object this Event is about. In most cases it's an Object reporting controller implements. E.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object."),
            "related": crate::schema_ref_with_description(crate::api::core::v1::ObjectReference::schema(), "Optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object."),
            "reportingController": {
              "description": "Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.",
              "type": "string"
            },
            "reportingInstance": {
              "description": "ID of the controller instance, e.g. `kubelet-xyzf`.",
              "type": "string"
            },
            "series": crate::schema_ref_with_description(crate::api::events::v1beta1::EventSeries::schema(), "Data about the Event series this event represents or nil if it's a singleton Event."),
            "type": {
              "description": "Type of this event (Normal, Warning), new types could be added in the future.",
              "type": "string"
            }
          },
          "required": [
            "eventTime",
            "metadata"
          ],
          "type": "object"
        })
    }
}
