// Generated from definition io.k8s.api.autoscaling.v2.ExternalMetricSource

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalMetricSource {
    /// metric identifies the target metric by name and selector
    pub metric: crate::api::autoscaling::v2::MetricIdentifier,

    /// target specifies the target value for the given metric
    pub target: crate::api::autoscaling::v2::MetricTarget,
}

impl crate::DeepMerge for ExternalMetricSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.metric, other.metric);
        crate::DeepMerge::merge_from(&mut self.target, other.target);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ExternalMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metric,
            Key_target,
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
                            "metric" => Field::Key_metric,
                            "target" => Field::Key_target,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ExternalMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metric: Option<crate::api::autoscaling::v2::MetricIdentifier> = None;
                let mut value_target: Option<crate::api::autoscaling::v2::MetricTarget> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metric => value_metric = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target => value_target = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalMetricSource {
                    metric: value_metric.unwrap_or_default(),
                    target: value_target.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalMetricSource",
            &[
                "metric",
                "target",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalMetricSource",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metric", &self.metric)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExternalMetricSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.ExternalMetricSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
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
                    (
                        "target".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::MetricTarget>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("target specifies the target value for the given metric".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "metric".into(),
                    "target".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
