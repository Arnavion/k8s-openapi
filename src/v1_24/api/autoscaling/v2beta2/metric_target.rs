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

#[cfg(feature = "dsl")]
impl MetricTarget  {
    /// Set [`Self::average_utilization`]
    pub  fn average_utilization_set(&mut self, average_utilization: impl Into<Option<i32>>) -> &mut Self {
        self.average_utilization = average_utilization.into(); self
    }

    pub  fn average_utilization(&mut self) -> &mut i32 {
        if self.average_utilization.is_none() { self.average_utilization = Some(Default::default()) }
        self.average_utilization.as_mut().unwrap()
    }

    /// Modify [`Self::average_utilization`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn average_utilization_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.average_utilization.is_none() { self.average_utilization = Some(Default::default()) };
        func(self.average_utilization.as_mut().unwrap()); self
    }


    /// Set [`Self::average_value`]
    pub  fn average_value_set(&mut self, average_value: impl Into<Option<crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
        self.average_value = average_value.into(); self
    }

    pub  fn average_value(&mut self) -> &mut crate::apimachinery::pkg::api::resource::Quantity {
        if self.average_value.is_none() { self.average_value = Some(Default::default()) }
        self.average_value.as_mut().unwrap()
    }

    /// Modify [`Self::average_value`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn average_value_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.average_value.is_none() { self.average_value = Some(Default::default()) };
        func(self.average_value.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<String>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        &mut self.type_
    }

    /// Modify [`Self::type_`] with a `func`
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.type_); self
    }


    /// Set [`Self::value`]
    pub  fn value_set(&mut self, value: impl Into<Option<crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
        self.value = value.into(); self
    }

    pub  fn value(&mut self) -> &mut crate::apimachinery::pkg::api::resource::Quantity {
        if self.value.is_none() { self.value = Some(Default::default()) }
        self.value.as_mut().unwrap()
    }

    /// Modify [`Self::value`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn value_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.value.is_none() { self.value = Some(Default::default()) };
        func(self.value.as_mut().unwrap()); self
    }


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
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricTarget {
                    average_utilization: value_average_utilization,
                    average_value: value_average_value,
                    type_: value_type_.unwrap_or_default(),
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MetricTarget {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta2.MetricTarget".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("MetricTarget defines the target value, average value, or average utilization of a specific metric".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "averageUtilization".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "averageValue".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("averageValue is the target value of the average of the metric across all relevant pods (as a quantity)".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("type represents whether the metric type is Utilization, Value, or AverageValue".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "value".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("value is the target value of the metric (as a quantity).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "type".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
