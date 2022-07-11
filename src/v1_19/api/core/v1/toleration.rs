// Generated from definition io.k8s.api.core.v1.Toleration

/// The pod this Toleration is attached to tolerates any taint that matches the triple \<key,value,effect\> using the matching operator \<operator\>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Toleration {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: Option<String>,

    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    pub key: Option<String>,

    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    pub operator: Option<String>,

    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    pub toleration_seconds: Option<i64>,

    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    pub value: Option<String>,

}

#[cfg(feature = "dsl")]
impl Toleration  {
    /// Set [`Self::effect`]
    pub  fn effect_set(&mut self, effect: impl Into<Option<String>>) -> &mut Self {
        self.effect = effect.into(); self
    }

    pub  fn effect(&mut self) -> &mut String {
        if self.effect.is_none() { self.effect = Some(Default::default()) }
        self.effect.as_mut().unwrap()
    }

    /// Modify [`Self::effect`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn effect_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.effect.is_none() { self.effect = Some(Default::default()) };
        func(self.effect.as_mut().unwrap()); self
    }


    /// Set [`Self::key`]
    pub  fn key_set(&mut self, key: impl Into<Option<String>>) -> &mut Self {
        self.key = key.into(); self
    }

    pub  fn key(&mut self) -> &mut String {
        if self.key.is_none() { self.key = Some(Default::default()) }
        self.key.as_mut().unwrap()
    }

    /// Modify [`Self::key`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn key_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.key.is_none() { self.key = Some(Default::default()) };
        func(self.key.as_mut().unwrap()); self
    }


    /// Set [`Self::operator`]
    pub  fn operator_set(&mut self, operator: impl Into<Option<String>>) -> &mut Self {
        self.operator = operator.into(); self
    }

    pub  fn operator(&mut self) -> &mut String {
        if self.operator.is_none() { self.operator = Some(Default::default()) }
        self.operator.as_mut().unwrap()
    }

    /// Modify [`Self::operator`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn operator_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.operator.is_none() { self.operator = Some(Default::default()) };
        func(self.operator.as_mut().unwrap()); self
    }


    /// Set [`Self::toleration_seconds`]
    pub  fn toleration_seconds_set(&mut self, toleration_seconds: impl Into<Option<i64>>) -> &mut Self {
        self.toleration_seconds = toleration_seconds.into(); self
    }

    pub  fn toleration_seconds(&mut self) -> &mut i64 {
        if self.toleration_seconds.is_none() { self.toleration_seconds = Some(Default::default()) }
        self.toleration_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::toleration_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn toleration_seconds_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.toleration_seconds.is_none() { self.toleration_seconds = Some(Default::default()) };
        func(self.toleration_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::value`]
    pub  fn value_set(&mut self, value: impl Into<Option<String>>) -> &mut Self {
        self.value = value.into(); self
    }

    pub  fn value(&mut self) -> &mut String {
        if self.value.is_none() { self.value = Some(Default::default()) }
        self.value.as_mut().unwrap()
    }

    /// Modify [`Self::value`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn value_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.value.is_none() { self.value = Some(Default::default()) };
        func(self.value.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Toleration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_operator,
            Key_toleration_seconds,
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
                            "effect" => Field::Key_effect,
                            "key" => Field::Key_key,
                            "operator" => Field::Key_operator,
                            "tolerationSeconds" => Field::Key_toleration_seconds,
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
            type Value = Toleration;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Toleration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_effect: Option<String> = None;
                let mut value_key: Option<String> = None;
                let mut value_operator: Option<String> = None;
                let mut value_toleration_seconds: Option<i64> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_toleration_seconds => value_toleration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Toleration {
                    effect: value_effect,
                    key: value_key,
                    operator: value_operator,
                    toleration_seconds: value_toleration_seconds,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "Toleration",
            &[
                "effect",
                "key",
                "operator",
                "tolerationSeconds",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Toleration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Toleration",
            self.effect.as_ref().map_or(0, |_| 1) +
            self.key.as_ref().map_or(0, |_| 1) +
            self.operator.as_ref().map_or(0, |_| 1) +
            self.toleration_seconds.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.effect {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", value)?;
        }
        if let Some(value) = &self.key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        if let Some(value) = &self.operator {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", value)?;
        }
        if let Some(value) = &self.toleration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerationSeconds", value)?;
        }
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Toleration {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Toleration".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "effect".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operator".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tolerationSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "value".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
