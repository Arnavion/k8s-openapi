// Generated from definition io.k8s.api.extensions.v1beta1.ScaleSpec

/// describes the attributes of a scale subresource
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScaleSpec {
    /// desired number of instances for the scaled object.
    pub replicas: Option<i32>,
}

impl<'de> ::serde::Deserialize<'de> for ScaleSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_replicas,
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
                            "replicas" => Field::Key_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScaleSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ScaleSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_replicas: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_replicas => value_replicas = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleSpec {
                    replicas: value_replicas,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScaleSpec",
            &[
                "replicas",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ScaleSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleSpec",
            0 +
            self.replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.replicas {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
