// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricStatus

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricStatus {
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    pub current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// currentValue is the current value of the metric (as a quantity)
    pub current_value: crate::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of a metric used for autoscaling in metric system.
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    pub metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> crate::serde::Deserialize<'de> for ExternalMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_average_value,
            Key_current_value,
            Key_metric_name,
            Key_metric_selector,
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
                            "currentAverageValue" => Field::Key_current_average_value,
                            "currentValue" => Field::Key_current_value,
                            "metricName" => Field::Key_metric_name,
                            "metricSelector" => Field::Key_metric_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_current_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_average_value => value_current_average_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_value => value_current_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric_name => value_metric_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric_selector => value_metric_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricStatus {
                    current_average_value: value_current_average_value,
                    current_value: value_current_value.unwrap_or_default(),
                    metric_name: value_metric_name.unwrap_or_default(),
                    metric_selector: value_metric_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricStatus",
            &[
                "currentAverageValue",
                "currentValue",
                "metricName",
                "metricSelector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricStatus",
            2 +
            self.current_average_value.as_ref().map_or(0, |_| 1) +
            self.metric_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.current_average_value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentValue", &self.current_value)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.metric_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricSelector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExternalMetricStatus {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.ExternalMetricStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "currentAverageValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentAverageValue is the current value of metric averaged over autoscaled pods.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "currentValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentValue is the current value of the metric (as a quantity)".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "metricName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("metricName is the name of a metric used for autoscaling in metric system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metricSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("metricSelector is used to identify a specific time series within a given metric.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "currentValue".to_owned(),
                    "metricName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
