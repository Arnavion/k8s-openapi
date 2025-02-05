// Generated from definition io.k8s.api.batch.v1.SuccessPolicy

/// SuccessPolicy describes when a Job can be declared as succeeded based on the success of some indexes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SuccessPolicy {
    /// rules represents the list of alternative rules for the declaring the Jobs as successful before `.status.succeeded \>= .spec.completions`. Once any of the rules are met, the "SucceededCriteriaMet" condition is added, and the lingering pods are removed. The terminal state for such a Job has the "Complete" condition. Additionally, these rules are evaluated in order; Once the Job meets one of the rules, other rules are ignored. At most 20 elements are allowed.
    pub rules: std::vec::Vec<crate::api::batch::v1::SuccessPolicyRule>,
}

impl crate::DeepMerge for SuccessPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.rules, other.rules);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SuccessPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rules,
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
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SuccessPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SuccessPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_rules: Option<std::vec::Vec<crate::api::batch::v1::SuccessPolicyRule>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SuccessPolicy {
                    rules: value_rules.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "SuccessPolicy",
            &[
                "rules",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SuccessPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SuccessPolicy",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SuccessPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.SuccessPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SuccessPolicy describes when a Job can be declared as succeeded based on the success of some indexes.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "rules".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("rules represents the list of alternative rules for the declaring the Jobs as successful before `.status.succeeded >= .spec.completions`. Once any of the rules are met, the \"SucceededCriteriaMet\" condition is added, and the lingering pods are removed. The terminal state for such a Job has the \"Complete\" condition. Additionally, these rules are evaluated in order; Once the Job meets one of the rules, other rules are ignored. At most 20 elements are allowed.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::batch::v1::SuccessPolicyRule>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "rules".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
