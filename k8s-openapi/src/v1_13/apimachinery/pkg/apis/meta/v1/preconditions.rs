// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Preconditions

/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preconditions {
    /// Specifies the target UID.
    pub uid: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Preconditions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_uid,
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
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Preconditions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Preconditions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_uid: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_uid => value_uid = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Preconditions {
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "Preconditions",
            &[
                "uid",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Preconditions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Preconditions",
            0 +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.uid {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
