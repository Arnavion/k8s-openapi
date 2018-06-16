// Generated from definition io.k8s.api.authorization.v1.SelfSubjectRulesReviewSpec

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectRulesReviewSpec {
    /// Namespace to evaluate rules for. Required.
    pub namespace: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for SelfSubjectRulesReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespace,
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
                            "namespace" => Field::Key_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SelfSubjectRulesReviewSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct SelfSubjectRulesReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_namespace: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespace => value_namespace = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectRulesReviewSpec {
                    namespace: value_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectRulesReviewSpec",
            &[
                "namespace",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for SelfSubjectRulesReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectRulesReviewSpec",
            0 +
            self.namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.namespace {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
