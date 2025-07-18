// Generated from definition io.k8s.api.discovery.v1.EndpointConditions

/// EndpointConditions represents the current condition of an endpoint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointConditions {
    /// ready indicates that this endpoint is ready to receive traffic, according to whatever system is managing the endpoint. A nil value should be interpreted as "true". In general, an endpoint should be marked ready if it is serving and not terminating, though this can be overridden in some cases, such as when the associated Service has set the publishNotReadyAddresses flag.
    pub ready: Option<bool>,

    /// serving indicates that this endpoint is able to receive traffic, according to whatever system is managing the endpoint. For endpoints backed by pods, the EndpointSlice controller will mark the endpoint as serving if the pod's Ready condition is True. A nil value should be interpreted as "true".
    pub serving: Option<bool>,

    /// terminating indicates that this endpoint is terminating. A nil value should be interpreted as "false".
    pub terminating: Option<bool>,
}

impl crate::DeepMerge for EndpointConditions {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ready, other.ready);
        crate::DeepMerge::merge_from(&mut self.serving, other.serving);
        crate::DeepMerge::merge_from(&mut self.terminating, other.terminating);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EndpointConditions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ready,
            Key_serving,
            Key_terminating,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "ready" => Field::Key_ready,
                            "serving" => Field::Key_serving,
                            "terminating" => Field::Key_terminating,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointConditions;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EndpointConditions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ready: Option<bool> = None;
                let mut value_serving: Option<bool> = None;
                let mut value_terminating: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ready => value_ready = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_serving => value_serving = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_terminating => value_terminating = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointConditions {
                    ready: value_ready,
                    serving: value_serving,
                    terminating: value_terminating,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointConditions",
            &[
                "ready",
                "serving",
                "terminating",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointConditions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointConditions",
            self.ready.as_ref().map_or(0, |_| 1) +
            self.serving.as_ref().map_or(0, |_| 1) +
            self.terminating.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ready {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ready", value)?;
        }
        if let Some(value) = &self.serving {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serving", value)?;
        }
        if let Some(value) = &self.terminating {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminating", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EndpointConditions {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.discovery.v1.EndpointConditions".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EndpointConditions represents the current condition of an endpoint.",
            "type": "object",
            "properties": {
                "ready": {
                    "description": "ready indicates that this endpoint is ready to receive traffic, according to whatever system is managing the endpoint. A nil value should be interpreted as \"true\". In general, an endpoint should be marked ready if it is serving and not terminating, though this can be overridden in some cases, such as when the associated Service has set the publishNotReadyAddresses flag.",
                    "type": "boolean",
                },
                "serving": {
                    "description": "serving indicates that this endpoint is able to receive traffic, according to whatever system is managing the endpoint. For endpoints backed by pods, the EndpointSlice controller will mark the endpoint as serving if the pod's Ready condition is True. A nil value should be interpreted as \"true\".",
                    "type": "boolean",
                },
                "terminating": {
                    "description": "terminating indicates that this endpoint is terminating. A nil value should be interpreted as \"false\".",
                    "type": "boolean",
                },
            },
        })
    }
}
