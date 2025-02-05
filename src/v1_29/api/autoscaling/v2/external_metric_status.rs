// Generated from definition io.k8s.api.autoscaling.v2.ExternalMetricStatus

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricStatus {
    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2::MetricValueStatus,

    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2::MetricIdentifier,
}

impl crate::DeepMerge for ExternalMetricStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.current, other.current);
        crate::DeepMerge::merge_from(&mut self.metric, other.metric);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ExternalMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current,
            Key_metric,
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
                            "current" => Field::Key_current,
                            "metric" => Field::Key_metric,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ExternalMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_current: Option<crate::api::autoscaling::v2::MetricValueStatus> = None;
                let mut value_metric: Option<crate::api::autoscaling::v2::MetricIdentifier> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current => value_current = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric => value_metric = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricStatus {
                    current: value_current.unwrap_or_default(),
                    metric: value_metric.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricStatus",
            &[
                "current",
                "metric",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricStatus",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExternalMetricStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.ExternalMetricStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "current".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::MetricValueStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("current contains the current value for the given metric".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "metric".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::MetricIdentifier>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("metric identifies the target metric by name and selector".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "current".into(),
                    "metric".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
