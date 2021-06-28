// Generated from definition io.k8s.api.policy.v1beta1.AllowedCSIDriver

/// AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllowedCSIDriver {
    /// Name is the registered name of the CSI driver
    pub name: String,
}

impl<'de> crate::serde::Deserialize<'de> for AllowedCSIDriver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
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
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AllowedCSIDriver;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AllowedCSIDriver")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllowedCSIDriver {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllowedCSIDriver",
            &[
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AllowedCSIDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllowedCSIDriver",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for AllowedCSIDriver {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used.",
          "properties": {
            "name": {
              "description": "Name is the registered name of the CSI driver",
              "type": "string"
            }
          },
          "required": [
            "name"
          ],
          "type": "object"
        })
    }
}
