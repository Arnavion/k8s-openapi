// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService

/// APIService represents a server for a particular GroupVersion. Name must be "version.group".
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIService {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec contains information for locating and communicating with a server
    pub spec: Option<::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec>,

    /// Status contains derived information about an API server
    pub status: Option<::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus>,
}

// Begin apiregistration.k8s.io/v1beta1/APIService

// Generated from operation createApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum CreateApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Accepted(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// create an APIService
    pub fn create_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        body: &::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices")).map_err(::Error::URL)?;
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
                CreateApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateApiregistrationV1beta1APIServiceResponse::Created(result)
            },
            ::http::StatusCode::ACCEPTED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateApiregistrationV1beta1APIServiceResponse::Accepted(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => CreateApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum DeleteApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// delete an APIService
    pub fn delete_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => DeleteApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteApiregistrationV1beta1CollectionAPIService

#[derive(Debug)]
pub enum DeleteApiregistrationV1beta1CollectionAPIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// delete collection of APIService
    pub fn delete_apiregistration_v1beta1_collection_api_service<C>(
        __client: &C,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<DeleteApiregistrationV1beta1CollectionAPIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                DeleteApiregistrationV1beta1CollectionAPIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteApiregistrationV1beta1CollectionAPIServiceResponse::Unauthorized(response),
            other => DeleteApiregistrationV1beta1CollectionAPIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation listApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum ListApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// list or watch objects of kind APIService
    pub fn list_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<ListApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                ListApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => ListApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation patchApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum PatchApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// partially update the specified APIService
    pub fn patch_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => PatchApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation readApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum ReadApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// read the specified APIService
    pub fn read_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => ReadApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceApiregistrationV1beta1APIService

#[derive(Debug)]
pub enum ReplaceApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// replace the specified APIService
    pub fn replace_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        body: &::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceApiregistrationV1beta1APIServiceResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => ReplaceApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceApiregistrationV1beta1APIServiceStatus

#[derive(Debug)]
pub enum ReplaceApiregistrationV1beta1APIServiceStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Created(::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// replace status of the specified APIService
    pub fn replace_apiregistration_v1beta1_api_service_status<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        body: &::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIService,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceApiregistrationV1beta1APIServiceStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/apiservices/{name}/status", name = name)).map_err(::Error::URL)?;
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
                ReplaceApiregistrationV1beta1APIServiceStatusResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceApiregistrationV1beta1APIServiceStatusResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceApiregistrationV1beta1APIServiceStatusResponse::Unauthorized(response),
            other => ReplaceApiregistrationV1beta1APIServiceStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchApiregistrationV1beta1APIService

pub enum WatchApiregistrationV1beta1APIServiceResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// watch changes to an object of kind APIService
    pub fn watch_apiregistration_v1beta1_api_service<C>(
        __client: &C,
        // name of the APIService
        name: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchApiregistrationV1beta1APIServiceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/watch/apiservices/{name}", name = name)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                WatchApiregistrationV1beta1APIServiceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchApiregistrationV1beta1APIServiceResponse::Unauthorized(response),
            other => WatchApiregistrationV1beta1APIServiceResponse::Other(other, response),
        })
    }
}

// Generated from operation watchApiregistrationV1beta1APIServiceList

pub enum WatchApiregistrationV1beta1APIServiceListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl APIService {
    /// watch individual changes to a list of APIService
    pub fn watch_apiregistration_v1beta1_api_service_list<C>(
        __client: &C,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<WatchApiregistrationV1beta1APIServiceListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apiregistration.k8s.io/v1beta1/watch/apiservices")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(continue_) = continue_ {
                __query_pairs.append_pair("continue", &continue_);
            }
            if let Some(field_selector) = field_selector {
                __query_pairs.append_pair("fieldSelector", &field_selector);
            }
            if let Some(include_uninitialized) = include_uninitialized {
                __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
            }
            if let Some(label_selector) = label_selector {
                __query_pairs.append_pair("labelSelector", &label_selector);
            }
            if let Some(limit) = limit {
                __query_pairs.append_pair("limit", &limit.to_string());
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
                WatchApiregistrationV1beta1APIServiceListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchApiregistrationV1beta1APIServiceListResponse::Unauthorized(response),
            other => WatchApiregistrationV1beta1APIServiceListResponse::Other(other, response),
        })
    }
}

// End apiregistration.k8s.io/v1beta1/APIService

impl<'de> ::serde::Deserialize<'de> for APIService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIService;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIService")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceSpec> = None;
                let mut value_status: Option<::v1_9::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIService {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIService",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for APIService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIService",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
