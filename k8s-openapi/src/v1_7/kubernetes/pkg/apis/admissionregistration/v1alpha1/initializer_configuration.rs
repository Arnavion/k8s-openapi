// Generated from definition io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration

/// InitializerConfiguration describes the configuration of initializers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitializerConfiguration {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Initializers is a list of resources and their default initializers Order-sensitive. When merging multiple InitializerConfigurations, we sort the initializers from different InitializerConfigurations by the name of the InitializerConfigurations; the order of the initializers from the same InitializerConfiguration is preserved.
    pub initializers: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::Initializer>>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin admissionregistration.k8s.io/v1alpha1/InitializerConfiguration

// Generated from operation createAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// create an InitializerConfiguration
    pub fn create_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        body: &::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations")).map_err(::Error::URL)?;
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
                CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1CollectionInitializerConfiguration

#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// delete collection of InitializerConfiguration
    pub fn delete_admissionregistration_v1alpha1_collection_initializer_configuration<C>(
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
    ) -> Result<DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations")).map_err(::Error::URL)?;
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
                DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::Unauthorized(response),
            other => DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// delete an InitializerConfiguration
    pub fn delete_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        // name of the InitializerConfiguration
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation listAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum ListAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfigurationList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// list or watch objects of kind InitializerConfiguration
    pub fn list_admissionregistration_v1alpha1_initializer_configuration<C>(
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
    ) -> Result<ListAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations")).map_err(::Error::URL)?;
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
                ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation patchAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// partially update the specified InitializerConfiguration
    pub fn patch_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        // name of the InitializerConfiguration
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation readAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// read the specified InitializerConfiguration
    pub fn read_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        // name of the InitializerConfiguration
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAdmissionregistrationV1alpha1InitializerConfiguration

#[derive(Debug)]
pub enum ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// replace the specified InitializerConfiguration
    pub fn replace_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        // name of the InitializerConfiguration
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1InitializerConfiguration

pub enum WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// watch changes to an object of kind InitializerConfiguration
    pub fn watch_admissionregistration_v1alpha1_initializer_configuration<C>(
        __client: &C,
        // name of the InitializerConfiguration
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
    ) -> Result<WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations/{name}", name = name)).map_err(::Error::URL)?;
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
                WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized(response),
            other => WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other(other, response),
        })
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1InitializerConfigurationList

pub enum WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl InitializerConfiguration {
    /// watch individual changes to a list of InitializerConfiguration
    pub fn watch_admissionregistration_v1alpha1_initializer_configuration_list<C>(
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
    ) -> Result<WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations")).map_err(::Error::URL)?;
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
                WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Unauthorized(response),
            other => WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Other(other, response),
        })
    }
}

// End admissionregistration.k8s.io/v1alpha1/InitializerConfiguration

impl<'de> ::serde::Deserialize<'de> for InitializerConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_initializers,
            Key_kind,
            Key_metadata,
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
                            "initializers" => Field::Key_initializers,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InitializerConfiguration;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct InitializerConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_initializers: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::Initializer>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initializers => value_initializers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(InitializerConfiguration {
                    api_version: value_api_version,
                    initializers: value_initializers,
                    kind: value_kind,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "InitializerConfiguration",
            &[
                "apiVersion",
                "initializers",
                "kind",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for InitializerConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "InitializerConfiguration",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.initializers.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.initializers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "initializers", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
