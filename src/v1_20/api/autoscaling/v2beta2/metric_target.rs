// Generated from definition io.k8s.api.autoscaling.v2beta2.MetricTarget

/// MetricTarget defines the target value, average value, or average utilization of a specific metric
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricTarget {
    /// averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
    pub average_utilization: Option<i32>,

    /// averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    pub average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// type represents whether the metric type is Utilization, Value, or AverageValue
    pub type_: String,

    /// value is the target value of the metric (as a quantity).
    pub value: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> crate::serde::Deserialize<'de> for MetricTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_average_utilization,
            Key_average_value,
            Key_type_,
            Key_value,
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
                            "averageUtilization" => Field::Key_average_utilization,
                            "averageValue" => Field::Key_average_value,
                            "type" => Field::Key_type_,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MetricTarget;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MetricTarget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_average_utilization: Option<i32> = None;
                let mut value_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_type_: Option<String> = None;
                let mut value_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_average_utilization => value_average_utilization = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_average_value => value_average_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricTarget {
                    average_utilization: value_average_utilization,
                    average_value: value_average_value,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "MetricTarget",
            &[
                "averageUtilization",
                "averageValue",
                "type",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MetricTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricTarget",
            1 +
            self.average_utilization.as_ref().map_or(0, |_| 1) +
            self.average_value.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.average_utilization {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "averageUtilization", value)?;
        }
        if let Some(value) = &self.average_value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "averageValue", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for MetricTarget {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "MetricTarget defines the target value, average value, or average utilization of a specific metric",
          "properties": {
            "averageUtilization": {
              "description": "averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type",
              "format": "int32",
              "type": "integer"
            },
            "averageValue": crate::schema_ref_with_description(crate::apimachinery::pkg::api::resource::Quantity::schema(), "averageValue is the target value of the average of the metric across all relevant pods (as a quantity)"),
            "type": {
              "description": "type represents whether the metric type is Utilization, Value, or AverageValue",
              "type": "string"
            },
            "value": crate::schema_ref_with_description(crate::apimachinery::pkg::api::resource::Quantity::schema(), "value is the target value of the metric (as a quantity).")
          },
          "required": [
            "type"
          ],
          "type": "object"
        })
    }
}
