// Generated from definition io.k8s.api.core.v1.PreferredSchedulingTerm

/// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreferredSchedulingTerm {
    /// A node selector term, associated with the corresponding weight.
    pub preference: crate::api::core::v1::NodeSelectorTerm,

    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    pub weight: i32,
}

impl<'de> crate::serde::Deserialize<'de> for PreferredSchedulingTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_preference,
            Key_weight,
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
                            "preference" => Field::Key_preference,
                            "weight" => Field::Key_weight,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PreferredSchedulingTerm;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PreferredSchedulingTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_preference: Option<crate::api::core::v1::NodeSelectorTerm> = None;
                let mut value_weight: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_preference => value_preference = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_weight => value_weight = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PreferredSchedulingTerm {
                    preference: value_preference.ok_or_else(|| crate::serde::de::Error::missing_field("preference"))?,
                    weight: value_weight.ok_or_else(|| crate::serde::de::Error::missing_field("weight"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PreferredSchedulingTerm",
            &[
                "preference",
                "weight",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PreferredSchedulingTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PreferredSchedulingTerm",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preference", &self.preference)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "weight", &self.weight)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PreferredSchedulingTerm {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).",
          "properties": {
            "preference": crate::schema_ref_with_values(crate::api::core::v1::NodeSelectorTerm::schema(), serde_json::json!({"description": "A node selector term, associated with the corresponding weight."})),
            "weight": {
              "description": "Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.",
              "format": "int32",
              "type": "integer"
            }
          },
          "required": [
            "preference",
            "weight"
          ],
          "type": "object"
        })
    }
}
