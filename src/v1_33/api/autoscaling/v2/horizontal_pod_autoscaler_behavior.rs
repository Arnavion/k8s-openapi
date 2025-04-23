// Generated from definition io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerBehavior

/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerBehavior {
    /// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
    pub scale_down: Option<crate::api::autoscaling::v2::HPAScalingRules>,

    /// scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
    ///   * increase no more than 4 pods per 60 seconds
    ///   * double the number of pods per 60 seconds
    /// No stabilization is used.
    pub scale_up: Option<crate::api::autoscaling::v2::HPAScalingRules>,
}

impl crate::DeepMerge for HorizontalPodAutoscalerBehavior {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.scale_down, other.scale_down);
        crate::DeepMerge::merge_from(&mut self.scale_up, other.scale_up);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HorizontalPodAutoscalerBehavior {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_scale_down,
            Key_scale_up,
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
                            "scaleDown" => Field::Key_scale_down,
                            "scaleUp" => Field::Key_scale_up,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerBehavior;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HorizontalPodAutoscalerBehavior")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_scale_down: Option<crate::api::autoscaling::v2::HPAScalingRules> = None;
                let mut value_scale_up: Option<crate::api::autoscaling::v2::HPAScalingRules> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_scale_down => value_scale_down = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_up => value_scale_up = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HorizontalPodAutoscalerBehavior {
                    scale_down: value_scale_down,
                    scale_up: value_scale_up,
                })
            }
        }

        deserializer.deserialize_struct(
            "HorizontalPodAutoscalerBehavior",
            &[
                "scaleDown",
                "scaleUp",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HorizontalPodAutoscalerBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerBehavior",
            self.scale_down.as_ref().map_or(0, |_| 1) +
            self.scale_up.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.scale_down {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleDown", value)?;
        }
        if let Some(value) = &self.scale_up {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleUp", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HorizontalPodAutoscalerBehavior {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerBehavior".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "scaleDown".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::HPAScalingRules>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scaleUp".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::HPAScalingRules>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:\n  * increase no more than 4 pods per 60 seconds\n  * double the number of pods per 60 seconds\nNo stabilization is used.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
