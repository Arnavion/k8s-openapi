// Generated from definition io.k8s.api.lifecycle.v1alpha1.Eviction

/// Eviction initiates an eviction process, which should ideally result in a graceful eviction of a .spec.target (e.g. termination of a pod).
///
/// The evictionrequest-controller observes intents of all EvictionRequests and transforms them into Evictions. It manages the Eviction lifecycle. Requesters are preserved in .status.requesters even after they have withdrawn their request. If all requesters withdraw their eviction intent for a common target, the eviction will be canceled. Once all EvictionRequest corresponding to this Eviction .spec.target have been removed, this Eviction object will eventually be garbage collected.
///
/// If the target is a pod, the .status.targetResponders is populated from Pod's .spec.evictionResponders.
///
/// Responders should observe and communicate through the .status to help with the eviction of the target when they see their state == Active in .status.targetResponders. ResponderStatus struct should then be periodically updated to indicate the progress or completion of the eviction process by each responder in .status.responders. If .status.responders\[\].heartbeatTime is not updated within the heartbeat deadline defined by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder with a lower priority.
///
/// If there are no other responders and the target is a pod, the last default imperative-eviction.k8s.io/evictor responder with a priority of 100 will evict the pod using the imperative Eviction API (pods/\<name\>/eviction subresource).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Eviction {
    /// metadata is the standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata. .metadata.name set by the evictionrequest-controller is purely informative and subject to change. .spec.target field should be used to identify the target precisesly.
    ///
    /// The requester and responder names will be used as label keys and added to the labels of the eviction in one of the following formats: 1. acme.io/foo: "requester" 2. acme.io/foo: "responder" 3. acme.io/foo: "requester-responder"
    ///
    /// Please see EvictionParticipantRole for available role label values.
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// spec defines the eviction specification. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: crate::api::lifecycle::v1alpha1::EvictionSpec,

    /// status represents the most recently observed status of the eviction. Populated by responders and evictionrequest-controller. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<crate::api::lifecycle::v1alpha1::EvictionStatus>,
}

impl crate::Resource for Eviction {
    const API_VERSION: &'static str = "lifecycle.k8s.io/v1alpha1";
    const GROUP: &'static str = "lifecycle.k8s.io";
    const KIND: &'static str = "Eviction";
    const VERSION: &'static str = "v1alpha1";
    const URL_PATH_SEGMENT: &'static str = "evictions";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for Eviction {
    const LIST_KIND: &'static str = "EvictionList";
}

impl crate::Metadata for Eviction {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for Eviction {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.spec, other.spec);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Eviction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Eviction;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::lifecycle::v1alpha1::EvictionSpec> = None;
                let mut value_status: Option<crate::api::lifecycle::v1alpha1::EvictionStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Eviction {
                    metadata: value_metadata.unwrap_or_default(),
                    spec: value_spec.unwrap_or_default(),
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
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

impl crate::serde::Serialize for Eviction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        if let Some(value) = &self.status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Eviction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.Eviction".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Eviction initiates an eviction process, which should ideally result in a graceful eviction of a .spec.target (e.g. termination of a pod).\n\nThe evictionrequest-controller observes intents of all EvictionRequests and transforms them into Evictions. It manages the Eviction lifecycle. Requesters are preserved in .status.requesters even after they have withdrawn their request. If all requesters withdraw their eviction intent for a common target, the eviction will be canceled. Once all EvictionRequest corresponding to this Eviction .spec.target have been removed, this Eviction object will eventually be garbage collected.\n\nIf the target is a pod, the .status.targetResponders is populated from Pod's .spec.evictionResponders.\n\nResponders should observe and communicate through the .status to help with the eviction of the target when they see their state == Active in .status.targetResponders. ResponderStatus struct should then be periodically updated to indicate the progress or completion of the eviction process by each responder in .status.responders. If .status.responders[].heartbeatTime is not updated within the heartbeat deadline defined by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder with a lower priority.\n\nIf there are no other responders and the target is a pod, the last default imperative-eviction.k8s.io/evictor responder with a priority of 100 will evict the pod using the imperative Eviction API (pods/<name>/eviction subresource).",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>();
                    schema_obj.ensure_object().insert("description".into(), "metadata is the standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata. .metadata.name set by the evictionrequest-controller is purely informative and subject to change. .spec.target field should be used to identify the target precisesly.\n\nThe requester and responder names will be used as label keys and added to the labels of the eviction in one of the following formats: 1. acme.io/foo: \"requester\" 2. acme.io/foo: \"responder\" 3. acme.io/foo: \"requester-responder\"\n\nPlease see EvictionParticipantRole for available role label values.".into());
                    schema_obj
                }),
                "spec": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionSpec>();
                    schema_obj.ensure_object().insert("description".into(), "spec defines the eviction specification. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into());
                    schema_obj
                }),
                "status": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionStatus>();
                    schema_obj.ensure_object().insert("description".into(), "status represents the most recently observed status of the eviction. Populated by responders and evictionrequest-controller. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into());
                    schema_obj
                }),
            },
            "required": [
                "metadata",
                "spec",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for Eviction {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.Eviction".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("Eviction initiates an eviction process, which should ideally result in a graceful eviction of a .spec.target (e.g. termination of a pod).\n\nThe evictionrequest-controller observes intents of all EvictionRequests and transforms them into Evictions. It manages the Eviction lifecycle. Requesters are preserved in .status.requesters even after they have withdrawn their request. If all requesters withdraw their eviction intent for a common target, the eviction will be canceled. Once all EvictionRequest corresponding to this Eviction .spec.target have been removed, this Eviction object will eventually be garbage collected.\n\nIf the target is a pod, the .status.targetResponders is populated from Pod's .spec.evictionResponders.\n\nResponders should observe and communicate through the .status to help with the eviction of the target when they see their state == Active in .status.targetResponders. ResponderStatus struct should then be periodically updated to indicate the progress or completion of the eviction process by each responder in .status.responders. If .status.responders[].heartbeatTime is not updated within the heartbeat deadline defined by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder with a lower priority.\n\nIf there are no other responders and the target is a pod, the last default imperative-eviction.k8s.io/evictor responder with a priority of 100 will evict the pod using the imperative Eviction API (pods/<name>/eviction subresource).".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("metadata is the standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata. .metadata.name set by the evictionrequest-controller is purely informative and subject to change. .spec.target field should be used to identify the target precisesly.\n\nThe requester and responder names will be used as label keys and added to the labels of the eviction in one of the following formats: 1. acme.io/foo: \"requester\" 2. acme.io/foo: \"responder\" 3. acme.io/foo: \"requester-responder\"\n\nPlease see EvictionParticipantRole for available role label values.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "spec".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionSpec>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("spec defines the eviction specification. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("status represents the most recently observed status of the eviction. Populated by responders and evictionrequest-controller. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "metadata".into(),
                    "spec".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
