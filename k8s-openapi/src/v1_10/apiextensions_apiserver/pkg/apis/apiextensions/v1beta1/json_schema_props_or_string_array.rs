// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrStringArray

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Debug, Default)]
pub struct JSONSchemaPropsOrStringArray {
    pub property: Vec<String>,

    pub schema: ::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps,
}

impl<'de> ::serde::Deserialize<'de> for JSONSchemaPropsOrStringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_property,
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
                            "Property" => Field::Key_property,
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
            type Value = JSONSchemaPropsOrStringArray;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct JSONSchemaPropsOrStringArray")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_property: Option<Vec<String>> = None;
                let mut value_schema: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_property => value_property = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_schema => value_schema = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSONSchemaPropsOrStringArray {
                    property: value_property.ok_or_else(|| ::serde::de::Error::missing_field("Property"))?,
                    schema: value_schema.ok_or_else(|| ::serde::de::Error::missing_field("Schema"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSONSchemaPropsOrStringArray",
            &[
                "Property",
                "Schema",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for JSONSchemaPropsOrStringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSONSchemaPropsOrStringArray",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Property", &self.property)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Schema", &self.schema)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
