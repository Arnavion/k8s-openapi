// Generated from definition io.k8s.api.extensions.v1beta1.RunAsUserStrategyOptions

/// RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy. Deprecated: use RunAsUserStrategyOptions from policy API Group instead.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunAsUserStrategyOptions {
    /// ranges are the allowed ranges of uids that may be used. If you would like to force a single uid then supply a single range with the same start and end. Required for MustRunAs.
    pub ranges: Option<Vec<::v1_12::api::extensions::v1beta1::IDRange>>,

    /// rule is the strategy that will dictate the allowable RunAsUser values that may be set.
    pub rule: String,
}

impl<'de> ::serde::Deserialize<'de> for RunAsUserStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ranges,
            Key_rule,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RunAsUserStrategyOptions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct RunAsUserStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_ranges: Option<Vec<::v1_12::api::extensions::v1beta1::IDRange>> = None;
                let mut value_rule: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ranges => value_ranges = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rule => value_rule = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RunAsUserStrategyOptions {
                    ranges: value_ranges,
                    rule: value_rule.ok_or_else(|| ::serde::de::Error::missing_field("rule"))?,
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

impl ::serde::Serialize for RunAsUserStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RunAsUserStrategyOptions",
            0 +
            self.ranges.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.ranges {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", &self.rule)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
