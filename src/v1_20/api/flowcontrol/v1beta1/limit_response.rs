// Generated from definition io.k8s.api.flowcontrol.v1beta1.LimitResponse

/// LimitResponse defines how to handle requests that can not be executed right now.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitResponse {
    /// `queuing` holds the configuration parameters for queuing. This field may be non-empty only if `type` is `"Queue"`.
    pub queuing: Option<crate::api::flowcontrol::v1beta1::QueuingConfiguration>,

    /// `type` is "Queue" or "Reject". "Queue" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. "Reject" means that requests that can not be executed upon arrival are rejected. Required.
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for LimitResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_queuing,
            Key_type_,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LimitResponse;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LimitResponse")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_queuing: Option<crate::api::flowcontrol::v1beta1::QueuingConfiguration> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_queuing => value_queuing = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitResponse {
                    queuing: value_queuing,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
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

impl serde::Serialize for LimitResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitResponse",
            1 +
            self.queuing.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.queuing {
            serde::ser::SerializeStruct::serialize_field(&mut state, "queuing", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
