// Generated from definition io.k8s.api.node.v1alpha1.RuntimeClassSpec

/// RuntimeClassSpec is a specification of a RuntimeClass. It contains parameters that are required to describe the RuntimeClass to the Container Runtime Interface (CRI) implementation, as well as any other components that need to understand how the pod will be run. The RuntimeClassSpec is immutable.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuntimeClassSpec {
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. For more details, see https://git.k8s.io/enhancements/keps/sig-node/20190226-pod-overhead.md This field is alpha-level as of Kubernetes v1.15, and is only honored by servers that enable the PodOverhead feature.
    pub overhead: Option<crate::api::node::v1alpha1::Overhead>,

    /// RuntimeHandler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called "runc" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The RuntimeHandler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.
    pub runtime_handler: String,

    /// Scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it. If scheduling is nil, this RuntimeClass is assumed to be supported by all nodes.
    pub scheduling: Option<crate::api::node::v1alpha1::Scheduling>,
}

impl<'de> serde::Deserialize<'de> for RuntimeClassSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_overhead,
            Key_runtime_handler,
            Key_scheduling,
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
                            "overhead" => Field::Key_overhead,
                            "runtimeHandler" => Field::Key_runtime_handler,
                            "scheduling" => Field::Key_scheduling,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuntimeClassSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RuntimeClassSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_overhead: Option<crate::api::node::v1alpha1::Overhead> = None;
                let mut value_runtime_handler: Option<String> = None;
                let mut value_scheduling: Option<crate::api::node::v1alpha1::Scheduling> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_overhead => value_overhead = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_runtime_handler => value_runtime_handler = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_scheduling => value_scheduling = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuntimeClassSpec {
                    overhead: value_overhead,
                    runtime_handler: value_runtime_handler.ok_or_else(|| serde::de::Error::missing_field("runtimeHandler"))?,
                    scheduling: value_scheduling,
                })
            }
        }

        deserializer.deserialize_struct(
            "RuntimeClassSpec",
            &[
                "overhead",
                "runtimeHandler",
                "scheduling",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RuntimeClassSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RuntimeClassSpec",
            1 +
            self.overhead.as_ref().map_or(0, |_| 1) +
            self.scheduling.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.overhead {
            serde::ser::SerializeStruct::serialize_field(&mut state, "overhead", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "runtimeHandler", &self.runtime_handler)?;
        if let Some(value) = &self.scheduling {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scheduling", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
