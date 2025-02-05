// Generated from definition io.k8s.api.batch.v1.SuccessPolicyRule

/// SuccessPolicyRule describes rule for declaring a Job as succeeded. Each rule must have at least one of the "succeededIndexes" or "succeededCount" specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SuccessPolicyRule {
    /// succeededCount specifies the minimal required size of the actual set of the succeeded indexes for the Job. When succeededCount is used along with succeededIndexes, the check is constrained only to the set of indexes specified by succeededIndexes. For example, given that succeededIndexes is "1-4", succeededCount is "3", and completed indexes are "1", "3", and "5", the Job isn't declared as succeeded because only "1" and "3" indexes are considered in that rules. When this field is null, this doesn't default to any value and is never evaluated at any time. When specified it needs to be a positive integer.
    pub succeeded_count: Option<i32>,

    /// succeededIndexes specifies the set of indexes which need to be contained in the actual set of the succeeded indexes for the Job. The list of indexes must be within 0 to ".spec.completions-1" and must not contain duplicates. At least one element is required. The indexes are represented as intervals separated by commas. The intervals can be a decimal integer or a pair of decimal integers separated by a hyphen. The number are listed in represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7". When this field is null, this field doesn't default to any value and is never evaluated at any time.
    pub succeeded_indexes: Option<std::string::String>,
}

impl crate::DeepMerge for SuccessPolicyRule {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.succeeded_count, other.succeeded_count);
        crate::DeepMerge::merge_from(&mut self.succeeded_indexes, other.succeeded_indexes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SuccessPolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_succeeded_count,
            Key_succeeded_indexes,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "succeededCount" => Field::Key_succeeded_count,
                            "succeededIndexes" => Field::Key_succeeded_indexes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SuccessPolicyRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SuccessPolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_succeeded_count: Option<i32> = None;
                let mut value_succeeded_indexes: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_succeeded_count => value_succeeded_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_succeeded_indexes => value_succeeded_indexes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SuccessPolicyRule {
                    succeeded_count: value_succeeded_count,
                    succeeded_indexes: value_succeeded_indexes,
                })
            }
        }

        deserializer.deserialize_struct(
            "SuccessPolicyRule",
            &[
                "succeededCount",
                "succeededIndexes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SuccessPolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SuccessPolicyRule",
            self.succeeded_count.as_ref().map_or(0, |_| 1) +
            self.succeeded_indexes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.succeeded_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "succeededCount", value)?;
        }
        if let Some(value) = &self.succeeded_indexes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "succeededIndexes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SuccessPolicyRule {
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.SuccessPolicyRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SuccessPolicyRule describes rule for declaring a Job as succeeded. Each rule must have at least one of the \"succeededIndexes\" or \"succeededCount\" specified.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "succeededCount".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("succeededCount specifies the minimal required size of the actual set of the succeeded indexes for the Job. When succeededCount is used along with succeededIndexes, the check is constrained only to the set of indexes specified by succeededIndexes. For example, given that succeededIndexes is \"1-4\", succeededCount is \"3\", and completed indexes are \"1\", \"3\", and \"5\", the Job isn't declared as succeeded because only \"1\" and \"3\" indexes are considered in that rules. When this field is null, this doesn't default to any value and is never evaluated at any time. When specified it needs to be a positive integer.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "succeededIndexes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("succeededIndexes specifies the set of indexes which need to be contained in the actual set of the succeeded indexes for the Job. The list of indexes must be within 0 to \".spec.completions-1\" and must not contain duplicates. At least one element is required. The indexes are represented as intervals separated by commas. The intervals can be a decimal integer or a pair of decimal integers separated by a hyphen. The number are listed in represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as \"1,3-5,7\". When this field is null, this field doesn't default to any value and is never evaluated at any time.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
