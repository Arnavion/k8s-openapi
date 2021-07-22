// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMeta {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    pub annotations: std::collections::BTreeMap<String, String>,

    /// The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
    pub cluster_name: Option<String>,

    /// CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
    ///
    /// Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub creation_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
    pub deletion_grace_period_seconds: Option<i64>,

    /// DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub deletion_timestamp: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed.
    pub finalizers: Vec<String>,

    /// GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
    ///
    /// If this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).
    ///
    /// Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency
    pub generate_name: Option<String>,

    /// A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.
    pub generation: Option<i64>,

    /// An initializer is a controller which enforces some system invariant at object creation time. This field is a list of initializers that have not yet acted on this object. If nil or empty, this object has been completely initialized. Otherwise, the object is considered uninitialized and is hidden (in list/watch and get calls) from clients that haven't explicitly asked to observe uninitialized objects.
    ///
    /// When an object is created, the system will populate this list with the current set of initializers. Only privileged users may set or modify this list. Once it is empty, it may not be modified further by any user.
    ///
    /// DEPRECATED - initializers are an alpha field and will be removed in v1.15.
    pub initializers: Option<crate::apimachinery::pkg::apis::meta::v1::Initializers>,

    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    pub labels: std::collections::BTreeMap<String, String>,

    /// ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.
    ///
    /// This field is alpha and can be changed or removed without notice.
    pub managed_fields: Vec<crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry>,

    /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: Option<String>,

    /// Namespace defines the space within each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
    pub namespace: Option<String>,

    /// List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
    pub owner_references: Vec<crate::apimachinery::pkg::apis::meta::v1::OwnerReference>,

    /// An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.
    ///
    /// Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// SelfLink is a URL representing this object. Populated by the system. Read-only.
    pub self_link: Option<String>,

    /// UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.
    ///
    /// Populated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: Option<String>,
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
            Key_initializers,
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
                            "initializers" => Field::Key_initializers,
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
                let mut value_initializers: Option<crate::apimachinery::pkg::apis::meta::v1::Initializers> = None;
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
                        Field::Key_initializers => value_initializers = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    annotations: value_annotations.unwrap_or_default(),
                    cluster_name: value_cluster_name,
                    creation_timestamp: value_creation_timestamp,
                    deletion_grace_period_seconds: value_deletion_grace_period_seconds,
                    deletion_timestamp: value_deletion_timestamp,
                    finalizers: value_finalizers.unwrap_or_default(),
                    generate_name: value_generate_name,
                    generation: value_generation,
                    initializers: value_initializers,
                    labels: value_labels.unwrap_or_default(),
                    managed_fields: value_managed_fields.unwrap_or_default(),
                    name: value_name,
                    namespace: value_namespace,
                    owner_references: value_owner_references.unwrap_or_default(),
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
                "initializers",
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
            usize::from(!self.annotations.is_empty()) +
            self.cluster_name.as_ref().map_or(0, |_| 1) +
            self.creation_timestamp.as_ref().map_or(0, |_| 1) +
            self.deletion_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.deletion_timestamp.as_ref().map_or(0, |_| 1) +
            usize::from(!self.finalizers.is_empty()) +
            self.generate_name.as_ref().map_or(0, |_| 1) +
            self.generation.as_ref().map_or(0, |_| 1) +
            self.initializers.as_ref().map_or(0, |_| 1) +
            usize::from(!self.labels.is_empty()) +
            usize::from(!self.managed_fields.is_empty()) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            usize::from(!self.owner_references.is_empty()) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.self_link.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if !self.annotations.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", &self.annotations)?;
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
        if !self.finalizers.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", &self.finalizers)?;
        }
        if let Some(value) = &self.generate_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generateName", value)?;
        }
        if let Some(value) = &self.generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generation", value)?;
        }
        if let Some(value) = &self.initializers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initializers", value)?;
        }
        if !self.labels.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labels", &self.labels)?;
        }
        if !self.managed_fields.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "managedFields", &self.managed_fields)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if !self.owner_references.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ownerReferences", &self.owner_references)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for ObjectMeta {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.",
          "properties": {
            "annotations": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations",
              "type": "object"
            },
            "clusterName": {
              "description": "The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.",
              "type": "string"
            },
            "creationTimestamp": crate::schema_ref_with_values(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), serde_json::json!({"description": "CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.\n\nPopulated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"})),
            "deletionGracePeriodSeconds": {
              "description": "Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.",
              "format": "int64",
              "type": "integer"
            },
            "deletionTimestamp": crate::schema_ref_with_values(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), serde_json::json!({"description": "DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.\n\nPopulated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"})),
            "finalizers": {
              "description": "Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed.",
              "items": {
                "type": "string"
              },
              "x-kubernetes-patch-strategy": "merge",
              "type": "array"
            },
            "generateName": {
              "description": "GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.\n\nIf this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).\n\nApplied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency",
              "type": "string"
            },
            "generation": {
              "description": "A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.",
              "format": "int64",
              "type": "integer"
            },
            "initializers": crate::schema_ref_with_values(crate::apimachinery::pkg::apis::meta::v1::Initializers::schema(), serde_json::json!({"description": "An initializer is a controller which enforces some system invariant at object creation time. This field is a list of initializers that have not yet acted on this object. If nil or empty, this object has been completely initialized. Otherwise, the object is considered uninitialized and is hidden (in list/watch and get calls) from clients that haven't explicitly asked to observe uninitialized objects.\n\nWhen an object is created, the system will populate this list with the current set of initializers. Only privileged users may set or modify this list. Once it is empty, it may not be modified further by any user.\n\nDEPRECATED - initializers are an alpha field and will be removed in v1.15."})),
            "labels": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels",
              "type": "object"
            },
            "managedFields": {
              "description": "ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like \"ci-cd\". The set of fields is always in the version that the workflow used when modifying the object.\n\nThis field is alpha and can be changed or removed without notice.",
              "items": crate::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry::schema(),
              "type": "array"
            },
            "name": {
              "description": "Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
              "type": "string"
            },
            "namespace": {
              "description": "Namespace defines the space within each name must be unique. An empty namespace is equivalent to the \"default\" namespace, but \"default\" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.\n\nMust be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces",
              "type": "string"
            },
            "ownerReferences": {
              "description": "List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.",
              "items": crate::apimachinery::pkg::apis::meta::v1::OwnerReference::schema(),
              "x-kubernetes-patch-merge-key": "uid",
              "x-kubernetes-patch-strategy": "merge",
              "type": "array"
            },
            "resourceVersion": {
              "description": "An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.\n\nPopulated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
              "type": "string"
            },
            "selfLink": {
              "description": "SelfLink is a URL representing this object. Populated by the system. Read-only.",
              "type": "string"
            },
            "uid": {
              "description": "UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.\n\nPopulated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
