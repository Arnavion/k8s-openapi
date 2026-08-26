// Generated from definition io.k8s.api.core.v1.VolumeHealthStatus

/// VolumeHealthStatus contains health information for a volume reported by the CSI controller plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeHealthStatus {
    /// conditions is the set of adverse conditions reported by the CSI controller plugin. An empty list means no adverse condition. At most 16 conditions may be reported.
    pub health_conditions: Option<std::vec::Vec<crate::api::core::v1::VolumeHealthCondition>>,

    /// lastTransitionTime is when the current set of conditions first appeared.
    pub last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for VolumeHealthStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.health_conditions,
            other.health_conditions,
            &[|lhs, rhs| lhs.status == rhs.status],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.last_transition_time, other.last_transition_time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeHealthStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_health_conditions,
            Key_last_transition_time,
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
                            "healthConditions" => Field::Key_health_conditions,
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeHealthStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeHealthStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_health_conditions: Option<std::vec::Vec<crate::api::core::v1::VolumeHealthCondition>> = None;
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_health_conditions => value_health_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_transition_time => value_last_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeHealthStatus {
                    health_conditions: value_health_conditions,
                    last_transition_time: value_last_transition_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeHealthStatus",
            &[
                "healthConditions",
                "lastTransitionTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeHealthStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeHealthStatus",
            self.health_conditions.as_ref().map_or(0, |_| 1) +
            self.last_transition_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.health_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "healthConditions", value)?;
        }
        if let Some(value) = &self.last_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeHealthStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.VolumeHealthStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "VolumeHealthStatus contains health information for a volume reported by the CSI controller plugin.",
            "type": "object",
            "properties": {
                "healthConditions": {
                    "description": "conditions is the set of adverse conditions reported by the CSI controller plugin. An empty list means no adverse condition. At most 16 conditions may be reported.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::VolumeHealthCondition>()),
                },
                "lastTransitionTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "lastTransitionTime is when the current set of conditions first appeared.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for VolumeHealthStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.VolumeHealthStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("VolumeHealthStatus contains health information for a volume reported by the CSI controller plugin.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "healthConditions".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("conditions is the set of adverse conditions reported by the CSI controller plugin. An empty list means no adverse condition. At most 16 conditions may be reported.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::VolumeHealthCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lastTransitionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("lastTransitionTime is when the current set of conditions first appeared.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
