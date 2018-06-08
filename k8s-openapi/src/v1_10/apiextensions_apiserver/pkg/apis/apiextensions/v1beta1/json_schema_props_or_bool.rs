// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.
#[derive(Debug, Default)]
pub struct JSONSchemaPropsOrBool {
    pub allows: bool,

    pub schema: Box<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>,
}

impl<'de> ::serde::Deserialize<'de> for JSONSchemaPropsOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allows,
            Key_schema,
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
                            "Allows" => Field::Key_allows,
                            "Schema" => Field::Key_schema,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrBool;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct JSONSchemaPropsOrBool")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_allows: Option<bool> = None;
                let mut value_schema: Option<Box<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allows => value_allows = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_schema => value_schema = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSONSchemaPropsOrBool {
                    allows: value_allows.ok_or_else(|| ::serde::de::Error::missing_field("Allows"))?,
                    schema: value_schema.ok_or_else(|| ::serde::de::Error::missing_field("Schema"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSONSchemaPropsOrBool",
            &[
                "Allows",
                "Schema",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for JSONSchemaPropsOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSONSchemaPropsOrBool",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Allows", &self.allows)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Schema", &self.schema)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
