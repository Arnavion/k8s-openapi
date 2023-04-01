// Generated from definition io.k8s.api.policy.v1beta1.FSGroupStrategyOptions

/// FSGroupStrategyOptions defines the strategy type and options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FSGroupStrategyOptions {
    /// ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end. Required for MustRunAs.
    pub ranges: Option<Vec<crate::api::policy::v1beta1::IDRange>>,

    /// rule is the strategy that will dictate what FSGroup is used in the SecurityContext.
    pub rule: Option<String>,
}

impl crate::DeepMerge for FSGroupStrategyOptions {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.ranges, other.ranges);
        crate::DeepMerge::merge_from(&mut self.rule, other.rule);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FSGroupStrategyOptions {
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
            type Value = FSGroupStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FSGroupStrategyOptions")
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

                Ok(FSGroupStrategyOptions {
                    ranges: value_ranges,
                    rule: value_rule,
                })
            }
        }

        deserializer.deserialize_struct(
            "FSGroupStrategyOptions",
            &[
                "ranges",
                "rule",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FSGroupStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FSGroupStrategyOptions",
            self.ranges.as_ref().map_or(0, |_| 1) +
            self.rule.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ranges {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        if let Some(value) = &self.rule {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FSGroupStrategyOptions {
    fn schema_name() -> String {
        "io.k8s.api.policy.v1beta1.FSGroupStrategyOptions".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FSGroupStrategyOptions defines the strategy type and options used to create the strategy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ranges".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ranges are the allowed ranges of fs groups.  If you would like to force a single fs group then supply a single range with the same start and end. Required for MustRunAs.".to_owned()),
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
                                description: Some("rule is the strategy that will dictate what FSGroup is used in the SecurityContext.".to_owned()),
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
