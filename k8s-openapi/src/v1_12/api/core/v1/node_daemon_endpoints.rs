// Generated from definition io.k8s.api.core.v1.NodeDaemonEndpoints

/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeDaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    pub kubelet_endpoint: Option<::v1_12::api::core::v1::DaemonEndpoint>,
}

impl<'de> ::serde::Deserialize<'de> for NodeDaemonEndpoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_kubelet_endpoint,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeDaemonEndpoints;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeDaemonEndpoints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_kubelet_endpoint: Option<::v1_12::api::core::v1::DaemonEndpoint> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_kubelet_endpoint => value_kubelet_endpoint = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for NodeDaemonEndpoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeDaemonEndpoints",
            0 +
            self.kubelet_endpoint.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.kubelet_endpoint {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kubeletEndpoint", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
