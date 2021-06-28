// Generated from definition io.k8s.api.core.v1.LimitRangeItem

/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitRangeItem {
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    pub default: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    pub default_request: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// Max usage constraints on this kind by resource name.
    pub max: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    pub max_limit_request_ratio: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// Min usage constraints on this kind by resource name.
    pub min: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// Type of resource that this limit applies to.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for LimitRangeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default,
            Key_default_request,
            Key_max,
            Key_max_limit_request_ratio,
            Key_min,
            Key_type_,
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
                            "default" => Field::Key_default,
                            "defaultRequest" => Field::Key_default_request,
                            "max" => Field::Key_max,
                            "maxLimitRequestRatio" => Field::Key_max_limit_request_ratio,
                            "min" => Field::Key_min,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LimitRangeItem;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LimitRangeItem")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_default_request: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_max: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_max_limit_request_ratio: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_min: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default => value_default = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_request => value_default_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max => value_max = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_limit_request_ratio => value_max_limit_request_ratio = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min => value_min = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitRangeItem {
                    default: value_default.unwrap_or_default(),
                    default_request: value_default_request.unwrap_or_default(),
                    max: value_max.unwrap_or_default(),
                    max_limit_request_ratio: value_max_limit_request_ratio.unwrap_or_default(),
                    min: value_min.unwrap_or_default(),
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "LimitRangeItem",
            &[
                "default",
                "defaultRequest",
                "max",
                "maxLimitRequestRatio",
                "min",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LimitRangeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitRangeItem",
            1 +
            usize::from(!self.default.is_empty()) +
            usize::from(!self.default_request.is_empty()) +
            usize::from(!self.max.is_empty()) +
            usize::from(!self.max_limit_request_ratio.is_empty()) +
            usize::from(!self.min.is_empty()),
        )?;
        if !self.default.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "default", &self.default)?;
        }
        if !self.default_request.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultRequest", &self.default_request)?;
        }
        if !self.max.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "max", &self.max)?;
        }
        if !self.max_limit_request_ratio.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxLimitRequestRatio", &self.max_limit_request_ratio)?;
        }
        if !self.min.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "min", &self.min)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for LimitRangeItem {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "LimitRangeItem defines a min/max usage limit for any resource that matches on kind.",
          "properties": {
            "default": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "Default resource requirement limit value by resource name if resource limit is omitted.",
              "type": "object"
            },
            "defaultRequest": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.",
              "type": "object"
            },
            "max": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "Max usage constraints on this kind by resource name.",
              "type": "object"
            },
            "maxLimitRequestRatio": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.",
              "type": "object"
            },
            "min": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "Min usage constraints on this kind by resource name.",
              "type": "object"
            },
            "type": {
              "description": "Type of resource that this limit applies to.",
              "type": "string"
            }
          },
          "required": [
            "type"
          ],
          "type": "object"
        })
    }
}
