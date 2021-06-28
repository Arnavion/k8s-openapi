// Generated from definition io.k8s.api.core.v1.LimitRangeSpec

/// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    pub limits: Vec<crate::api::core::v1::LimitRangeItem>,
}

impl<'de> crate::serde::Deserialize<'de> for LimitRangeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_limits,
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
                            "limits" => Field::Key_limits,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LimitRangeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LimitRangeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_limits: Option<Vec<crate::api::core::v1::LimitRangeItem>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_limits => value_limits = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitRangeSpec {
                    limits: value_limits.ok_or_else(|| crate::serde::de::Error::missing_field("limits"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "LimitRangeSpec",
            &[
                "limits",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LimitRangeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitRangeSpec",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "limits", &self.limits)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for LimitRangeSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "LimitRangeSpec defines a min/max usage limit for resources that match on kind.",
          "properties": {
            "limits": {
              "description": "Limits is the list of LimitRangeItem objects that are enforced.",
              "items": crate::api::core::v1::LimitRangeItem::schema(),
              "type": "array"
            }
          },
          "required": [
            "limits"
          ],
          "type": "object"
        })
    }
}
