// Generated from definition io.k8s.api.extensions.v1beta1.SupplementalGroupsStrategyOptions

/// SupplementalGroupsStrategyOptions defines the strategy type and options used to create the strategy. Deprecated: use SupplementalGroupsStrategyOptions from policy API Group instead.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SupplementalGroupsStrategyOptions {
    /// ranges are the allowed ranges of supplemental groups.  If you would like to force a single supplemental group then supply a single range with the same start and end. Required for MustRunAs.
    pub ranges: Option<Vec<::v1_12::api::extensions::v1beta1::IDRange>>,

    /// rule is the strategy that will dictate what supplemental groups is used in the SecurityContext.
    pub rule: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for SupplementalGroupsStrategyOptions {
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
            type Value = SupplementalGroupsStrategyOptions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct SupplementalGroupsStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_ranges: Option<Vec<::v1_12::api::extensions::v1beta1::IDRange>> = None;
                let mut value_rule: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ranges => value_ranges = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rule => value_rule = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SupplementalGroupsStrategyOptions {
                    ranges: value_ranges,
                    rule: value_rule,
                })
            }
        }

        deserializer.deserialize_struct(
            "SupplementalGroupsStrategyOptions",
            &[
                "ranges",
                "rule",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for SupplementalGroupsStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SupplementalGroupsStrategyOptions",
            0 +
            self.ranges.as_ref().map_or(0, |_| 1) +
            self.rule.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ranges {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ranges", value)?;
        }
        if let Some(value) = &self.rule {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
