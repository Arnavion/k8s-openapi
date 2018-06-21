// Generated from definition io.k8s.kubernetes.pkg.api.v1.Event

/// Event is a report of an event somewhere in the cluster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// The number of times this event has occurred.
    pub count: Option<i32>,

    /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
    pub first_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time>,

    /// The object that this event is about.
    pub involved_object: ::v1_7::kubernetes::pkg::api::v1::ObjectReference,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// The time at which the most recent occurrence of this event was recorded.
    pub last_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human-readable description of the status of this operation.
    pub message: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: ::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
    pub reason: Option<String>,

    /// The component reporting this event. Should be a short machine understandable string.
    pub source: Option<::v1_7::kubernetes::pkg::api::v1::EventSource>,

    /// Type of this event (Normal, Warning), new types could be added in the future
    pub type_: Option<String>,
}

// Begin /v1/Event

// Generated from operation createCoreV1NamespacedEvent

#[derive(Debug)]
pub enum CreateCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Event),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// create an Event
    pub fn create_core_v1_namespaced_event<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Event,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1NamespacedEventResponse::Unauthorized(response),
            other => CreateCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedEvent

#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// delete collection of Event
    pub fn delete_core_v1_collection_namespaced_event<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteCoreV1CollectionNamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCoreV1CollectionNamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1CollectionNamespacedEventResponse::Unauthorized(response),
            other => DeleteCoreV1CollectionNamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1NamespacedEvent

#[derive(Debug)]
pub enum DeleteCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// delete an Event
    pub fn delete_core_v1_namespaced_event<C>(
        __client: &C,
        // name of the Event
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(grace_period_seconds) = grace_period_seconds {
                __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
            }
            if let Some(orphan_dependents) = orphan_dependents {
                __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(propagation_policy) = propagation_policy {
                __query_pairs.append_pair("propagationPolicy", &propagation_policy);
            }
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeleteCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1NamespacedEventResponse::Unauthorized(response),
            other => DeleteCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1EventForAllNamespaces

#[derive(Debug)]
pub enum ListCoreV1EventForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::EventList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// list or watch objects of kind Event
    pub fn list_core_v1_event_for_all_namespaces<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1EventForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/events")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListCoreV1EventForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1EventForAllNamespacesResponse::Unauthorized(response),
            other => ListCoreV1EventForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1NamespacedEvent

#[derive(Debug)]
pub enum ListCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::EventList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// list or watch objects of kind Event
    pub fn list_core_v1_namespaced_event<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1NamespacedEventResponse::Unauthorized(response),
            other => ListCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1NamespacedEvent

#[derive(Debug)]
pub enum PatchCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Event),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// partially update the specified Event
    pub fn patch_core_v1_namespaced_event<C>(
        __client: &C,
        // name of the Event
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedEventResponse::Unauthorized(response),
            other => PatchCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1NamespacedEvent

#[derive(Debug)]
pub enum ReadCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Event),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// read the specified Event
    pub fn read_core_v1_namespaced_event<C>(
        __client: &C,
        // name of the Event
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(exact) = exact {
                __query_pairs.append_pair("exact", &exact.to_string());
            }
            if let Some(export) = export {
                __query_pairs.append_pair("export", &export.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedEventResponse::Unauthorized(response),
            other => ReadCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1NamespacedEvent

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::Event),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// replace the specified Event
    pub fn replace_core_v1_namespaced_event<C>(
        __client: &C,
        // name of the Event
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Event,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/namespaces/{namespace}/events/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedEventResponse::Unauthorized(response),
            other => ReplaceCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1EventListForAllNamespaces

pub enum WatchCoreV1EventListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// watch individual changes to a list of Event
    pub fn watch_core_v1_event_list_for_all_namespaces<C>(
        __client: &C,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1EventListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/events")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1EventListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1EventListForAllNamespacesResponse::Unauthorized(response),
            other => WatchCoreV1EventListForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1NamespacedEvent

pub enum WatchCoreV1NamespacedEventResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// watch changes to an object of kind Event
    pub fn watch_core_v1_namespaced_event<C>(
        __client: &C,
        // name of the Event
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedEventResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/events/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1NamespacedEventResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedEventResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedEventResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1NamespacedEventList

pub enum WatchCoreV1NamespacedEventListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Event {
    /// watch individual changes to a list of Event
    pub fn watch_core_v1_namespaced_event_list<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchCoreV1NamespacedEventListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/namespaces/{namespace}/events", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
            if let Some(resource_version) = resource_version {
                __query_pairs.append_pair("resourceVersion", &resource_version);
            }
            if let Some(timeout_seconds) = timeout_seconds {
                __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
            }
            if let Some(watch) = watch {
                __query_pairs.append_pair("watch", &watch.to_string());
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchCoreV1NamespacedEventListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedEventListResponse::Unauthorized(response),
            other => WatchCoreV1NamespacedEventListResponse::Other(other, response),
        })
    }
}

// End /v1/Event

impl<'de> ::serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_count,
            Key_first_timestamp,
            Key_involved_object,
            Key_kind,
            Key_last_timestamp,
            Key_message,
            Key_metadata,
            Key_reason,
            Key_source,
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
                            "apiVersion" => Field::Key_api_version,
                            "count" => Field::Key_count,
                            "firstTimestamp" => Field::Key_first_timestamp,
                            "involvedObject" => Field::Key_involved_object,
                            "kind" => Field::Key_kind,
                            "lastTimestamp" => Field::Key_last_timestamp,
                            "message" => Field::Key_message,
                            "metadata" => Field::Key_metadata,
                            "reason" => Field::Key_reason,
                            "source" => Field::Key_source,
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
                let mut value_api_version: Option<String> = None;
                let mut value_count: Option<i32> = None;
                let mut value_first_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_involved_object: Option<::v1_7::kubernetes::pkg::api::v1::ObjectReference> = None;
                let mut value_kind: Option<String> = None;
                let mut value_last_timestamp: Option<::v1_7::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_reason: Option<String> = None;
                let mut value_source: Option<::v1_7::kubernetes::pkg::api::v1::EventSource> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_count => value_count = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_first_timestamp => value_first_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_involved_object => value_involved_object = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_timestamp => value_last_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reason => value_reason = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source => value_source = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Event {
                    api_version: value_api_version,
                    count: value_count,
                    first_timestamp: value_first_timestamp,
                    involved_object: value_involved_object.ok_or_else(|| ::serde::de::Error::missing_field("involvedObject"))?,
                    kind: value_kind,
                    last_timestamp: value_last_timestamp,
                    message: value_message,
                    metadata: value_metadata.ok_or_else(|| ::serde::de::Error::missing_field("metadata"))?,
                    reason: value_reason,
                    source: value_source,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "Event",
            &[
                "apiVersion",
                "count",
                "firstTimestamp",
                "involvedObject",
                "kind",
                "lastTimestamp",
                "message",
                "metadata",
                "reason",
                "source",
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
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.count.as_ref().map_or(0, |_| 1) +
            self.first_timestamp.as_ref().map_or(0, |_| 1) +
            1 +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.last_timestamp.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            1 +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.source.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.count {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        if let Some(value) = &self.first_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "firstTimestamp", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "involvedObject", &self.involved_object)?;
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.last_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTimestamp", value)?;
        }
        if let Some(value) = &self.message {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.reason {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.source {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "source", value)?;
        }
        if let Some(value) = &self.type_ {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
