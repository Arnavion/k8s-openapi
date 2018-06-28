// Generated from definition io.k8s.api.rbac.v1.Subject

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subject {
    /// APIGroup holds the API group of the referenced subject. Defaults to "" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    pub api_group: Option<String>,

    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,

    /// Name of the object being referenced.
    pub name: String,

    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    pub namespace: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_group,
            Key_kind,
            Key_name,
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
                            "apiGroup" => Field::Key_api_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
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
            type Value = Subject;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Subject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_group => value_api_group = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Subject {
                    api_group: value_api_group,
                    kind: value_kind.ok_or_else(|| ::serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "Subject",
            &[
                "apiGroup",
                "kind",
                "name",
                "namespace",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Subject",
            0 +
            self.api_group.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_group {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroup", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
