// Generated from definition io.k8s.api.admissionregistration.v1beta1.TypeChecking

/// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypeChecking {
    /// The type checking warnings for each expression.
    pub expression_warnings: Option<std::vec::Vec<crate::api::admissionregistration::v1beta1::ExpressionWarning>>,
}

impl crate::DeepMerge for TypeChecking {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.expression_warnings, other.expression_warnings);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TypeChecking {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expression_warnings,
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
                            "expressionWarnings" => Field::Key_expression_warnings,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TypeChecking;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TypeChecking")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expression_warnings: Option<std::vec::Vec<crate::api::admissionregistration::v1beta1::ExpressionWarning>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expression_warnings => value_expression_warnings = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TypeChecking {
                    expression_warnings: value_expression_warnings,
                })
            }
        }

        deserializer.deserialize_struct(
            "TypeChecking",
            &[
                "expressionWarnings",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TypeChecking {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TypeChecking",
            self.expression_warnings.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.expression_warnings {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expressionWarnings", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TypeChecking {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1beta1.TypeChecking".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "expressionWarnings".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The type checking warnings for each expression.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1beta1::ExpressionWarning>()))),
                                ..Default::default()
                            })),
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
