// Generated from definition io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet

/// StatefulSet represents a set of pods with consistent identities. Identities are defined as:
///  - Network: A single stable DNS and hostname.
///  - Storage: As many VolumeClaims as requested.
/// The StatefulSet guarantees that a given network identity will always map to the same storage identity.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSet {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired identities of pods in this set.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetSpec>,

    /// Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
    pub status: Option<::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetStatus>,
}

// Begin apps/v1beta1/StatefulSet

// Generated from operation createAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum CreateAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// create a StatefulSet
    pub fn create_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets", namespace = namespace)).map_err(::Error::URL)?;
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
                CreateAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => CreateAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteAppsV1beta1CollectionNamespacedStatefulSet

#[derive(Debug)]
pub enum DeleteAppsV1beta1CollectionNamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// delete collection of StatefulSet
    pub fn delete_apps_v1beta1_collection_namespaced_stateful_set<C>(
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
    ) -> Result<DeleteAppsV1beta1CollectionNamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets", namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteAppsV1beta1CollectionNamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAppsV1beta1CollectionNamespacedStatefulSetResponse::Unauthorized(response),
            other => DeleteAppsV1beta1CollectionNamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum DeleteAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// delete a StatefulSet
    pub fn delete_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // name of the StatefulSet
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
    ) -> Result<DeleteAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => DeleteAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation listAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum ListAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    pub fn list_apps_v1beta1_namespaced_stateful_set<C>(
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
    ) -> Result<ListAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets", namespace = namespace)).map_err(::Error::URL)?;
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
                ListAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => ListAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation listAppsV1beta1StatefulSetForAllNamespaces

#[derive(Debug)]
pub enum ListAppsV1beta1StatefulSetForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// list or watch objects of kind StatefulSet
    pub fn list_apps_v1beta1_stateful_set_for_all_namespaces<C>(
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
    ) -> Result<ListAppsV1beta1StatefulSetForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/statefulsets")).map_err(::Error::URL)?;
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
                ListAppsV1beta1StatefulSetForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListAppsV1beta1StatefulSetForAllNamespacesResponse::Unauthorized(response),
            other => ListAppsV1beta1StatefulSetForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation patchAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum PatchAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// partially update the specified StatefulSet
    pub fn patch_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => PatchAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation patchAppsV1beta1NamespacedStatefulSetStatus

#[derive(Debug)]
pub enum PatchAppsV1beta1NamespacedStatefulSetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// partially update status of the specified StatefulSet
    pub fn patch_apps_v1beta1_namespaced_stateful_set_status<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAppsV1beta1NamespacedStatefulSetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAppsV1beta1NamespacedStatefulSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAppsV1beta1NamespacedStatefulSetStatusResponse::Unauthorized(response),
            other => PatchAppsV1beta1NamespacedStatefulSetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation readAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum ReadAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// read the specified StatefulSet
    pub fn read_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => ReadAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation readAppsV1beta1NamespacedStatefulSetStatus

#[derive(Debug)]
pub enum ReadAppsV1beta1NamespacedStatefulSetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// read status of the specified StatefulSet
    pub fn read_apps_v1beta1_namespaced_stateful_set_status<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAppsV1beta1NamespacedStatefulSetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadAppsV1beta1NamespacedStatefulSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAppsV1beta1NamespacedStatefulSetStatusResponse::Unauthorized(response),
            other => ReadAppsV1beta1NamespacedStatefulSetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAppsV1beta1NamespacedStatefulSet

#[derive(Debug)]
pub enum ReplaceAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// replace the specified StatefulSet
    pub fn replace_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => ReplaceAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAppsV1beta1NamespacedStatefulSetStatus

#[derive(Debug)]
pub enum ReplaceAppsV1beta1NamespacedStatefulSetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// replace status of the specified StatefulSet
    pub fn replace_apps_v1beta1_namespaced_stateful_set_status<C>(
        __client: &C,
        // name of the StatefulSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAppsV1beta1NamespacedStatefulSetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAppsV1beta1NamespacedStatefulSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAppsV1beta1NamespacedStatefulSetStatusResponse::Unauthorized(response),
            other => ReplaceAppsV1beta1NamespacedStatefulSetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchAppsV1beta1NamespacedStatefulSet

pub enum WatchAppsV1beta1NamespacedStatefulSetResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// watch changes to an object of kind StatefulSet
    pub fn watch_apps_v1beta1_namespaced_stateful_set<C>(
        __client: &C,
        // name of the StatefulSet
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
    ) -> Result<WatchAppsV1beta1NamespacedStatefulSetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                WatchAppsV1beta1NamespacedStatefulSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAppsV1beta1NamespacedStatefulSetResponse::Unauthorized(response),
            other => WatchAppsV1beta1NamespacedStatefulSetResponse::Other(other, response),
        })
    }
}

// Generated from operation watchAppsV1beta1NamespacedStatefulSetList

pub enum WatchAppsV1beta1NamespacedStatefulSetListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// watch individual changes to a list of StatefulSet
    pub fn watch_apps_v1beta1_namespaced_stateful_set_list<C>(
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
    ) -> Result<WatchAppsV1beta1NamespacedStatefulSetListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets", namespace = namespace)).map_err(::Error::URL)?;
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
                WatchAppsV1beta1NamespacedStatefulSetListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAppsV1beta1NamespacedStatefulSetListResponse::Unauthorized(response),
            other => WatchAppsV1beta1NamespacedStatefulSetListResponse::Other(other, response),
        })
    }
}

// Generated from operation watchAppsV1beta1StatefulSetListForAllNamespaces

pub enum WatchAppsV1beta1StatefulSetListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl StatefulSet {
    /// watch individual changes to a list of StatefulSet
    pub fn watch_apps_v1beta1_stateful_set_list_for_all_namespaces<C>(
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
    ) -> Result<WatchAppsV1beta1StatefulSetListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta1/watch/statefulsets")).map_err(::Error::URL)?;
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
                WatchAppsV1beta1StatefulSetListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAppsV1beta1StatefulSetListForAllNamespacesResponse::Unauthorized(response),
            other => WatchAppsV1beta1StatefulSetListForAllNamespacesResponse::Other(other, response),
        })
    }
}

// End apps/v1beta1/StatefulSet

impl<'de> ::serde::Deserialize<'de> for StatefulSet {
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
            type Value = StatefulSet;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct StatefulSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::apps::v1beta1::StatefulSetStatus> = None;

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

                Ok(StatefulSet {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSet",
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

impl ::serde::Serialize for StatefulSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSet",
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
