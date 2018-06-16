// Generated from definition io.k8s.api.authorization.v1beta1.NonResourceAttributes

/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request
    pub path: Option<String>,

    /// Verb is the standard HTTP verb
    pub verb: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for NonResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path,
            Key_verb,
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
                            "path" => Field::Key_path,
                            "verb" => Field::Key_verb,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NonResourceAttributes;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NonResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_path: Option<String> = None;
                let mut value_verb: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path => value_path = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verb => value_verb = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourceAttributes {
                    path: value_path,
                    verb: value_verb,
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourceAttributes",
            &[
                "path",
                "verb",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NonResourceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceAttributes",
            0 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.path {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.verb {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "verb", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
