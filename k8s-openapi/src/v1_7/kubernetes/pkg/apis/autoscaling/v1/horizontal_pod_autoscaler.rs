// Generated from definition io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler

/// configuration of a horizontal pod autoscaler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscaler {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerSpec>,

    /// current information about the autoscaler.
    pub status: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerStatus>,
}

// Generated from operation createAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum CreateAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// create a HorizontalPodAutoscaler
    pub fn create_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", namespace = namespace)).map_err(::Error::URL)?;
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
                CreateAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => CreateAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation deleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum DeleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// delete collection of HorizontalPodAutoscaler
    pub fn delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler<C>(
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
    ) -> Result<DeleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => DeleteAutoscalingV1CollectionNamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation deleteAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum DeleteAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// delete a HorizontalPodAutoscaler
    pub fn delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
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
    ) -> Result<DeleteAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                DeleteAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => DeleteAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation listAutoscalingV1HorizontalPodAutoscalerForAllNamespaces

#[derive(Debug)]
pub enum ListAutoscalingV1HorizontalPodAutoscalerForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    pub fn list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces<C>(
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
    ) -> Result<ListAutoscalingV1HorizontalPodAutoscalerForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/horizontalpodautoscalers")).map_err(::Error::URL)?;
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
                ListAutoscalingV1HorizontalPodAutoscalerForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListAutoscalingV1HorizontalPodAutoscalerForAllNamespacesResponse::Unauthorized(response),
            other => ListAutoscalingV1HorizontalPodAutoscalerForAllNamespacesResponse::Other(other, response),
        })
    }

}

// Generated from operation listAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum ListAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// list or watch objects of kind HorizontalPodAutoscaler
    pub fn list_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
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
    ) -> Result<ListAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", namespace = namespace)).map_err(::Error::URL)?;
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
                ListAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => ListAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation patchAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum PatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// partially update the specified HorizontalPodAutoscaler
    pub fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => PatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation patchAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

#[derive(Debug)]
pub enum PatchAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// partially update status of the specified HorizontalPodAutoscaler
    pub fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized(response),
            other => PatchAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation readAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum ReadAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// read the specified HorizontalPodAutoscaler
    pub fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => ReadAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation readAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

#[derive(Debug)]
pub enum ReadAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// read status of the specified HorizontalPodAutoscaler
    pub fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized(response),
            other => ReadAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation replaceAutoscalingV1NamespacedHorizontalPodAutoscaler

#[derive(Debug)]
pub enum ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// replace the specified HorizontalPodAutoscaler
    pub fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation replaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatus

#[derive(Debug)]
pub enum ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// replace status of the specified HorizontalPodAutoscaler
    pub fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscaler,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Unauthorized(response),
            other => ReplaceAutoscalingV1NamespacedHorizontalPodAutoscalerStatusResponse::Other(other, response),
        })
    }

}

// Generated from operation watchAutoscalingV1HorizontalPodAutoscalerListForAllNamespaces

pub enum WatchAutoscalingV1HorizontalPodAutoscalerListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    pub fn watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces<C>(
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
    ) -> Result<WatchAutoscalingV1HorizontalPodAutoscalerListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/watch/horizontalpodautoscalers")).map_err(::Error::URL)?;
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
                WatchAutoscalingV1HorizontalPodAutoscalerListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAutoscalingV1HorizontalPodAutoscalerListForAllNamespacesResponse::Unauthorized(response),
            other => WatchAutoscalingV1HorizontalPodAutoscalerListForAllNamespacesResponse::Other(other, response),
        })
    }

}

// Generated from operation watchAutoscalingV1NamespacedHorizontalPodAutoscaler

pub enum WatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// watch changes to an object of kind HorizontalPodAutoscaler
    pub fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler<C>(
        __client: &C,
        // name of the HorizontalPodAutoscaler
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
    ) -> Result<WatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                WatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Unauthorized(response),
            other => WatchAutoscalingV1NamespacedHorizontalPodAutoscalerResponse::Other(other, response),
        })
    }

}

// Generated from operation watchAutoscalingV1NamespacedHorizontalPodAutoscalerList

pub enum WatchAutoscalingV1NamespacedHorizontalPodAutoscalerListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl HorizontalPodAutoscaler {
    /// watch individual changes to a list of HorizontalPodAutoscaler
    pub fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list<C>(
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
    ) -> Result<WatchAutoscalingV1NamespacedHorizontalPodAutoscalerListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers", namespace = namespace)).map_err(::Error::URL)?;
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
                WatchAutoscalingV1NamespacedHorizontalPodAutoscalerListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAutoscalingV1NamespacedHorizontalPodAutoscalerListResponse::Unauthorized(response),
            other => WatchAutoscalingV1NamespacedHorizontalPodAutoscalerListResponse::Other(other, response),
        })
    }

}

impl<'de> ::serde::Deserialize<'de> for HorizontalPodAutoscaler {
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
            type Value = HorizontalPodAutoscaler;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct HorizontalPodAutoscaler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v1::HorizontalPodAutoscalerStatus> = None;

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

                Ok(HorizontalPodAutoscaler {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscaler",
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

impl ::serde::Serialize for HorizontalPodAutoscaler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscaler",
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
