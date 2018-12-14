// Generated from definition io.k8s.api.admissionregistration.v1beta1.ServiceReference

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceReference {
    /// `name` is the name of the service. Required
    pub name: String,

    /// `namespace` is the namespace of the service. Required
    pub namespace: String,

    /// `path` is an optional URL path which will be sent in any request to this service.
    pub path: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for ServiceReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_namespace,
            Key_path,
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
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceReference;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ServiceReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceReference {
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace.ok_or_else(|| ::serde::de::Error::missing_field("namespace"))?,
                    path: value_path,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceReference",
            &[
                "name",
                "namespace",
                "path",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ServiceReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceReference",
            0 +
            1 +
            1 +
            self.path.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        if let Some(value) = &self.path {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
