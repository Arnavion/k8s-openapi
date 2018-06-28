// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionVersion

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionVersion {
    /// Name is the version name, e.g. “v1”, “v2beta1”, etc.
    pub name: String,

    /// Served is a flag enabling/disabling this version from being served via REST APIs
    pub served: bool,

    /// Storage flags the version as storage version. There must be exactly one flagged as storage version.
    pub storage: bool,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceDefinitionVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_served,
            Key_storage,
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
                            "served" => Field::Key_served,
                            "storage" => Field::Key_storage,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionVersion;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceDefinitionVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_served: Option<bool> = None;
                let mut value_storage: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_served => value_served = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_storage => value_storage = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionVersion {
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    served: value_served.ok_or_else(|| ::serde::de::Error::missing_field("served"))?,
                    storage: value_storage.ok_or_else(|| ::serde::de::Error::missing_field("storage"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionVersion",
            &[
                "name",
                "served",
                "storage",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CustomResourceDefinitionVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionVersion",
            0 +
            1 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "served", &self.served)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "storage", &self.storage)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
