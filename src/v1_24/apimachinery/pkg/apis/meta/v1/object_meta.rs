// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMeta {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    pub annotations: Option<std::collections::BTreeMap<String, String>>,

    /// Deprecated: ClusterName is a legacy field that was always cleared by the system and never used; it will be removed completely in 1.25.
    ///
    /// The name in the go struct is changed to help clients detect accidental use.
    pub cluster_name: Option<String>,

    /// CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
    ///
    /// Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub creation_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
    pub deletion_grace_period_seconds: Option<i64>,

    /// DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub deletion_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.
    pub finalizers: Option<Vec<String>>,

    /// GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
    ///
    /// If this field is specified and the generated name exists, the server will return a 409.
    ///
    /// Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
    pub generate_name: Option<String>,

    /// A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.
    pub generation: Option<i64>,

    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    pub labels: Option<std::collections::BTreeMap<String, String>>,

    /// ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.
    pub managed_fields: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>>,

    /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: Option<String>,

    /// Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
    pub namespace: Option<String>,

    /// List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
    pub owner_references: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>>,

    /// An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.
    ///
    /// Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.
    pub self_link: Option<String>,

    /// UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.
    ///
    /// Populated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: Option<String>,

}

#[cfg(feature = "dsl")]
impl ObjectMeta  {
    /// Set [`Self::annotations`]
    pub  fn annotations_set(&mut self, annotations: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.annotations = annotations.into(); self
    }

    pub  fn annotations(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.annotations.is_none() { self.annotations = Some(Default::default()) }
        self.annotations.as_mut().unwrap()
    }

    /// Modify [`Self::annotations`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn annotations_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.annotations.is_none() { self.annotations = Some(Default::default()) };
        func(self.annotations.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::annotations`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn annotations_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.annotations.is_none() {
            self.annotations = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.annotations.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::annotations`]
    pub  fn annotations_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.annotations.is_none() { self.annotations = Some(std::collections::BTreeMap::new()); }
         let annotations = &mut self.annotations.as_mut().unwrap();
         for (name, value) in other.borrow() {
             annotations.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::cluster_name`]
    pub  fn cluster_name_set(&mut self, cluster_name: impl Into<Option<String>>) -> &mut Self {
        self.cluster_name = cluster_name.into(); self
    }

    pub  fn cluster_name(&mut self) -> &mut String {
        if self.cluster_name.is_none() { self.cluster_name = Some(Default::default()) }
        self.cluster_name.as_mut().unwrap()
    }

    /// Modify [`Self::cluster_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cluster_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.cluster_name.is_none() { self.cluster_name = Some(Default::default()) };
        func(self.cluster_name.as_mut().unwrap()); self
    }


    /// Set [`Self::creation_timestamp`]
    pub  fn creation_timestamp_set(&mut self, creation_timestamp: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.creation_timestamp = creation_timestamp.into(); self
    }


    /// Set [`Self::deletion_grace_period_seconds`]
    pub  fn deletion_grace_period_seconds_set(&mut self, deletion_grace_period_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.deletion_grace_period_seconds = deletion_grace_period_seconds.into(); self
    }

    pub  fn deletion_grace_period_seconds(&mut self) -> &mut i64 {
        if self.deletion_grace_period_seconds.is_none() { self.deletion_grace_period_seconds = Some(Default::default()) }
        self.deletion_grace_period_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::deletion_grace_period_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn deletion_grace_period_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.deletion_grace_period_seconds.is_none() { self.deletion_grace_period_seconds = Some(Default::default()) };
        func(self.deletion_grace_period_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::deletion_timestamp`]
    pub  fn deletion_timestamp_set(&mut self, deletion_timestamp: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.deletion_timestamp = deletion_timestamp.into(); self
    }


    /// Set [`Self::finalizers`]
    pub  fn finalizers_set(&mut self, finalizers: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.finalizers = finalizers.into(); self
    }

    pub  fn finalizers(&mut self) -> &mut Vec<String> {
        if self.finalizers.is_none() { self.finalizers = Some(Default::default()) }
        self.finalizers.as_mut().unwrap()
    }

    /// Modify [`Self::finalizers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn finalizers_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.finalizers.is_none() { self.finalizers = Some(Default::default()) };
        func(self.finalizers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::finalizers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn finalizers_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.finalizers.is_none() {
            self.finalizers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.finalizers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::finalizers`]
    pub  fn finalizers_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.finalizers.is_none() { self.finalizers = Some(Vec::new()); }
         let finalizers = &mut self.finalizers.as_mut().unwrap();
         for item in other.borrow() {
             finalizers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::generate_name`]
    pub  fn generate_name_set(&mut self, generate_name: impl Into<Option<String>>) -> &mut Self {
        self.generate_name = generate_name.into(); self
    }

    pub  fn generate_name(&mut self) -> &mut String {
        if self.generate_name.is_none() { self.generate_name = Some(Default::default()) }
        self.generate_name.as_mut().unwrap()
    }

    /// Modify [`Self::generate_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn generate_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.generate_name.is_none() { self.generate_name = Some(Default::default()) };
        func(self.generate_name.as_mut().unwrap()); self
    }


    /// Set [`Self::generation`]
    pub  fn generation_set(&mut self, generation: impl Into<Option<i64>>) -> &mut Self {
        self.generation = generation.into(); self
    }

    pub  fn generation(&mut self) -> &mut i64 {
        if self.generation.is_none() { self.generation = Some(Default::default()) }
        self.generation.as_mut().unwrap()
    }

    /// Modify [`Self::generation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn generation_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.generation.is_none() { self.generation = Some(Default::default()) };
        func(self.generation.as_mut().unwrap()); self
    }


    /// Set [`Self::labels`]
    pub  fn labels_set(&mut self, labels: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.labels = labels.into(); self
    }

    pub  fn labels(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.labels.is_none() { self.labels = Some(Default::default()) }
        self.labels.as_mut().unwrap()
    }

    /// Modify [`Self::labels`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn labels_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.labels.is_none() { self.labels = Some(Default::default()) };
        func(self.labels.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::labels`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn labels_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.labels.is_none() {
            self.labels = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.labels.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::labels`]
    pub  fn labels_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.labels.is_none() { self.labels = Some(std::collections::BTreeMap::new()); }
         let labels = &mut self.labels.as_mut().unwrap();
         for (name, value) in other.borrow() {
             labels.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::managed_fields`]
    pub  fn managed_fields_set(&mut self, managed_fields: impl Into<Option<Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>>>) -> &mut Self {
        self.managed_fields = managed_fields.into(); self
    }

    pub  fn managed_fields(&mut self) -> &mut Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry> {
        if self.managed_fields.is_none() { self.managed_fields = Some(Default::default()) }
        self.managed_fields.as_mut().unwrap()
    }

    /// Modify [`Self::managed_fields`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn managed_fields_with(&mut self, func: impl FnOnce(&mut Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>)) -> &mut Self {
        if self.managed_fields.is_none() { self.managed_fields = Some(Default::default()) };
        func(self.managed_fields.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::managed_fields`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn managed_fields_push_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry)) -> &mut Self {
        if self.managed_fields.is_none() {
            self.managed_fields = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.managed_fields.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::managed_fields`]
    pub  fn managed_fields_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry]>) -> &mut Self {
         if self.managed_fields.is_none() { self.managed_fields = Some(Vec::new()); }
         let managed_fields = &mut self.managed_fields.as_mut().unwrap();
         for item in other.borrow() {
             managed_fields.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<Option<String>>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        if self.name.is_none() { self.name = Some(Default::default()) }
        self.name.as_mut().unwrap()
    }

    /// Modify [`Self::name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.name.is_none() { self.name = Some(Default::default()) };
        func(self.name.as_mut().unwrap()); self
    }


    /// Set [`Self::namespace`]
    pub  fn namespace_set(&mut self, namespace: impl Into<Option<String>>) -> &mut Self {
        self.namespace = namespace.into(); self
    }

    pub  fn namespace(&mut self) -> &mut String {
        if self.namespace.is_none() { self.namespace = Some(Default::default()) }
        self.namespace.as_mut().unwrap()
    }

    /// Modify [`Self::namespace`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn namespace_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.namespace.is_none() { self.namespace = Some(Default::default()) };
        func(self.namespace.as_mut().unwrap()); self
    }


    /// Set [`Self::owner_references`]
    pub  fn owner_references_set(&mut self, owner_references: impl Into<Option<Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>>>) -> &mut Self {
        self.owner_references = owner_references.into(); self
    }

    pub  fn owner_references(&mut self) -> &mut Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference> {
        if self.owner_references.is_none() { self.owner_references = Some(Default::default()) }
        self.owner_references.as_mut().unwrap()
    }

    /// Modify [`Self::owner_references`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn owner_references_with(&mut self, func: impl FnOnce(&mut Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>)) -> &mut Self {
        if self.owner_references.is_none() { self.owner_references = Some(Default::default()) };
        func(self.owner_references.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::owner_references`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn owner_references_push_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::OwnerReference)) -> &mut Self {
        if self.owner_references.is_none() {
            self.owner_references = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.owner_references.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::owner_references`]
    pub  fn owner_references_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apimachinery::pkg::apis::meta::v1::OwnerReference]>) -> &mut Self {
         if self.owner_references.is_none() { self.owner_references = Some(Vec::new()); }
         let owner_references = &mut self.owner_references.as_mut().unwrap();
         for item in other.borrow() {
             owner_references.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::resource_version`]
    pub  fn resource_version_set(&mut self, resource_version: impl Into<Option<String>>) -> &mut Self {
        self.resource_version = resource_version.into(); self
    }

    pub  fn resource_version(&mut self) -> &mut String {
        if self.resource_version.is_none() { self.resource_version = Some(Default::default()) }
        self.resource_version.as_mut().unwrap()
    }

    /// Modify [`Self::resource_version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resource_version.is_none() { self.resource_version = Some(Default::default()) };
        func(self.resource_version.as_mut().unwrap()); self
    }


    /// Set [`Self::self_link`]
    pub  fn self_link_set(&mut self, self_link: impl Into<Option<String>>) -> &mut Self {
        self.self_link = self_link.into(); self
    }

    pub  fn self_link(&mut self) -> &mut String {
        if self.self_link.is_none() { self.self_link = Some(Default::default()) }
        self.self_link.as_mut().unwrap()
    }

    /// Modify [`Self::self_link`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn self_link_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.self_link.is_none() { self.self_link = Some(Default::default()) };
        func(self.self_link.as_mut().unwrap()); self
    }


    /// Set [`Self::uid`]
    pub  fn uid_set(&mut self, uid: impl Into<Option<String>>) -> &mut Self {
        self.uid = uid.into(); self
    }

    pub  fn uid(&mut self) -> &mut String {
        if self.uid.is_none() { self.uid = Some(Default::default()) }
        self.uid.as_mut().unwrap()
    }

    /// Modify [`Self::uid`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn uid_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.uid.is_none() { self.uid = Some(Default::default()) };
        func(self.uid.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ObjectMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_annotations,
            Key_cluster_name,
            Key_creation_timestamp,
            Key_deletion_grace_period_seconds,
            Key_deletion_timestamp,
            Key_finalizers,
            Key_generate_name,
            Key_generation,
            Key_labels,
            Key_managed_fields,
            Key_name,
            Key_namespace,
            Key_owner_references,
            Key_resource_version,
            Key_self_link,
            Key_uid,
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
                            "annotations" => Field::Key_annotations,
                            "clusterName" => Field::Key_cluster_name,
                            "creationTimestamp" => Field::Key_creation_timestamp,
                            "deletionGracePeriodSeconds" => Field::Key_deletion_grace_period_seconds,
                            "deletionTimestamp" => Field::Key_deletion_timestamp,
                            "finalizers" => Field::Key_finalizers,
                            "generateName" => Field::Key_generate_name,
                            "generation" => Field::Key_generation,
                            "labels" => Field::Key_labels,
                            "managedFields" => Field::Key_managed_fields,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "ownerReferences" => Field::Key_owner_references,
                            "resourceVersion" => Field::Key_resource_version,
                            "selfLink" => Field::Key_self_link,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectMeta;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMeta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_annotations: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_cluster_name: Option<String> = None;
                let mut value_creation_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deletion_grace_period_seconds: Option<i64> = None;
                let mut value_deletion_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_finalizers: Option<Vec<String>> = None;
                let mut value_generate_name: Option<String> = None;
                let mut value_generation: Option<i64> = None;
                let mut value_labels: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_managed_fields: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_owner_references: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_self_link: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_annotations => value_annotations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_name => value_cluster_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_creation_timestamp => value_creation_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deletion_grace_period_seconds => value_deletion_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deletion_timestamp => value_deletion_timestamp = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_finalizers => value_finalizers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generate_name => value_generate_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generation => value_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_labels => value_labels = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_managed_fields => value_managed_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_owner_references => value_owner_references = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_self_link => value_self_link = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMeta {
                    annotations: value_annotations,
                    cluster_name: value_cluster_name,
                    creation_timestamp: value_creation_timestamp,
                    deletion_grace_period_seconds: value_deletion_grace_period_seconds,
                    deletion_timestamp: value_deletion_timestamp,
                    finalizers: value_finalizers,
                    generate_name: value_generate_name,
                    generation: value_generation,
                    labels: value_labels,
                    managed_fields: value_managed_fields,
                    name: value_name,
                    namespace: value_namespace,
                    owner_references: value_owner_references,
                    resource_version: value_resource_version,
                    self_link: value_self_link,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMeta",
            &[
                "annotations",
                "clusterName",
                "creationTimestamp",
                "deletionGracePeriodSeconds",
                "deletionTimestamp",
                "finalizers",
                "generateName",
                "generation",
                "labels",
                "managedFields",
                "name",
                "namespace",
                "ownerReferences",
                "resourceVersion",
                "selfLink",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ObjectMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMeta",
            self.annotations.as_ref().map_or(0, |_| 1) +
            self.cluster_name.as_ref().map_or(0, |_| 1) +
            self.creation_timestamp.as_ref().map_or(0, |_| 1) +
            self.deletion_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.deletion_timestamp.as_ref().map_or(0, |_| 1) +
            self.finalizers.as_ref().map_or(0, |_| 1) +
            self.generate_name.as_ref().map_or(0, |_| 1) +
            self.generation.as_ref().map_or(0, |_| 1) +
            self.labels.as_ref().map_or(0, |_| 1) +
            self.managed_fields.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.owner_references.as_ref().map_or(0, |_| 1) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.self_link.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.annotations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", value)?;
        }
        if let Some(value) = &self.cluster_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterName", value)?;
        }
        if let Some(value) = &self.creation_timestamp {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "creationTimestamp", value)?;
        }
        if let Some(value) = &self.deletion_grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deletionGracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.deletion_timestamp {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deletionTimestamp", value)?;
        }
        if let Some(value) = &self.finalizers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        if let Some(value) = &self.generate_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generateName", value)?;
        }
        if let Some(value) = &self.generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generation", value)?;
        }
        if let Some(value) = &self.labels {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labels", value)?;
        }
        if let Some(value) = &self.managed_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "managedFields", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.owner_references {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ownerReferences", value)?;
        }
        if let Some(value) = &self.resource_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.self_link {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selfLink", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ObjectMeta {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "annotations".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations".to_owned()),
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
                        "clusterName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Deprecated: ClusterName is a legacy field that was always cleared by the system and never used; it will be removed completely in 1.25.\n\nThe name in the go struct is changed to help clients detect accidental use.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "creationTimestamp".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.\n\nPopulated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deletionGracePeriodSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deletionTimestamp".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.\n\nPopulated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "finalizers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.".to_owned()),
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
                        "generateName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.\n\nIf this field is specified and the generated name exists, the server will return a 409.\n\nApplied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "generation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "labels".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels".to_owned()),
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
                        "managedFields".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like \"ci-cd\". The set of fields is always in the version that the workflow used when modifying the object.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the \"default\" namespace, but \"default\" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.\n\nMust be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ownerReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.\n\nPopulated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selfLink".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.\n\nPopulated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
