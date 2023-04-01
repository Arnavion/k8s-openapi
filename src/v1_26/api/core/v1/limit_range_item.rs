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

impl crate::DeepMerge for LimitRangeItem {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.default, other.default, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.default_request, other.default_request, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.max, other.max, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.max_limit_request_ratio, other.max_limit_request_ratio, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.min, other.min, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
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
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitRangeItem {
                    default: value_default,
                    default_request: value_default_request,
                    max: value_max,
                    max_limit_request_ratio: value_max_limit_request_ratio,
                    min: value_min,
                    type_: value_type_.unwrap_or_default(),
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
            self.default.as_ref().map_or(0, |_| 1) +
            self.default_request.as_ref().map_or(0, |_| 1) +
            self.max.as_ref().map_or(0, |_| 1) +
            self.max_limit_request_ratio.as_ref().map_or(0, |_| 1) +
            self.min.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "default", value)?;
        }
        if let Some(value) = &self.default_request {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultRequest", value)?;
        }
        if let Some(value) = &self.max {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "max", value)?;
        }
        if let Some(value) = &self.max_limit_request_ratio {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxLimitRequestRatio", value)?;
        }
        if let Some(value) = &self.min {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "min", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LimitRangeItem {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.LimitRangeItem".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LimitRangeItem defines a min/max usage limit for any resource that matches on kind.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "default".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Default resource requirement limit value by resource name if resource limit is omitted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "defaultRequest".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "max".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Max usage constraints on this kind by resource name.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxLimitRequestRatio".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "min".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Min usage constraints on this kind by resource name.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type of resource that this limit applies to.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
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
