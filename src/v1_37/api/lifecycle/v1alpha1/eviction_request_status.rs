// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionRequestStatus

/// EvictionRequestStatus represents the last observed status of the eviction request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionRequestStatus {
    /// conditions contain information about the eviction request.
    ///
    /// EvictionRequest specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed
    ///   by any eviction responder. This can happen if the request is canceled or if no responder
    ///   managed to evict the target (e.g. terminate or delete a pod).
    /// - TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).
    ///
    /// These conditions can be reset if the eviction was unsuccessful and a new Eviction intent has been submitted.
    ///
    /// The maximum length of the conditions list is 100.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// observedGeneration is EvictionRequest's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.
    pub observed_generation: Option<i64>,
}

impl crate::DeepMerge for EvictionRequestStatus {
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
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_observed_generation,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionRequestStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_observed_generation: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionRequestStatus {
                    conditions: value_conditions,
                    observed_generation: value_observed_generation,
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionRequestStatus",
            &[
                "conditions",
                "observedGeneration",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionRequestStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionRequestStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionRequestStatus represents the last observed status of the eviction request.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "conditions contain information about the eviction request.\n\nEvictionRequest specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed\n  by any eviction responder. This can happen if the request is canceled or if no responder\n  managed to evict the target (e.g. terminate or delete a pod).\n- TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).\n\nThese conditions can be reset if the eviction was unsuccessful and a new Eviction intent has been submitted.\n\nThe maximum length of the conditions list is 100.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "observedGeneration": {
                    "description": "observedGeneration is EvictionRequest's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.",
                    "type": "integer",
                    "format": "int64",
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionRequestStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionRequestStatus represents the last observed status of the eviction request.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "conditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("conditions contain information about the eviction request.\n\nEvictionRequest specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed\n  by any eviction responder. This can happen if the request is canceled or if no responder\n  managed to evict the target (e.g. terminate or delete a pod).\n- TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).\n\nThese conditions can be reset if the eviction was unsuccessful and a new Eviction intent has been submitted.\n\nThe maximum length of the conditions list is 100.".into()),
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
                                description: Some("observedGeneration is EvictionRequest's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
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
