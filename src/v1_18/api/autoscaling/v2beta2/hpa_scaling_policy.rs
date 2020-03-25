// Generated from definition io.k8s.api.autoscaling.v2beta2.HPAScalingPolicy

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HPAScalingPolicy {
    /// PeriodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    pub period_seconds: i32,

    /// Type is used to specify the scaling policy.
    pub type_: String,

    /// Value contains the amount of change which is permitted by the policy. It must be greater than zero
    pub value: i32,
}

impl<'de> serde::Deserialize<'de> for HPAScalingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_period_seconds,
            Key_type_,
            Key_value,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HPAScalingPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HPAScalingPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_period_seconds: Option<i32> = None;
                let mut value_type_: Option<String> = None;
                let mut value_value: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_period_seconds => value_period_seconds = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_value => value_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HPAScalingPolicy {
                    period_seconds: value_period_seconds.ok_or_else(|| serde::de::Error::missing_field("periodSeconds"))?,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    value: value_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
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

impl serde::Serialize for HPAScalingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingPolicy",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "periodSeconds", &self.period_seconds)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        serde::ser::SerializeStruct::end(state)
    }
}
