// Generated from definition io.k8s.api.core.v1.VolumeNodeAffinity

/// VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeNodeAffinity {
    /// Required specifies hard node constraints that must be met.
    pub required: Option<crate::api::core::v1::NodeSelector>,
}

impl<'de> crate::serde::Deserialize<'de> for VolumeNodeAffinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_required,
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
                            "required" => Field::Key_required,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeNodeAffinity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeNodeAffinity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_required: Option<crate::api::core::v1::NodeSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_required => value_required = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeNodeAffinity {
                    required: value_required,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeNodeAffinity",
            &[
                "required",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeNodeAffinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeNodeAffinity",
            self.required.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.required {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "required", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for VolumeNodeAffinity {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.",
          "properties": {
            "required": crate::schema_ref_with_values(crate::api::core::v1::NodeSelector::schema(), serde_json::json!({"description": "Required specifies hard node constraints that must be met."}))
          },
          "type": "object"
        })
    }
}
