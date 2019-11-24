// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.ExternalDocumentation

/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalDocumentation {
    pub description: Option<String>,

    pub url: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ExternalDocumentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_description,
            Key_url,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExternalDocumentation;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ExternalDocumentation")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_description: Option<String> = None;
                let mut value_url: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_description => value_description = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_url => value_url = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for ExternalDocumentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalDocumentation",
            self.description.as_ref().map_or(0, |_| 1) +
            self.url.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.description {
            serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.url {
            serde::ser::SerializeStruct::serialize_field(&mut state, "url", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
