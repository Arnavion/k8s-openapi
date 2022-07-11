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

#[cfg(feature = "dsl")]
impl LimitRangeItem  {
    /// Set [`Self::default`]
    pub  fn default_set(&mut self, default: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.default = default.into(); self
    }

    pub  fn default(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.default.is_none() { self.default = Some(Default::default()) }
        self.default.as_mut().unwrap()
    }

    /// Modify [`Self::default`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.default.is_none() { self.default = Some(Default::default()) };
        func(self.default.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::default`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn default_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.default.is_none() {
            self.default = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.default.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::default`]
    pub  fn default_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.default.is_none() { self.default = Some(std::collections::BTreeMap::new()); }
         let default = &mut self.default.as_mut().unwrap();
         for (name, value) in other.borrow() {
             default.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::default_request`]
    pub  fn default_request_set(&mut self, default_request: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.default_request = default_request.into(); self
    }

    pub  fn default_request(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.default_request.is_none() { self.default_request = Some(Default::default()) }
        self.default_request.as_mut().unwrap()
    }

    /// Modify [`Self::default_request`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_request_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.default_request.is_none() { self.default_request = Some(Default::default()) };
        func(self.default_request.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::default_request`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn default_request_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.default_request.is_none() {
            self.default_request = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.default_request.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::default_request`]
    pub  fn default_request_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.default_request.is_none() { self.default_request = Some(std::collections::BTreeMap::new()); }
         let default_request = &mut self.default_request.as_mut().unwrap();
         for (name, value) in other.borrow() {
             default_request.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::max`]
    pub  fn max_set(&mut self, max: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.max = max.into(); self
    }

    pub  fn max(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.max.is_none() { self.max = Some(Default::default()) }
        self.max.as_mut().unwrap()
    }

    /// Modify [`Self::max`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn max_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.max.is_none() { self.max = Some(Default::default()) };
        func(self.max.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::max`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn max_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.max.is_none() {
            self.max = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.max.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::max`]
    pub  fn max_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.max.is_none() { self.max = Some(std::collections::BTreeMap::new()); }
         let max = &mut self.max.as_mut().unwrap();
         for (name, value) in other.borrow() {
             max.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::max_limit_request_ratio`]
    pub  fn max_limit_request_ratio_set(&mut self, max_limit_request_ratio: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.max_limit_request_ratio = max_limit_request_ratio.into(); self
    }

    pub  fn max_limit_request_ratio(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.max_limit_request_ratio.is_none() { self.max_limit_request_ratio = Some(Default::default()) }
        self.max_limit_request_ratio.as_mut().unwrap()
    }

    /// Modify [`Self::max_limit_request_ratio`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn max_limit_request_ratio_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.max_limit_request_ratio.is_none() { self.max_limit_request_ratio = Some(Default::default()) };
        func(self.max_limit_request_ratio.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::max_limit_request_ratio`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn max_limit_request_ratio_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.max_limit_request_ratio.is_none() {
            self.max_limit_request_ratio = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.max_limit_request_ratio.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::max_limit_request_ratio`]
    pub  fn max_limit_request_ratio_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.max_limit_request_ratio.is_none() { self.max_limit_request_ratio = Some(std::collections::BTreeMap::new()); }
         let max_limit_request_ratio = &mut self.max_limit_request_ratio.as_mut().unwrap();
         for (name, value) in other.borrow() {
             max_limit_request_ratio.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::min`]
    pub  fn min_set(&mut self, min: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.min = min.into(); self
    }

    pub  fn min(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.min.is_none() { self.min = Some(Default::default()) }
        self.min.as_mut().unwrap()
    }

    /// Modify [`Self::min`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.min.is_none() { self.min = Some(Default::default()) };
        func(self.min.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::min`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn min_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.min.is_none() {
            self.min = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.min.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::min`]
    pub  fn min_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.min.is_none() { self.min = Some(std::collections::BTreeMap::new()); }
         let min = &mut self.min.as_mut().unwrap();
         for (name, value) in other.borrow() {
             min.insert(name.to_owned(), value.to_owned());
         }
         self
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
