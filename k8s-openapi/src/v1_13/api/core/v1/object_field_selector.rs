// Generated from definition io.k8s.api.core.v1.ObjectFieldSelector

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    pub api_version: Option<String>,

    /// Path of the field to select in the specified API version.
    pub field_path: String,
}

impl<'de> ::serde::Deserialize<'de> for ObjectFieldSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_field_path,
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
                            "apiVersion" => Field::Key_api_version,
                            "fieldPath" => Field::Key_field_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectFieldSelector;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ObjectFieldSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_field_path: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_field_path => value_field_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectFieldSelector {
                    api_version: value_api_version,
                    field_path: value_field_path.ok_or_else(|| ::serde::de::Error::missing_field("fieldPath"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectFieldSelector",
            &[
                "apiVersion",
                "fieldPath",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ObjectFieldSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectFieldSelector",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldPath", &self.field_path)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
