// Generated from definition io.k8s.api.autoscaling.v2beta2.HorizontalPodAutoscalerBehavior

/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HorizontalPodAutoscalerBehavior {
    /// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
    pub scale_down: Option<crate::api::autoscaling::v2beta2::HPAScalingRules>,

    /// scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
    ///   * increase no more than 4 pods per 60 seconds
    ///   * double the number of pods per 60 seconds
    /// No stabilization is used.
    pub scale_up: Option<crate::api::autoscaling::v2beta2::HPAScalingRules>,
}

impl<'de> serde::Deserialize<'de> for HorizontalPodAutoscalerBehavior {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_scale_down,
            Key_scale_up,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HorizontalPodAutoscalerBehavior;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HorizontalPodAutoscalerBehavior")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_scale_down: Option<crate::api::autoscaling::v2beta2::HPAScalingRules> = None;
                let mut value_scale_up: Option<crate::api::autoscaling::v2beta2::HPAScalingRules> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_scale_down => value_scale_down = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_up => value_scale_up = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for HorizontalPodAutoscalerBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HorizontalPodAutoscalerBehavior",
            self.scale_down.as_ref().map_or(0, |_| 1) +
            self.scale_up.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.scale_down {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scaleDown", value)?;
        }
        if let Some(value) = &self.scale_up {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scaleUp", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
