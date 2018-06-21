// Generated from definition io.k8s.kubernetes.pkg.api.v1.PersistentVolume

/// PersistentVolume (PV) is a storage resource provisioned by an administrator. It is analogous to a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolume {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines a specification of a persistent volume owned by the cluster. Provisioned by an administrator. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::PersistentVolumeSpec>,

    /// Status represents the current information/status for the persistent volume. Populated by the system. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes
    pub status: Option<::v1_7::kubernetes::pkg::api::v1::PersistentVolumeStatus>,
}

// Begin /v1/PersistentVolume

// Generated from operation createCoreV1PersistentVolume

#[derive(Debug)]
pub enum CreateCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// create a PersistentVolume
    pub fn create_core_v1_persistent_volume<C>(
        __client: &C,
        body: &::v1_7::kubernetes::pkg::api::v1::PersistentVolume,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes")).map_err(::Error::URL)?;
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
                CreateCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => CreateCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1CollectionPersistentVolume

#[derive(Debug)]
pub enum DeleteCoreV1CollectionPersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// delete collection of PersistentVolume
    pub fn delete_core_v1_collection_persistent_volume<C>(
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
    ) -> Result<DeleteCoreV1CollectionPersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes")).map_err(::Error::URL)?;
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
                DeleteCoreV1CollectionPersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1CollectionPersistentVolumeResponse::Unauthorized(response),
            other => DeleteCoreV1CollectionPersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteCoreV1PersistentVolume

#[derive(Debug)]
pub enum DeleteCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// delete a PersistentVolume
    pub fn delete_core_v1_persistent_volume<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => DeleteCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation listCoreV1PersistentVolume

#[derive(Debug)]
pub enum ListCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolumeList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// list or watch objects of kind PersistentVolume
    pub fn list_core_v1_persistent_volume<C>(
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
    ) -> Result<ListCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes")).map_err(::Error::URL)?;
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
                ListCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => ListCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1PersistentVolume

#[derive(Debug)]
pub enum PatchCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// partially update the specified PersistentVolume
    pub fn patch_core_v1_persistent_volume<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => PatchCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation patchCoreV1PersistentVolumeStatus

#[derive(Debug)]
pub enum PatchCoreV1PersistentVolumeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// partially update status of the specified PersistentVolume
    pub fn patch_core_v1_persistent_volume_status<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchCoreV1PersistentVolumeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                PatchCoreV1PersistentVolumeStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1PersistentVolumeStatusResponse::Unauthorized(response),
            other => PatchCoreV1PersistentVolumeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1PersistentVolume

#[derive(Debug)]
pub enum ReadCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// read the specified PersistentVolume
    pub fn read_core_v1_persistent_volume<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => ReadCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation readCoreV1PersistentVolumeStatus

#[derive(Debug)]
pub enum ReadCoreV1PersistentVolumeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// read status of the specified PersistentVolume
    pub fn read_core_v1_persistent_volume_status<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadCoreV1PersistentVolumeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                ReadCoreV1PersistentVolumeStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1PersistentVolumeStatusResponse::Unauthorized(response),
            other => ReadCoreV1PersistentVolumeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1PersistentVolume

#[derive(Debug)]
pub enum ReplaceCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// replace the specified PersistentVolume
    pub fn replace_core_v1_persistent_volume<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::PersistentVolume,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => ReplaceCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceCoreV1PersistentVolumeStatus

#[derive(Debug)]
pub enum ReplaceCoreV1PersistentVolumeStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::api::v1::PersistentVolume),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// replace status of the specified PersistentVolume
    pub fn replace_core_v1_persistent_volume_status<C>(
        __client: &C,
        // name of the PersistentVolume
        name: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::PersistentVolume,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceCoreV1PersistentVolumeStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/persistentvolumes/{name}/status", name = name)).map_err(::Error::URL)?;
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
                ReplaceCoreV1PersistentVolumeStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1PersistentVolumeStatusResponse::Unauthorized(response),
            other => ReplaceCoreV1PersistentVolumeStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1PersistentVolume

pub enum WatchCoreV1PersistentVolumeResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// watch changes to an object of kind PersistentVolume
    pub fn watch_core_v1_persistent_volume<C>(
        __client: &C,
        // name of the PersistentVolume
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
    ) -> Result<WatchCoreV1PersistentVolumeResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/persistentvolumes/{name}", name = name)).map_err(::Error::URL)?;
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
                WatchCoreV1PersistentVolumeResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1PersistentVolumeResponse::Unauthorized(response),
            other => WatchCoreV1PersistentVolumeResponse::Other(other, response),
        })
    }
}

// Generated from operation watchCoreV1PersistentVolumeList

pub enum WatchCoreV1PersistentVolumeListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PersistentVolume {
    /// watch individual changes to a list of PersistentVolume
    pub fn watch_core_v1_persistent_volume_list<C>(
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
    ) -> Result<WatchCoreV1PersistentVolumeListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/api/v1/watch/persistentvolumes")).map_err(::Error::URL)?;
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
                WatchCoreV1PersistentVolumeListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1PersistentVolumeListResponse::Unauthorized(response),
            other => WatchCoreV1PersistentVolumeListResponse::Other(other, response),
        })
    }
}

// End /v1/PersistentVolume

impl<'de> ::serde::Deserialize<'de> for PersistentVolume {
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
            type Value = PersistentVolume;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PersistentVolume")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::api::v1::PersistentVolumeSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::api::v1::PersistentVolumeStatus> = None;

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

                Ok(PersistentVolume {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolume",
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

impl ::serde::Serialize for PersistentVolume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolume",
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
