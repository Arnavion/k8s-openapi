// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ExpressionWarning

/// ExpressionWarning is a warning information that targets a specific expression.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpressionWarning {
    /// The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is "spec.validations\[0\].expression"
    pub field_ref: std::string::String,

    /// The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.
    pub warning: std::string::String,
}

impl crate::DeepMerge for ExpressionWarning {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.field_ref, other.field_ref);
        crate::DeepMerge::merge_from(&mut self.warning, other.warning);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ExpressionWarning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field_ref,
            Key_warning,
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
                            "fieldRef" => Field::Key_field_ref,
                            "warning" => Field::Key_warning,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExpressionWarning;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ExpressionWarning")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_field_ref: Option<std::string::String> = None;
                let mut value_warning: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field_ref => value_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_warning => value_warning = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExpressionWarning {
                    field_ref: value_field_ref.unwrap_or_default(),
                    warning: value_warning.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ExpressionWarning",
            &[
                "fieldRef",
                "warning",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExpressionWarning {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExpressionWarning",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldRef", &self.field_ref)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "warning", &self.warning)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExpressionWarning {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1alpha1.ExpressionWarning".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ExpressionWarning is a warning information that targets a specific expression.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fieldRef".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is \"spec.validations[0].expression\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "warning".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "fieldRef".into(),
                    "warning".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
