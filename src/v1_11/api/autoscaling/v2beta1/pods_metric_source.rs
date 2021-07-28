// Generated from definition io.k8s.api.autoscaling.v2beta1.PodsMetricSource

/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodsMetricSource {
    /// metricName is the name of the metric in question
    pub metric_name: String,

    /// targetAverageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    pub target_average_value: crate::apimachinery::pkg::api::resource::Quantity,
}

impl<'de> crate::serde::Deserialize<'de> for PodsMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric_name,
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
                            "metricName" => Field::Key_metric_name,
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
            type Value = PodsMetricSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodsMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metric_name: Option<String> = None;
                let mut value_target_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric_name => value_metric_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target_average_value => value_target_average_value = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodsMetricSource {
                    metric_name: value_metric_name.ok_or_else(|| crate::serde::de::Error::missing_field("metricName"))?,
                    target_average_value: value_target_average_value.ok_or_else(|| crate::serde::de::Error::missing_field("targetAverageValue"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodsMetricSource",
            &[
                "metricName",
                "targetAverageValue",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodsMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodsMetricSource",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetAverageValue", &self.target_average_value)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodsMetricSource {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.PodsMetricSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: std::array::IntoIter::new([
                    (
                        "metricName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("metricName is the name of the metric in question".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetAverageValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetAverageValue is the target value of the average of the metric across all relevant pods (as a quantity)".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ]).collect(),
                required: std::array::IntoIter::new([
                    "metricName",
                    "targetAverageValue",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

