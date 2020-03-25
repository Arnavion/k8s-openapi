// Generated from definition io.k8s.api.policy.v1beta1.RunAsGroupStrategyOptions

/// RunAsGroupStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunAsGroupStrategyOptions {
    /// ranges are the allowed ranges of gids that may be used. If you would like to force a single gid then supply a single range with the same start and end. Required for MustRunAs.
    pub ranges: Option<Vec<crate::api::policy::v1beta1::IDRange>>,

    /// rule is the strategy that will dictate the allowable RunAsGroup values that may be set.
    pub rule: String,
}

impl<'de> serde::Deserialize<'de> for RunAsGroupStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ranges,
            Key_rule,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunAsGroupStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RunAsGroupStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ranges: Option<Vec<crate::api::policy::v1beta1::IDRange>> = None;
                let mut value_rule: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ranges => value_ranges = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rule => value_rule = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RunAsGroupStrategyOptions {
                    ranges: value_ranges,
                    rule: value_rule.ok_or_else(|| serde::de::Error::missing_field("rule"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RunAsGroupStrategyOptions",
            &[
                "ranges",
                "rule",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RunAsGroupStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RunAsGroupStrategyOptions",
            1 +
            self.ranges.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ranges {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "rule", &self.rule)?;
        serde::ser::SerializeStruct::end(state)
    }
}
