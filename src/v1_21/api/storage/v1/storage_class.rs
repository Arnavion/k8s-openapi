// Generated from definition io.k8s.api.storage.v1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    pub allow_volume_expansion: Option<bool>,

    /// Restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
    pub allowed_topologies: Option<Vec<crate::api::core::v1::TopologySelectorTerm>>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    pub mount_options: Option<Vec<String>>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    pub reclaim_policy: Option<String>,

    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
    pub volume_binding_mode: Option<String>,

}

#[cfg(feature = "dsl")]
impl StorageClass  {
    /// Set [`Self::allow_volume_expansion`]
    pub  fn allow_volume_expansion_set(&mut self, allow_volume_expansion: impl Into<Option<bool>>) -> &mut Self {
        self.allow_volume_expansion = allow_volume_expansion.into(); self
    }

    pub  fn allow_volume_expansion(&mut self) -> &mut bool {
        if self.allow_volume_expansion.is_none() { self.allow_volume_expansion = Some(Default::default()) }
        self.allow_volume_expansion.as_mut().unwrap()
    }

    /// Modify [`Self::allow_volume_expansion`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allow_volume_expansion_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.allow_volume_expansion.is_none() { self.allow_volume_expansion = Some(Default::default()) };
        func(self.allow_volume_expansion.as_mut().unwrap()); self
    }


    /// Set [`Self::allowed_topologies`]
    pub  fn allowed_topologies_set(&mut self, allowed_topologies: impl Into<Option<Vec<crate::api::core::v1::TopologySelectorTerm>>>) -> &mut Self {
        self.allowed_topologies = allowed_topologies.into(); self
    }

    pub  fn allowed_topologies(&mut self) -> &mut Vec<crate::api::core::v1::TopologySelectorTerm> {
        if self.allowed_topologies.is_none() { self.allowed_topologies = Some(Default::default()) }
        self.allowed_topologies.as_mut().unwrap()
    }

    /// Modify [`Self::allowed_topologies`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allowed_topologies_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::TopologySelectorTerm>)) -> &mut Self {
        if self.allowed_topologies.is_none() { self.allowed_topologies = Some(Default::default()) };
        func(self.allowed_topologies.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::allowed_topologies`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn allowed_topologies_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::TopologySelectorTerm)) -> &mut Self {
        if self.allowed_topologies.is_none() {
            self.allowed_topologies = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.allowed_topologies.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::allowed_topologies`]
    pub  fn allowed_topologies_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::TopologySelectorTerm]>) -> &mut Self {
         if self.allowed_topologies.is_none() { self.allowed_topologies = Some(Vec::new()); }
         let allowed_topologies = &mut self.allowed_topologies.as_mut().unwrap();
         for item in other.borrow() {
             allowed_topologies.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::metadata`]
    pub  fn metadata_set(&mut self, metadata: impl Into<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>) -> &mut Self {
        self.metadata = metadata.into(); self
    }

    pub  fn metadata(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        &mut self.metadata
    }

    /// Modify [`Self::metadata`] with a `func`
    pub  fn metadata_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::ObjectMeta)) -> &mut Self {
        func(&mut self.metadata); self
    }


    /// Set [`Self::mount_options`]
    pub  fn mount_options_set(&mut self, mount_options: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.mount_options = mount_options.into(); self
    }

    pub  fn mount_options(&mut self) -> &mut Vec<String> {
        if self.mount_options.is_none() { self.mount_options = Some(Default::default()) }
        self.mount_options.as_mut().unwrap()
    }

    /// Modify [`Self::mount_options`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn mount_options_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.mount_options.is_none() { self.mount_options = Some(Default::default()) };
        func(self.mount_options.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::mount_options`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn mount_options_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.mount_options.is_none() {
            self.mount_options = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.mount_options.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::mount_options`]
    pub  fn mount_options_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.mount_options.is_none() { self.mount_options = Some(Vec::new()); }
         let mount_options = &mut self.mount_options.as_mut().unwrap();
         for item in other.borrow() {
             mount_options.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::parameters`]
    pub  fn parameters_set(&mut self, parameters: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.parameters = parameters.into(); self
    }

    pub  fn parameters(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.parameters.is_none() { self.parameters = Some(Default::default()) }
        self.parameters.as_mut().unwrap()
    }

    /// Modify [`Self::parameters`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn parameters_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.parameters.is_none() { self.parameters = Some(Default::default()) };
        func(self.parameters.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::parameters`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn parameters_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.parameters.is_none() {
            self.parameters = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.parameters.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::parameters`]
    pub  fn parameters_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.parameters.is_none() { self.parameters = Some(std::collections::BTreeMap::new()); }
         let parameters = &mut self.parameters.as_mut().unwrap();
         for (name, value) in other.borrow() {
             parameters.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::provisioner`]
    pub  fn provisioner_set(&mut self, provisioner: impl Into<String>) -> &mut Self {
        self.provisioner = provisioner.into(); self
    }

    pub  fn provisioner(&mut self) -> &mut String {
        &mut self.provisioner
    }

    /// Modify [`Self::provisioner`] with a `func`
    pub  fn provisioner_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.provisioner); self
    }


    /// Set [`Self::reclaim_policy`]
    pub  fn reclaim_policy_set(&mut self, reclaim_policy: impl Into<Option<String>>) -> &mut Self {
        self.reclaim_policy = reclaim_policy.into(); self
    }

    pub  fn reclaim_policy(&mut self) -> &mut String {
        if self.reclaim_policy.is_none() { self.reclaim_policy = Some(Default::default()) }
        self.reclaim_policy.as_mut().unwrap()
    }

    /// Modify [`Self::reclaim_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn reclaim_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.reclaim_policy.is_none() { self.reclaim_policy = Some(Default::default()) };
        func(self.reclaim_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_binding_mode`]
    pub  fn volume_binding_mode_set(&mut self, volume_binding_mode: impl Into<Option<String>>) -> &mut Self {
        self.volume_binding_mode = volume_binding_mode.into(); self
    }

    pub  fn volume_binding_mode(&mut self) -> &mut String {
        if self.volume_binding_mode.is_none() { self.volume_binding_mode = Some(Default::default()) }
        self.volume_binding_mode.as_mut().unwrap()
    }

    /// Modify [`Self::volume_binding_mode`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_binding_mode_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volume_binding_mode.is_none() { self.volume_binding_mode = Some(Default::default()) };
        func(self.volume_binding_mode.as_mut().unwrap()); self
    }


}


// Begin storage.k8s.io/v1/StorageClass

// Generated from operation createStorageV1StorageClass

impl StorageClass {
    /// create a StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::CreateResponse`]`<Self>>` constructor, or [`crate::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create(
        body: &crate::api::storage::v1::StorageClass,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = "/apis/storage.k8s.io/v1/storageclasses?".to_owned();
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

// Generated from operation deleteStorageV1CollectionStorageClass

impl StorageClass {
    /// delete collection of StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>>` constructor, or [`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection(
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<crate::List<Self>>>), crate::RequestError> {
        let __url = "/apis/storage.k8s.io/v1/storageclasses?".to_owned();
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

// Generated from operation deleteStorageV1StorageClass

impl StorageClass {
    /// delete a StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<Self>>` constructor, or [`crate::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete(
        name: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation listStorageV1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
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
    pub fn list(
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = "/apis/storage.k8s.io/v1/storageclasses?".to_owned();
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

// Generated from operation patchStorageV1StorageClass

impl StorageClass {
    /// partially update the specified StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch(
        name: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation readStorageV1StorageClass

impl StorageClass {
    /// read the specified StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadStorageClassResponse`]`>` constructor, or [`ReadStorageClassResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    #[cfg(feature = "api")]
    pub fn read(
        name: &str,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadStorageClassResponse>), crate::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ReadStorageClassResponse as Response>::try_from_parts` to parse the HTTP response body of [`StorageClass::read`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadStorageClassResponse {
    Ok(crate::api::storage::v1::StorageClass),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadStorageClassResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadStorageClassResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadStorageClassResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceStorageV1StorageClass

impl StorageClass {
    /// replace the specified StorageClass
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the StorageClass
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace(
        name: &str,
        body: &crate::api::storage::v1::StorageClass,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1/storageclasses/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
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

// Generated from operation watchStorageV1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
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
    pub fn watch(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = "/apis/storage.k8s.io/v1/storageclasses?".to_owned();
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

// End storage.k8s.io/v1/StorageClass

impl crate::Resource for StorageClass {
    const API_VERSION: &'static str = "storage.k8s.io/v1";
    const GROUP: &'static str = "storage.k8s.io";
    const KIND: &'static str = "StorageClass";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "storageclasses";
    type Scope = crate::ClusterResourceScope;
}

impl crate::ListableResource for StorageClass {
    const LIST_KIND: &'static str = "StorageClassList";
}

impl crate::Metadata for StorageClass {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_allow_volume_expansion,
            Key_allowed_topologies,
            Key_metadata,
            Key_mount_options,
            Key_parameters,
            Key_provisioner,
            Key_reclaim_policy,
            Key_volume_binding_mode,
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
                            "allowVolumeExpansion" => Field::Key_allow_volume_expansion,
                            "allowedTopologies" => Field::Key_allowed_topologies,
                            "metadata" => Field::Key_metadata,
                            "mountOptions" => Field::Key_mount_options,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
                            "reclaimPolicy" => Field::Key_reclaim_policy,
                            "volumeBindingMode" => Field::Key_volume_binding_mode,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageClass;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allow_volume_expansion: Option<bool> = None;
                let mut value_allowed_topologies: Option<Vec<crate::api::core::v1::TopologySelectorTerm>> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_parameters: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;
                let mut value_reclaim_policy: Option<String> = None;
                let mut value_volume_binding_mode: Option<String> = None;

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
                        Field::Key_allow_volume_expansion => value_allow_volume_expansion = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_topologies => value_allowed_topologies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reclaim_policy => value_reclaim_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_binding_mode => value_volume_binding_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    allow_volume_expansion: value_allow_volume_expansion,
                    allowed_topologies: value_allowed_topologies,
                    metadata: value_metadata.unwrap_or_default(),
                    mount_options: value_mount_options,
                    parameters: value_parameters,
                    provisioner: value_provisioner.unwrap_or_default(),
                    reclaim_policy: value_reclaim_policy,
                    volume_binding_mode: value_volume_binding_mode,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "allowVolumeExpansion",
                "allowedTopologies",
                "metadata",
                "mountOptions",
                "parameters",
                "provisioner",
                "reclaimPolicy",
                "volumeBindingMode",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.allow_volume_expansion.as_ref().map_or(0, |_| 1) +
            self.allowed_topologies.as_ref().map_or(0, |_| 1) +
            self.mount_options.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            self.reclaim_policy.as_ref().map_or(0, |_| 1) +
            self.volume_binding_mode.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.allow_volume_expansion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowVolumeExpansion", value)?;
        }
        if let Some(value) = &self.allowed_topologies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedTopologies", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.mount_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        if let Some(value) = &self.reclaim_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reclaimPolicy", value)?;
        }
        if let Some(value) = &self.volume_binding_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeBindingMode", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageClass {
    fn schema_name() -> String {
        "io.k8s.api.storage.v1.StorageClass".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.\n\nStorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowVolumeExpansion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllowVolumeExpansion shows whether the storage class allow volume expand".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allowedTopologies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::TopologySelectorTerm>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "mountOptions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. [\"ro\", \"soft\"]. Not validated - mount of the PVs will simply fail if one is invalid.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parameters".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Parameters holds the parameters for the provisioner that should create volumes of this storage class.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "provisioner".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Provisioner indicates the type of the provisioner.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reclaimPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeBindingMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "metadata".to_owned(),
                    "provisioner".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
