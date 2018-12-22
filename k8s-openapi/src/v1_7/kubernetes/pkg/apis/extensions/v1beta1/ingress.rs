// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.Ingress

/// Ingress is a collection of rules that allow inbound connections to reach the endpoints defined by a backend. An Ingress can be configured to give services externally-reachable urls, load balance traffic, terminate SSL, offer name based virtual hosting etc.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ingress {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec is the desired state of the Ingress. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressSpec>,

    /// Status is the current state of the Ingress. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressStatus>,
}

// Begin extensions/v1beta1/Ingress

// Generated from operation createExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// create an Ingress
    ///
    /// Use [`CreateExtensionsV1beta1NamespacedIngressResponse`](./enum.CreateExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
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
    pub fn create_extensions_v1beta1_namespaced_ingress(
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress,
        optional: CreateExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateExtensionsV1beta1NamespacedIngressOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::create_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.create_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct CreateExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::create_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.create_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum CreateExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for CreateExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateExtensionsV1beta1NamespacedIngressResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((CreateExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1CollectionNamespacedIngress

impl Ingress {
    /// delete collection of Ingress
    ///
    /// Use [`DeleteExtensionsV1beta1CollectionNamespacedIngressResponse`](./enum.DeleteExtensionsV1beta1CollectionNamespacedIngressResponse.html) to parse the HTTP response.
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
    pub fn delete_extensions_v1beta1_collection_namespaced_ingress(
        namespace: &str,
        optional: DeleteExtensionsV1beta1CollectionNamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteExtensionsV1beta1CollectionNamespacedIngressOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::delete_extensions_v1beta1_collection_namespaced_ingress`](./struct.Ingress.html#method.delete_extensions_v1beta1_collection_namespaced_ingress)
#[derive(Debug, Default)]
pub struct DeleteExtensionsV1beta1CollectionNamespacedIngressOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::delete_extensions_v1beta1_collection_namespaced_ingress`](./struct.Ingress.html#method.delete_extensions_v1beta1_collection_namespaced_ingress)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1CollectionNamespacedIngressResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteExtensionsV1beta1CollectionNamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1CollectionNamespacedIngressResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1CollectionNamespacedIngressResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1CollectionNamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1CollectionNamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// delete an Ingress
    ///
    /// Use [`DeleteExtensionsV1beta1NamespacedIngressResponse`](./enum.DeleteExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_extensions_v1beta1_namespaced_ingress(
        name: &str,
        namespace: &str,
        optional: DeleteExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteExtensionsV1beta1NamespacedIngressOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::delete_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.delete_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct DeleteExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::delete_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.delete_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum DeleteExtensionsV1beta1NamespacedIngressResponse {
    OkStatus(crate::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1NamespacedIngressResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteExtensionsV1beta1NamespacedIngressResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((DeleteExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation listExtensionsV1beta1IngressForAllNamespaces

impl Ingress {
    /// list or watch objects of kind Ingress
    ///
    /// Use [`ListExtensionsV1beta1IngressForAllNamespacesResponse`](./enum.ListExtensionsV1beta1IngressForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_extensions_v1beta1_ingress_for_all_namespaces(
        optional: ListExtensionsV1beta1IngressForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListExtensionsV1beta1IngressForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/ingresses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::list_extensions_v1beta1_ingress_for_all_namespaces`](./struct.Ingress.html#method.list_extensions_v1beta1_ingress_for_all_namespaces)
#[derive(Debug, Default)]
pub struct ListExtensionsV1beta1IngressForAllNamespacesOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::list_extensions_v1beta1_ingress_for_all_namespaces`](./struct.Ingress.html#method.list_extensions_v1beta1_ingress_for_all_namespaces)
#[derive(Debug)]
pub enum ListExtensionsV1beta1IngressForAllNamespacesResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressList),
    Unauthorized,
    Other,
}

impl crate::Response for ListExtensionsV1beta1IngressForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListExtensionsV1beta1IngressForAllNamespacesResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListExtensionsV1beta1IngressForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((ListExtensionsV1beta1IngressForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation listExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// list or watch objects of kind Ingress
    ///
    /// Use [`ListExtensionsV1beta1NamespacedIngressResponse`](./enum.ListExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
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
    pub fn list_extensions_v1beta1_namespaced_ingress(
        namespace: &str,
        optional: ListExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListExtensionsV1beta1NamespacedIngressOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::list_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.list_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct ListExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::list_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.list_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum ListExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressList),
    Unauthorized,
    Other,
}

impl crate::Response for ListExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListExtensionsV1beta1NamespacedIngressResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((ListExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// partially update the specified Ingress
    ///
    /// Use [`PatchExtensionsV1beta1NamespacedIngressResponse`](./enum.PatchExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
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
    pub fn patch_extensions_v1beta1_namespaced_ingress(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchExtensionsV1beta1NamespacedIngressOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::patch_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.patch_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct PatchExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::patch_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.patch_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for PatchExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1NamespacedIngressResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedIngressStatus

impl Ingress {
    /// partially update status of the specified Ingress
    ///
    /// Use [`PatchExtensionsV1beta1NamespacedIngressStatusResponse`](./enum.PatchExtensionsV1beta1NamespacedIngressStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
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
    pub fn patch_extensions_v1beta1_namespaced_ingress_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchExtensionsV1beta1NamespacedIngressStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchExtensionsV1beta1NamespacedIngressStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::patch_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.patch_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug, Default)]
pub struct PatchExtensionsV1beta1NamespacedIngressStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::patch_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.patch_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug)]
pub enum PatchExtensionsV1beta1NamespacedIngressStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for PatchExtensionsV1beta1NamespacedIngressStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchExtensionsV1beta1NamespacedIngressStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchExtensionsV1beta1NamespacedIngressStatusResponse::Unauthorized, 0)),
            _ => Ok((PatchExtensionsV1beta1NamespacedIngressStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// read the specified Ingress
    ///
    /// Use [`ReadExtensionsV1beta1NamespacedIngressResponse`](./enum.ReadExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_extensions_v1beta1_namespaced_ingress(
        name: &str,
        namespace: &str,
        optional: ReadExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadExtensionsV1beta1NamespacedIngressOptional {
            exact,
            export,
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
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

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::read_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.read_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct ReadExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    pub exact: Option<bool>,
    /// Should this value be exported.  Export strips fields that a user can not specify.
    pub export: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::read_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.read_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for ReadExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1NamespacedIngressResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation readExtensionsV1beta1NamespacedIngressStatus

impl Ingress {
    /// read status of the specified Ingress
    ///
    /// Use [`ReadExtensionsV1beta1NamespacedIngressStatusResponse`](./enum.ReadExtensionsV1beta1NamespacedIngressStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_extensions_v1beta1_namespaced_ingress_status(
        name: &str,
        namespace: &str,
        optional: ReadExtensionsV1beta1NamespacedIngressStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadExtensionsV1beta1NamespacedIngressStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::read_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.read_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug, Default)]
pub struct ReadExtensionsV1beta1NamespacedIngressStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::read_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.read_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug)]
pub enum ReadExtensionsV1beta1NamespacedIngressStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for ReadExtensionsV1beta1NamespacedIngressStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadExtensionsV1beta1NamespacedIngressStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadExtensionsV1beta1NamespacedIngressStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadExtensionsV1beta1NamespacedIngressStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// replace the specified Ingress
    ///
    /// Use [`ReplaceExtensionsV1beta1NamespacedIngressResponse`](./enum.ReplaceExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
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
    pub fn replace_extensions_v1beta1_namespaced_ingress(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress,
        optional: ReplaceExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceExtensionsV1beta1NamespacedIngressOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::replace_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.replace_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct ReplaceExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::replace_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.replace_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedIngressResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedIngressStatus

impl Ingress {
    /// replace status of the specified Ingress
    ///
    /// Use [`ReplaceExtensionsV1beta1NamespacedIngressStatusResponse`](./enum.ReplaceExtensionsV1beta1NamespacedIngressStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
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
    pub fn replace_extensions_v1beta1_namespaced_ingress_status(
        name: &str,
        namespace: &str,
        body: &crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress,
        optional: ReplaceExtensionsV1beta1NamespacedIngressStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceExtensionsV1beta1NamespacedIngressStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/ingresses/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::replace_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.replace_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug, Default)]
pub struct ReplaceExtensionsV1beta1NamespacedIngressStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`Ingress::replace_extensions_v1beta1_namespaced_ingress_status`](./struct.Ingress.html#method.replace_extensions_v1beta1_namespaced_ingress_status)
#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1NamespacedIngressStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::Ingress),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceExtensionsV1beta1NamespacedIngressStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceExtensionsV1beta1NamespacedIngressStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceExtensionsV1beta1NamespacedIngressStatusResponse::Unauthorized, 0)),
            _ => Ok((ReplaceExtensionsV1beta1NamespacedIngressStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1IngressListForAllNamespaces

impl Ingress {
    /// watch individual changes to a list of Ingress
    ///
    /// Use [`WatchExtensionsV1beta1IngressListForAllNamespacesResponse`](./enum.WatchExtensionsV1beta1IngressListForAllNamespacesResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_extensions_v1beta1_ingress_list_for_all_namespaces(
        optional: WatchExtensionsV1beta1IngressListForAllNamespacesOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchExtensionsV1beta1IngressListForAllNamespacesOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/watch/ingresses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::watch_extensions_v1beta1_ingress_list_for_all_namespaces`](./struct.Ingress.html#method.watch_extensions_v1beta1_ingress_list_for_all_namespaces)
#[derive(Debug, Default)]
pub struct WatchExtensionsV1beta1IngressListForAllNamespacesOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::watch_extensions_v1beta1_ingress_list_for_all_namespaces`](./struct.Ingress.html#method.watch_extensions_v1beta1_ingress_list_for_all_namespaces)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1IngressListForAllNamespacesResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchExtensionsV1beta1IngressListForAllNamespacesResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchExtensionsV1beta1IngressListForAllNamespacesResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1IngressListForAllNamespacesResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1IngressListForAllNamespacesResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1NamespacedIngress

impl Ingress {
    /// watch changes to an object of kind Ingress
    ///
    /// Use [`WatchExtensionsV1beta1NamespacedIngressResponse`](./enum.WatchExtensionsV1beta1NamespacedIngressResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Ingress
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_extensions_v1beta1_namespaced_ingress(
        name: &str,
        namespace: &str,
        optional: WatchExtensionsV1beta1NamespacedIngressOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchExtensionsV1beta1NamespacedIngressOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/watch/namespaces/{namespace}/ingresses/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::watch_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.watch_extensions_v1beta1_namespaced_ingress)
#[derive(Debug, Default)]
pub struct WatchExtensionsV1beta1NamespacedIngressOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::watch_extensions_v1beta1_namespaced_ingress`](./struct.Ingress.html#method.watch_extensions_v1beta1_namespaced_ingress)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1NamespacedIngressResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchExtensionsV1beta1NamespacedIngressResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchExtensionsV1beta1NamespacedIngressResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1NamespacedIngressResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1NamespacedIngressResponse::Other, 0)),
        }
    }
}

// Generated from operation watchExtensionsV1beta1NamespacedIngressList

impl Ingress {
    /// watch individual changes to a list of Ingress
    ///
    /// Use [`WatchExtensionsV1beta1NamespacedIngressListResponse`](./enum.WatchExtensionsV1beta1NamespacedIngressListResponse.html) to parse the HTTP response.
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
    pub fn watch_extensions_v1beta1_namespaced_ingress_list(
        namespace: &str,
        optional: WatchExtensionsV1beta1NamespacedIngressListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchExtensionsV1beta1NamespacedIngressListOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/extensions/v1beta1/watch/namespaces/{namespace}/ingresses?", namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Ingress::watch_extensions_v1beta1_namespaced_ingress_list`](./struct.Ingress.html#method.watch_extensions_v1beta1_namespaced_ingress_list)
#[derive(Debug, Default)]
pub struct WatchExtensionsV1beta1NamespacedIngressListOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`Ingress::watch_extensions_v1beta1_namespaced_ingress_list`](./struct.Ingress.html#method.watch_extensions_v1beta1_namespaced_ingress_list)
#[derive(Debug)]
pub enum WatchExtensionsV1beta1NamespacedIngressListResponse {
    Ok(crate::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchExtensionsV1beta1NamespacedIngressListResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchExtensionsV1beta1NamespacedIngressListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchExtensionsV1beta1NamespacedIngressListResponse::Unauthorized, 0)),
            _ => Ok((WatchExtensionsV1beta1NamespacedIngressListResponse::Other, 0)),
        }
    }
}

// End extensions/v1beta1/Ingress

impl crate::Resource for Ingress {
    fn api_version() -> &'static str {
        "extensions/v1beta1"
    }

    fn group() -> &'static str {
        "extensions"
    }

    fn kind() -> &'static str {
        "Ingress"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for Ingress {
    type Ty = crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Ingress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Ingress;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Ingress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressSpec> = None;
                let mut value_status: Option<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::IngressStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Ingress {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Ingress",
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

impl serde::Serialize for Ingress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Ingress",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
