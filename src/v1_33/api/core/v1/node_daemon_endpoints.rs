// Generated from definition io.k8s.api.core.v1.NodeDaemonEndpoints

/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeDaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    pub kubelet_endpoint: Option<crate::api::core::v1::DaemonEndpoint>,
}

impl crate::DeepMerge for NodeDaemonEndpoints {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.kubelet_endpoint, other.kubelet_endpoint);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeDaemonEndpoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_kubelet_endpoint,
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
                            "kubeletEndpoint" => Field::Key_kubelet_endpoint,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeDaemonEndpoints;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeDaemonEndpoints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_kubelet_endpoint: Option<crate::api::core::v1::DaemonEndpoint> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_kubelet_endpoint => value_kubelet_endpoint = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeDaemonEndpoints {
                    kubelet_endpoint: value_kubelet_endpoint,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeDaemonEndpoints",
            &[
                "kubeletEndpoint",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeDaemonEndpoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeDaemonEndpoints",
            self.kubelet_endpoint.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.kubelet_endpoint {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kubeletEndpoint", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeDaemonEndpoints {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodeDaemonEndpoints".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeDaemonEndpoints lists ports opened by daemons running on the Node.",
            "type": "object",
            "properties": {
                "kubeletEndpoint": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::DaemonEndpoint>();
                    schema_obj.ensure_object().insert("description".into(), "Endpoint on which Kubelet is listening.".into());
                    schema_obj
                }),
            },
        })
    }
}
