// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSON

/// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, \[\]interface{}, map\[string\]interface{} and nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSON {
    pub raw: ::ByteString,
}

impl<'de> ::serde::Deserialize<'de> for JSON {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_raw,
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
                            "Raw" => Field::Key_raw,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JSON;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct JSON")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_raw: Option<::ByteString> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_raw => value_raw = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSON {
                    raw: value_raw.ok_or_else(|| ::serde::de::Error::missing_field("Raw"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSON",
            &[
                "Raw",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for JSON {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSON",
            0 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Raw", &self.raw)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
