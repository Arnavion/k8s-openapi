// Generated from definition io.k8s.api.autoscaling.v2beta1.ResourceMetricStatus

/// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceMetricStatus {
    /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.  It will only be present if `targetAverageValue` was set in the corresponding metric specification.
    pub current_average_utilization: Option<i32>,

    /// currentAverageValue is the current value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the "pods" metric source type. It will always be set, regardless of the corresponding metric specification.
    pub current_average_value: crate::apimachinery::pkg::api::resource::Quantity,

    /// name is the name of the resource in question.
    pub name: String,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_average_utilization,
            Key_current_average_value,
            Key_name,
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
                            "currentAverageUtilization" => Field::Key_current_average_utilization,
                            "currentAverageValue" => Field::Key_current_average_value,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_current_average_utilization: Option<i32> = None;
                let mut value_current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_average_utilization => value_current_average_utilization = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_average_value => value_current_average_value = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceMetricStatus {
                    current_average_utilization: value_current_average_utilization,
                    current_average_value: value_current_average_value.ok_or_else(|| crate::serde::de::Error::missing_field("currentAverageValue"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceMetricStatus",
            &[
                "currentAverageUtilization",
                "currentAverageValue",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceMetricStatus",
            2 +
            self.current_average_utilization.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.current_average_utilization {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageUtilization", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", &self.current_average_value)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl ResourceMetricStatus {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.",
          "properties": {
            "currentAverageUtilization": {
              "description": "currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.  It will only be present if `targetAverageValue` was set in the corresponding metric specification.",
              "format": "int32",
              "type": "integer"
            },
            "currentAverageValue": crate::schema_ref_with_description(crate::apimachinery::pkg::api::resource::Quantity::schema(), "currentAverageValue is the current value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the \"pods\" metric source type. It will always be set, regardless of the corresponding metric specification."),
            "name": {
              "description": "name is the name of the resource in question.",
              "type": "string"
            }
          },
          "required": [
            "currentAverageValue",
            "name"
          ],
          "type": "object"
        })
    }
}
