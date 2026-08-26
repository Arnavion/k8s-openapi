// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionStatus

/// EvictionStatus represents the last observed status of the eviction request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionStatus {
    /// conditions contain information about the eviction request.
    ///
    /// Eviction specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed
    ///   by any eviction responder. This can happen if the request is canceled or if no responder
    ///   managed to evict the target (e.g. terminate or delete a pod).
    /// - TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).
    ///
    ///   The maximum length of the conditions list is 100.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// observedGeneration is Eviction's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.
    pub observed_generation: Option<i64>,

    /// requesters allow you to identify the entities, that requested the eviction of the target. If all the requesters withdraw their eviction intent, the eviction will be canceled.
    ///
    /// The maximum length of the requesters list is 100. If this limit is exceeded, requesters with Withdrawn intent should be dropped first.
    pub requesters: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::Requester>>,

    /// responders represents the eviction process status of each declared responder.
    ///
    /// The responder list should be the same length and have the same .name fields as .status.targetResponders. Only responders with .name that have Active state in .targetResponders\[\].state should be updated and can be mutated. First initialization of the list is allowed.
    ///
    /// Each ResponderStatus is initialized by evictionrequest-controller and then managed by the designated responder.
    pub responders: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::ResponderStatus>>,

    /// targetResponders reference responders that should eventually respond to this eviction to help with the graceful eviction of a target. These responders are selected sequentially, according to their specified priority by setting the Active state to the TargetResponder .state field. The maximum number of active responders allowed is 1. Eventually each responder can end up in an Interrupted, Canceled or, Completed state. Responders should observe these states in order to navigate their lifecycle.
    ///
    /// If the target is a pod, the field is populated from Pod's .spec.evictionResponders. Default responders may be added to the list according to the target.
    ///
    /// Default responders: - imperative-eviction.k8s.io/evictor responder with a priority of 100 is added to the list if the
    ///   target is a pod. It will call the imperative Eviction API (pods/\<name\>/eviction subresource).
    ///   This call may not succeed due to PodDisruptionBudgets, which may block the pod termination.
    ///   It will update the responder message and try again with a backoff.
    ///
    /// The maximum length of the responders list is 11. The length and keys of the list cannot change once set. This field is managed by evictionrequest-controller.
    pub target_responders: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::TargetResponder>>,
}

impl crate::DeepMerge for EvictionStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::merge_strategies::list::map(
            &mut self.requesters,
            other.requesters,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.responders,
            other.responders,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.target_responders,
            other.target_responders,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_observed_generation,
            Key_requesters,
            Key_responders,
            Key_target_responders,
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
                            "conditions" => Field::Key_conditions,
                            "observedGeneration" => Field::Key_observed_generation,
                            "requesters" => Field::Key_requesters,
                            "responders" => Field::Key_responders,
                            "targetResponders" => Field::Key_target_responders,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_requesters: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::Requester>> = None;
                let mut value_responders: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::ResponderStatus>> = None;
                let mut value_target_responders: Option<std::vec::Vec<crate::api::lifecycle::v1alpha1::TargetResponder>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requesters => value_requesters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_responders => value_responders = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_responders => value_target_responders = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionStatus {
                    conditions: value_conditions,
                    observed_generation: value_observed_generation,
                    requesters: value_requesters,
                    responders: value_responders,
                    target_responders: value_target_responders,
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionStatus",
            &[
                "conditions",
                "observedGeneration",
                "requesters",
                "responders",
                "targetResponders",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.requesters.as_ref().map_or(0, |_| 1) +
            self.responders.as_ref().map_or(0, |_| 1) +
            self.target_responders.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.requesters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requesters", value)?;
        }
        if let Some(value) = &self.responders {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "responders", value)?;
        }
        if let Some(value) = &self.target_responders {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetResponders", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionStatus represents the last observed status of the eviction request.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "conditions contain information about the eviction request.\n\nEviction specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed\n  by any eviction responder. This can happen if the request is canceled or if no responder\n  managed to evict the target (e.g. terminate or delete a pod).\n- TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).\n\n\tThe maximum length of the conditions list is 100.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "observedGeneration": {
                    "description": "observedGeneration is Eviction's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.",
                    "type": "integer",
                    "format": "int64",
                },
                "requesters": {
                    "description": "requesters allow you to identify the entities, that requested the eviction of the target. If all the requesters withdraw their eviction intent, the eviction will be canceled.\n\nThe maximum length of the requesters list is 100. If this limit is exceeded, requesters with Withdrawn intent should be dropped first.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::lifecycle::v1alpha1::Requester>()),
                },
                "responders": {
                    "description": "responders represents the eviction process status of each declared responder.\n\nThe responder list should be the same length and have the same .name fields as .status.targetResponders. Only responders with .name that have Active state in .targetResponders[].state should be updated and can be mutated. First initialization of the list is allowed.\n\nEach ResponderStatus is initialized by evictionrequest-controller and then managed by the designated responder.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::lifecycle::v1alpha1::ResponderStatus>()),
                },
                "targetResponders": {
                    "description": "targetResponders reference responders that should eventually respond to this eviction to help with the graceful eviction of a target. These responders are selected sequentially, according to their specified priority by setting the Active state to the TargetResponder .state field. The maximum number of active responders allowed is 1. Eventually each responder can end up in an Interrupted, Canceled or, Completed state. Responders should observe these states in order to navigate their lifecycle.\n\nIf the target is a pod, the field is populated from Pod's .spec.evictionResponders. Default responders may be added to the list according to the target.\n\nDefault responders: - imperative-eviction.k8s.io/evictor responder with a priority of 100 is added to the list if the\n  target is a pod. It will call the imperative Eviction API (pods/<name>/eviction subresource).\n  This call may not succeed due to PodDisruptionBudgets, which may block the pod termination.\n  It will update the responder message and try again with a backoff.\n\nThe maximum length of the responders list is 11. The length and keys of the list cannot change once set. This field is managed by evictionrequest-controller.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::lifecycle::v1alpha1::TargetResponder>()),
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionStatus represents the last observed status of the eviction request.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("conditions contain information about the eviction request.\n\nEviction specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed\n  by any eviction responder. This can happen if the request is canceled or if no responder\n  managed to evict the target (e.g. terminate or delete a pod).\n- TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).\n\n\tThe maximum length of the conditions list is 100.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observedGeneration".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("observedGeneration is Eviction's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requesters".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("requesters allow you to identify the entities, that requested the eviction of the target. If all the requesters withdraw their eviction intent, the eviction will be canceled.\n\nThe maximum length of the requesters list is 100. If this limit is exceeded, requesters with Withdrawn intent should be dropped first.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::lifecycle::v1alpha1::Requester>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "responders".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("responders represents the eviction process status of each declared responder.\n\nThe responder list should be the same length and have the same .name fields as .status.targetResponders. Only responders with .name that have Active state in .targetResponders[].state should be updated and can be mutated. First initialization of the list is allowed.\n\nEach ResponderStatus is initialized by evictionrequest-controller and then managed by the designated responder.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::lifecycle::v1alpha1::ResponderStatus>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetResponders".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("targetResponders reference responders that should eventually respond to this eviction to help with the graceful eviction of a target. These responders are selected sequentially, according to their specified priority by setting the Active state to the TargetResponder .state field. The maximum number of active responders allowed is 1. Eventually each responder can end up in an Interrupted, Canceled or, Completed state. Responders should observe these states in order to navigate their lifecycle.\n\nIf the target is a pod, the field is populated from Pod's .spec.evictionResponders. Default responders may be added to the list according to the target.\n\nDefault responders: - imperative-eviction.k8s.io/evictor responder with a priority of 100 is added to the list if the\n  target is a pod. It will call the imperative Eviction API (pods/<name>/eviction subresource).\n  This call may not succeed due to PodDisruptionBudgets, which may block the pod termination.\n  It will update the responder message and try again with a backoff.\n\nThe maximum length of the responders list is 11. The length and keys of the list cannot change once set. This field is managed by evictionrequest-controller.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::lifecycle::v1alpha1::TargetResponder>()))),
                                ..Default::default()
                            })),
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
