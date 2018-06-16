// Generated from definition io.k8s.api.core.v1.NamespaceSpec

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    pub finalizers: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for NamespaceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_finalizers,
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
                            "finalizers" => Field::Key_finalizers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NamespaceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_finalizers: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_finalizers => value_finalizers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamespaceSpec {
                    finalizers: value_finalizers,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamespaceSpec",
            &[
                "finalizers",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NamespaceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamespaceSpec",
            0 +
            self.finalizers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.finalizers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
