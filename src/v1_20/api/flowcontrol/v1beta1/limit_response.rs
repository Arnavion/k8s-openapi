// Generated from definition io.k8s.api.flowcontrol.v1beta1.LimitResponse

/// LimitResponse defines how to handle requests that can not be executed right now.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitResponse {
    /// `queuing` holds the configuration parameters for queuing. This field may be non-empty only if `type` is `"Queue"`.
    pub queuing: Option<crate::api::flowcontrol::v1beta1::QueuingConfiguration>,

    /// `type` is "Queue" or "Reject". "Queue" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. "Reject" means that requests that can not be executed upon arrival are rejected. Required.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for LimitResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_queuing,
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
                            "queuing" => Field::Key_queuing,
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
            type Value = LimitResponse;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LimitResponse")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_queuing: Option<crate::api::flowcontrol::v1beta1::QueuingConfiguration> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_queuing => value_queuing = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitResponse {
                    queuing: value_queuing,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "LimitResponse",
            &[
                "queuing",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LimitResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitResponse",
            1 +
            self.queuing.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.queuing {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "queuing", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for LimitResponse {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "LimitResponse defines how to handle requests that can not be executed right now.",
          "properties": {
            "queuing": crate::schema_ref_with_description(crate::api::flowcontrol::v1beta1::QueuingConfiguration::schema(), "`queuing` holds the configuration parameters for queuing. This field may be non-empty only if `type` is `\"Queue\"`."),
            "type": {
              "description": "`type` is \"Queue\" or \"Reject\". \"Queue\" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. \"Reject\" means that requests that can not be executed upon arrival are rejected. Required.",
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
