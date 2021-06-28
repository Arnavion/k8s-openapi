// Generated from definition io.k8s.api.core.v1.DownwardAPIProjection

/// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file
    pub items: Vec<crate::api::core::v1::DownwardAPIVolumeFile>,
}

impl<'de> crate::serde::Deserialize<'de> for DownwardAPIProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_items,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIProjection;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DownwardAPIProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<crate::api::core::v1::DownwardAPIVolumeFile>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIProjection {
                    items: value_items.unwrap_or_default(),
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

impl crate::serde::Serialize for DownwardAPIProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIProjection",
            usize::from(!self.items.is_empty()),
        )?;
        if !self.items.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for DownwardAPIProjection {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.",
          "properties": {
            "items": {
              "description": "Items is a list of DownwardAPIVolume file",
              "items": crate::api::core::v1::DownwardAPIVolumeFile::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
