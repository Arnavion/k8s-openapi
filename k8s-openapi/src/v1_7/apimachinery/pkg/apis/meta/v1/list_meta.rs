// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta

/// ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListMeta {
    /// String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// SelfLink is a URL representing this object. Populated by the system. Read-only.
    pub self_link: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for ListMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource_version,
            Key_self_link,
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
                            "resourceVersion" => Field::Key_resource_version,
                            "selfLink" => Field::Key_self_link,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListMeta;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ListMeta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_resource_version: Option<String> = None;
                let mut value_self_link: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource_version => value_resource_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_self_link => value_self_link = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ListMeta {
                    resource_version: value_resource_version,
                    self_link: value_self_link,
                })
            }
        }

        deserializer.deserialize_struct(
            "ListMeta",
            &[
                "resourceVersion",
                "selfLink",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ListMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ListMeta",
            0 +
            (if self.resource_version.is_some() { 1 } else { 0 }) +
            (if self.self_link.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.resource_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.self_link {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selfLink", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
