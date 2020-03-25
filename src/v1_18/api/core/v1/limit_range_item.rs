// Generated from definition io.k8s.api.core.v1.LimitRangeItem

/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitRangeItem {
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    pub default: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    pub default_request: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Max usage constraints on this kind by resource name.
    pub max: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    pub max_limit_request_ratio: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Min usage constraints on this kind by resource name.
    pub min: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Type of resource that this limit applies to.
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for LimitRangeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LimitRangeItem;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LimitRangeItem")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_default: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_default_request: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_max: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_max_limit_request_ratio: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_min: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default => value_default = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_request => value_default_request = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max => value_max = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_limit_request_ratio => value_max_limit_request_ratio = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min => value_min = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitRangeItem {
                    default: value_default,
                    default_request: value_default_request,
                    max: value_max,
                    max_limit_request_ratio: value_max_limit_request_ratio,
                    min: value_min,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
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

impl serde::Serialize for LimitRangeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitRangeItem",
            1 +
            self.default.as_ref().map_or(0, |_| 1) +
            self.default_request.as_ref().map_or(0, |_| 1) +
            self.max.as_ref().map_or(0, |_| 1) +
            self.max_limit_request_ratio.as_ref().map_or(0, |_| 1) +
            self.min.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default {
            serde::ser::SerializeStruct::serialize_field(&mut state, "default", value)?;
        }
        if let Some(value) = &self.default_request {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultRequest", value)?;
        }
        if let Some(value) = &self.max {
            serde::ser::SerializeStruct::serialize_field(&mut state, "max", value)?;
        }
        if let Some(value) = &self.max_limit_request_ratio {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxLimitRequestRatio", value)?;
        }
        if let Some(value) = &self.min {
            serde::ser::SerializeStruct::serialize_field(&mut state, "min", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
