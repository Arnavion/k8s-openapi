// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectMeta {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    pub annotations: Option<::std::collections::BTreeMap<String, String>>,

    /// The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
    pub cluster_name: Option<String>,

    /// CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
    ///
    /// Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub creation_timestamp: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
    pub deletion_grace_period_seconds: Option<i64>,

    /// DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub deletion_timestamp: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed.
    pub finalizers: Option<Vec<String>>,

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
    pub initializers: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Initializers>,

    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    pub labels: Option<::std::collections::BTreeMap<String, String>>,

    /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: Option<String>,

    /// Namespace defines the space within each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
    pub namespace: Option<String>,

    /// List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
    pub owner_references: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::OwnerReference>>,

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

impl<'de> ::serde::Deserialize<'de> for ObjectMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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
            Key_name,
            Key_namespace,
            Key_owner_references,
            Key_resource_version,
            Key_self_link,
            Key_uid,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectMeta;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ObjectMeta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_annotations: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_cluster_name: Option<String> = None;
                let mut value_creation_timestamp: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_deletion_grace_period_seconds: Option<i64> = None;
                let mut value_deletion_timestamp: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_finalizers: Option<Vec<String>> = None;
                let mut value_generate_name: Option<String> = None;
                let mut value_generation: Option<i64> = None;
                let mut value_initializers: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Initializers> = None;
                let mut value_labels: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_owner_references: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::OwnerReference>> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_self_link: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_annotations => value_annotations = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_name => value_cluster_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_creation_timestamp => value_creation_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deletion_grace_period_seconds => value_deletion_grace_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deletion_timestamp => value_deletion_timestamp = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_finalizers => value_finalizers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generate_name => value_generate_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generation => value_generation = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initializers => value_initializers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_labels => value_labels = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_owner_references => value_owner_references = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_self_link => value_self_link = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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
                    initializers: value_initializers,
                    labels: value_labels,
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
                "initializers",
                "labels",
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

impl ::serde::Serialize for ObjectMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMeta",
            0 +
            self.annotations.as_ref().map_or(0, |_| 1) +
            self.cluster_name.as_ref().map_or(0, |_| 1) +
            self.creation_timestamp.as_ref().map_or(0, |_| 1) +
            self.deletion_grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.deletion_timestamp.as_ref().map_or(0, |_| 1) +
            self.finalizers.as_ref().map_or(0, |_| 1) +
            self.generate_name.as_ref().map_or(0, |_| 1) +
            self.generation.as_ref().map_or(0, |_| 1) +
            self.initializers.as_ref().map_or(0, |_| 1) +
            self.labels.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.owner_references.as_ref().map_or(0, |_| 1) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.self_link.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.annotations {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "annotations", value)?;
        }
        if let Some(value) = &self.cluster_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterName", value)?;
        }
        if let Some(value) = &self.creation_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "creationTimestamp", value)?;
        }
        if let Some(value) = &self.deletion_grace_period_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deletionGracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.deletion_timestamp {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "deletionTimestamp", value)?;
        }
        if let Some(value) = &self.finalizers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        if let Some(value) = &self.generate_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "generateName", value)?;
        }
        if let Some(value) = &self.generation {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "generation", value)?;
        }
        if let Some(value) = &self.initializers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "initializers", value)?;
        }
        if let Some(value) = &self.labels {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "labels", value)?;
        }
        if let Some(value) = &self.name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.owner_references {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ownerReferences", value)?;
        }
        if let Some(value) = &self.resource_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.self_link {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selfLink", value)?;
        }
        if let Some(value) = &self.uid {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
