// Generated from definition io.k8s.api.autoscaling.v2beta1.PodsMetricStatus

/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodsMetricStatus {
    /// currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    pub current_average_value: crate::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of the metric in question
    pub metric_name: String,

    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

}

#[cfg(feature = "dsl")]
impl PodsMetricStatus  {
    /// Set [`Self::current_average_value`]
    pub  fn current_average_value_set(&mut self, current_average_value: impl Into<crate::apimachinery::pkg::api::resource::Quantity>) -> &mut Self {
        self.current_average_value = current_average_value.into(); self
    }

    pub  fn current_average_value(&mut self) -> &mut crate::apimachinery::pkg::api::resource::Quantity {
        &mut self.current_average_value
    }

    /// Modify [`Self::current_average_value`] with a `func`
    pub  fn current_average_value_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        func(&mut self.current_average_value); self
    }


    /// Set [`Self::metric_name`]
    pub  fn metric_name_set(&mut self, metric_name: impl Into<String>) -> &mut Self {
        self.metric_name = metric_name.into(); self
    }

    pub  fn metric_name(&mut self) -> &mut String {
        &mut self.metric_name
    }

    /// Modify [`Self::metric_name`] with a `func`
    pub  fn metric_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.metric_name); self
    }


    /// Set [`Self::selector`]
    pub  fn selector_set(&mut self, selector: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>) -> &mut Self {
        self.selector = selector.into(); self
    }

    pub  fn selector(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector {
        if self.selector.is_none() { self.selector = Some(Default::default()) }
        self.selector.as_mut().unwrap()
    }

    /// Modify [`Self::selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn selector_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        if self.selector.is_none() { self.selector = Some(Default::default()) };
        func(self.selector.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PodsMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_average_value,
            Key_metric_name,
            Key_selector,
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
                            "metricName" => Field::Key_metric_name,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodsMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodsMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_average_value => value_current_average_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metric_name => value_metric_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodsMetricStatus {
                    current_average_value: value_current_average_value.unwrap_or_default(),
                    metric_name: value_metric_name.unwrap_or_default(),
                    selector: value_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodsMetricStatus",
            &[
                "currentAverageValue",
                "metricName",
                "selector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodsMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodsMetricStatus",
            2 +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", &self.current_average_value)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodsMetricStatus {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta1.PodsMetricStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).".to_owned()),
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
                                description: Some("currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
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
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("selector is the string-encoded form of a standard kubernetes label selector for the given metric When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "currentAverageValue".to_owned(),
                    "metricName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
