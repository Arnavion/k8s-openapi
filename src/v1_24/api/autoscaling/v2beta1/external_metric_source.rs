// Generated from definition io.k8s.api.autoscaling.v2beta1.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one "target" type should be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricSource {
    /// metricName is the name of the metric in question.
    pub metric_name: String,

    /// metricSelector is used to identify a specific time series within a given metric.
    pub metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue.
    pub target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue.
    pub target_value: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> crate::serde::Deserialize<'de> for ExternalMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric_name,
            Key_metric_selector,
            Key_target_average_value,
            Key_target_value,
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
                            "metricName" => Field::Key_metric_name,
                            "metricSelector" => Field::Key_metric_selector,
                            "targetAverageValue" => Field::Key_target_average_value,
                            "targetValue" => Field::Key_target_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metric_name: Option<String> = None;
                let mut value_metric_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_target_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric_name => value_metric_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric_selector => value_metric_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_average_value => value_target_average_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_value => value_target_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricSource {
                    metric_name: value_metric_name.unwrap_or_default(),
                    metric_selector: value_metric_selector,
                    target_average_value: value_target_average_value,
                    target_value: value_target_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricSource",
            &[
                "metricName",
                "metricSelector",
                "targetAverageValue",
                "targetValue",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricSource",
            1 +
            self.metric_selector.as_ref().map_or(0, |_| 1) +
            self.target_average_value.as_ref().map_or(0, |_| 1) +
            self.target_value.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.metric_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricSelector", value)?;
        }
        if let Some(value) = &self.target_average_value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetAverageValue", value)?;
        }
        if let Some(value) = &self.target_value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetValue", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExternalMetricSource {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.ExternalMetricSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one \"target\" type should be set.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "metricName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("metricName is the name of the metric in question.".to_owned()),
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
                    (
                        "targetAverageValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "targetValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "metricName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
