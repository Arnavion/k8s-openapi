// Generated from definition io.k8s.api.autoscaling.v2.HPAScalingPolicy

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HPAScalingPolicy {
    /// periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    pub period_seconds: i32,

    /// type is used to specify the scaling policy.
    pub type_: std::string::String,

    /// value contains the amount of change which is permitted by the policy. It must be greater than zero
    pub value: i32,
}

impl crate::DeepMerge for HPAScalingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.period_seconds, other.period_seconds);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HPAScalingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_period_seconds,
            Key_type_,
            Key_value,
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
                            "periodSeconds" => Field::Key_period_seconds,
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
            type Value = HPAScalingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HPAScalingPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_period_seconds: Option<i32> = None;
                let mut value_type_: Option<std::string::String> = None;
                let mut value_value: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_period_seconds => value_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HPAScalingPolicy {
                    period_seconds: value_period_seconds.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                    value: value_value.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "HPAScalingPolicy",
            &[
                "periodSeconds",
                "type",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HPAScalingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingPolicy",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "periodSeconds", &self.period_seconds)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HPAScalingPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.HPAScalingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("HPAScalingPolicy is a single policy which must hold true for a specified past interval.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "periodSeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("type is used to specify the scaling policy.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "value".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("value contains the amount of change which is permitted by the policy. It must be greater than zero".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "periodSeconds".into(),
                    "type".into(),
                    "value".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
