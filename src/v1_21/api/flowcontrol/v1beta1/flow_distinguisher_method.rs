// Generated from definition io.k8s.api.flowcontrol.v1beta1.FlowDistinguisherMethod

/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowDistinguisherMethod {
    /// `type` is the type of flow distinguisher method The supported types are "ByUser" and "ByNamespace". Required.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for FlowDistinguisherMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_type_,
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
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlowDistinguisherMethod;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowDistinguisherMethod")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowDistinguisherMethod {
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlowDistinguisherMethod",
            &[
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlowDistinguisherMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowDistinguisherMethod",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl FlowDistinguisherMethod {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "FlowDistinguisherMethod specifies the method of a flow distinguisher.",
          "properties": {
            "type": {
              "description": "`type` is the type of flow distinguisher method The supported types are \"ByUser\" and \"ByNamespace\". Required.",
              "type": "string"
            }
          },
          "required": [
            "type"
          ],
          "type": "object"
        })
    }
}
