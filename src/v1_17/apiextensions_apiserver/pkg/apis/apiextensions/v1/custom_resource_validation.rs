// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceValidation

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceValidation {
    /// openAPIV3Schema is the OpenAPI v3 schema to use for validation and pruning.
    pub open_api_v3_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>,
}

impl<'de> serde::Deserialize<'de> for CustomResourceValidation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_open_api_v3_schema,
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
                            "openAPIV3Schema" => Field::Key_open_api_v3_schema,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceValidation;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceValidation")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_open_api_v3_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_open_api_v3_schema => value_open_api_v3_schema = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceValidation {
                    open_api_v3_schema: value_open_api_v3_schema,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceValidation",
            &[
                "openAPIV3Schema",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomResourceValidation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceValidation",
            self.open_api_v3_schema.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.open_api_v3_schema {
            serde::ser::SerializeStruct::serialize_field(&mut state, "openAPIV3Schema", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
