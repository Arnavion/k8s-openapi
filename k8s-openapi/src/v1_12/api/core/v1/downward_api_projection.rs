// Generated from definition io.k8s.api.core.v1.DownwardAPIProjection

/// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file
    pub items: Option<Vec<::v1_12::api::core::v1::DownwardAPIVolumeFile>>,
}

impl<'de> ::serde::Deserialize<'de> for DownwardAPIProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_items,
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
                            "items" => Field::Key_items,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIProjection;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DownwardAPIProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<::v1_12::api::core::v1::DownwardAPIVolumeFile>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_items => value_items = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIProjection {
                    items: value_items,
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIProjection",
            &[
                "items",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for DownwardAPIProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIProjection",
            0 +
            self.items.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.items {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
