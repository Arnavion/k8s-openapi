// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.ExternalDocumentation

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalDocumentation {
    pub description: Option<std::string::String>,

    pub url: Option<std::string::String>,
}

impl crate::DeepMerge for ExternalDocumentation {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.description, other.description);
        crate::DeepMerge::merge_from(&mut self.url, other.url);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ExternalDocumentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_description,
            Key_url,
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
                            "description" => Field::Key_description,
                            "url" => Field::Key_url,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalDocumentation;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ExternalDocumentation")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_description: Option<std::string::String> = None;
                let mut value_url: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_description => value_description = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_url => value_url = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalDocumentation {
                    description: value_description,
                    url: value_url,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalDocumentation",
            &[
                "description",
                "url",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExternalDocumentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalDocumentation",
            self.description.as_ref().map_or(0, |_| 1) +
            self.url.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.description {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.url {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "url", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExternalDocumentation {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.ExternalDocumentation".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ExternalDocumentation allows referencing an external resource for extended documentation.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "description".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "url".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
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
