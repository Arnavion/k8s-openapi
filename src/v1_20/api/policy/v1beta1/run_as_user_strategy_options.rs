// Generated from definition io.k8s.api.policy.v1beta1.RunAsUserStrategyOptions

/// RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunAsUserStrategyOptions {
    /// ranges are the allowed ranges of uids that may be used. If you would like to force a single uid then supply a single range with the same start and end. Required for MustRunAs.
    pub ranges: Option<Vec<crate::api::policy::v1beta1::IDRange>>,

    /// rule is the strategy that will dictate the allowable RunAsUser values that may be set.
    pub rule: String,

}

#[cfg(feature = "dsl")]
impl RunAsUserStrategyOptions  {
    /// Set [`Self::ranges`]
    pub  fn ranges_set(&mut self, ranges: impl Into<Option<Vec<crate::api::policy::v1beta1::IDRange>>>) -> &mut Self {
        self.ranges = ranges.into(); self
    }

    pub  fn ranges(&mut self) -> &mut Vec<crate::api::policy::v1beta1::IDRange> {
        if self.ranges.is_none() { self.ranges = Some(Default::default()) }
        self.ranges.as_mut().unwrap()
    }

    /// Modify [`Self::ranges`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ranges_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::policy::v1beta1::IDRange>)) -> &mut Self {
        if self.ranges.is_none() { self.ranges = Some(Default::default()) };
        func(self.ranges.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ranges`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ranges_push_with(&mut self, func: impl FnOnce(&mut crate::api::policy::v1beta1::IDRange)) -> &mut Self {
        if self.ranges.is_none() {
            self.ranges = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ranges.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ranges`]
    pub  fn ranges_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::policy::v1beta1::IDRange]>) -> &mut Self {
         if self.ranges.is_none() { self.ranges = Some(Vec::new()); }
         let ranges = &mut self.ranges.as_mut().unwrap();
         for item in other.borrow() {
             ranges.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::rule`]
    pub  fn rule_set(&mut self, rule: impl Into<String>) -> &mut Self {
        self.rule = rule.into(); self
    }

    pub  fn rule(&mut self) -> &mut String {
        &mut self.rule
    }

    /// Modify [`Self::rule`] with a `func`
    pub  fn rule_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.rule); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for RunAsUserStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ranges,
            Key_rule,
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
                            "ranges" => Field::Key_ranges,
                            "rule" => Field::Key_rule,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RunAsUserStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RunAsUserStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ranges: Option<Vec<crate::api::policy::v1beta1::IDRange>> = None;
                let mut value_rule: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ranges => value_ranges = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rule => value_rule = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RunAsUserStrategyOptions {
                    ranges: value_ranges,
                    rule: value_rule.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "RunAsUserStrategyOptions",
            &[
                "ranges",
                "rule",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RunAsUserStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RunAsUserStrategyOptions",
            1 +
            self.ranges.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ranges {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", &self.rule)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RunAsUserStrategyOptions {
    fn schema_name() -> String {
        "io.k8s.api.policy.v1beta1.RunAsUserStrategyOptions".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ranges".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ranges are the allowed ranges of uids that may be used. If you would like to force a single uid then supply a single range with the same start and end. Required for MustRunAs.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::policy::v1beta1::IDRange>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rule".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("rule is the strategy that will dictate the allowable RunAsUser values that may be set.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "rule".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
