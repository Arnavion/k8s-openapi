// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.ThirdPartyResource

/// A ThirdPartyResource is a generic representation of a resource, it is used by add-ons and plugins to add new resource types to the API.  It consists of one or more Versions of the api.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThirdPartyResource {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Description is the description of this object.
    pub description: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Versions are versions for this third party object
    pub versions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::APIVersion>>,
}

// Begin extensions/v1beta1/ThirdPartyResource

// Generated from operation createExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum CreateExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// create a ThirdPartyResource
    pub fn create_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources")).map_err(::Error::URL)?;
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
                CreateExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => CreateExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteExtensionsV1beta1CollectionThirdPartyResource

#[derive(Debug)]
pub enum DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// delete collection of ThirdPartyResource
    pub fn delete_extensions_v1beta1_collection_third_party_resource<C>(
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
    ) -> Result<DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources")).map_err(::Error::URL)?;
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
                DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::Unauthorized(response),
            other => DeleteExtensionsV1beta1CollectionThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum DeleteExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// delete a ThirdPartyResource
    pub fn delete_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        // name of the ThirdPartyResource
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => DeleteExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation listExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum ListExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResourceList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// list or watch objects of kind ThirdPartyResource
    pub fn list_extensions_v1beta1_third_party_resource<C>(
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
    ) -> Result<ListExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources")).map_err(::Error::URL)?;
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
                ListExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => ListExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation patchExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum PatchExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// partially update the specified ThirdPartyResource
    pub fn patch_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        // name of the ThirdPartyResource
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => PatchExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation readExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum ReadExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// read the specified ThirdPartyResource
    pub fn read_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        // name of the ThirdPartyResource
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => ReadExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceExtensionsV1beta1ThirdPartyResource

#[derive(Debug)]
pub enum ReplaceExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// replace the specified ThirdPartyResource
    pub fn replace_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        // name of the ThirdPartyResource
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::ThirdPartyResource,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/thirdpartyresources/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => ReplaceExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation watchExtensionsV1beta1ThirdPartyResource

pub enum WatchExtensionsV1beta1ThirdPartyResourceResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// watch changes to an object of kind ThirdPartyResource
    pub fn watch_extensions_v1beta1_third_party_resource<C>(
        __client: &C,
        // name of the ThirdPartyResource
        name: &str,
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
    ) -> Result<WatchExtensionsV1beta1ThirdPartyResourceResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/watch/thirdpartyresources/{name}", name = name)).map_err(::Error::URL)?;
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
                WatchExtensionsV1beta1ThirdPartyResourceResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchExtensionsV1beta1ThirdPartyResourceResponse::Unauthorized(response),
            other => WatchExtensionsV1beta1ThirdPartyResourceResponse::Other(other, response),
        })
    }
}

// Generated from operation watchExtensionsV1beta1ThirdPartyResourceList

pub enum WatchExtensionsV1beta1ThirdPartyResourceListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ThirdPartyResource {
    /// watch individual changes to a list of ThirdPartyResource
    pub fn watch_extensions_v1beta1_third_party_resource_list<C>(
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
    ) -> Result<WatchExtensionsV1beta1ThirdPartyResourceListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/watch/thirdpartyresources")).map_err(::Error::URL)?;
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
                WatchExtensionsV1beta1ThirdPartyResourceListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchExtensionsV1beta1ThirdPartyResourceListResponse::Unauthorized(response),
            other => WatchExtensionsV1beta1ThirdPartyResourceListResponse::Other(other, response),
        })
    }
}

// End extensions/v1beta1/ThirdPartyResource

impl<'de> ::serde::Deserialize<'de> for ThirdPartyResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_description,
            Key_kind,
            Key_metadata,
            Key_versions,
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
                            "description" => Field::Key_description,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThirdPartyResource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ThirdPartyResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_description: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_versions: Option<Vec<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::APIVersion>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_description => value_description = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ThirdPartyResource {
                    api_version: value_api_version,
                    description: value_description,
                    kind: value_kind,
                    metadata: value_metadata,
                    versions: value_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "ThirdPartyResource",
            &[
                "apiVersion",
                "description",
                "kind",
                "metadata",
                "versions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ThirdPartyResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ThirdPartyResource",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.description.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.description {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.versions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
