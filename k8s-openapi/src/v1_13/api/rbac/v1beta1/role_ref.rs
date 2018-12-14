// Generated from definition io.k8s.api.rbac.v1beta1.RoleRef

/// RoleRef contains information that points to the role being used
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced
    pub api_group: String,

    /// Kind is the type of resource being referenced
    pub kind: String,

    /// Name is the name of resource being referenced
    pub name: String,
}

impl<'de> ::serde::Deserialize<'de> for RoleRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_group,
            Key_kind,
            Key_name,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoleRef;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct RoleRef")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_group => value_api_group = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoleRef {
                    api_group: value_api_group.ok_or_else(|| ::serde::de::Error::missing_field("apiGroup"))?,
                    kind: value_kind.ok_or_else(|| ::serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoleRef",
            &[
                "apiGroup",
                "kind",
                "name",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for RoleRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoleRef",
            0 +
            1 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroup", &self.api_group)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
