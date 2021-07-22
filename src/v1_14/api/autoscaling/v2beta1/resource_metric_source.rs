// Generated from definition io.k8s.api.autoscaling.v2beta1.ResourceMetricSource

/// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.  Only one "target" type should be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    pub name: String,

    /// targetAverageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
    pub target_average_utilization: Option<i32>,

    /// targetAverageValue is the target value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the "pods" metric source type.
    pub target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_target_average_utilization,
            Key_target_average_value,
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
                            "name" => Field::Key_name,
                            "targetAverageUtilization" => Field::Key_target_average_utilization,
                            "targetAverageValue" => Field::Key_target_average_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_target_average_utilization: Option<i32> = None;
                let mut value_target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target_average_utilization => value_target_average_utilization = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_average_value => value_target_average_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceMetricSource {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    target_average_utilization: value_target_average_utilization,
                    target_average_value: value_target_average_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceMetricSource",
            &[
                "name",
                "targetAverageUtilization",
                "targetAverageValue",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceMetricSource",
            1 +
            self.target_average_utilization.as_ref().map_or(0, |_| 1) +
            self.target_average_value.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.target_average_utilization {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetAverageUtilization", value)?;
        }
        if let Some(value) = &self.target_average_value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetAverageValue", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ResourceMetricSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.  Only one \"target\" type should be set.",
          "properties": {
            "name": {
              "description": "name is the name of the resource in question.",
              "type": "string"
            },
            "targetAverageUtilization": {
              "description": "targetAverageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.",
              "format": "int32",
              "type": "integer"
            },
            "targetAverageValue": crate::schema_ref_with_values(crate::apimachinery::pkg::api::resource::Quantity::schema(), serde_json::json!({"description": "targetAverageValue is the target value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the \"pods\" metric source type."}))
          },
          "required": [
            "name"
          ],
          "type": "object"
        })
    }
}
