// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicyPeer

/// DEPRECATED 1.9 - This group version of NetworkPolicyPeer is deprecated by networking/v1/NetworkPolicyPeer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPeer {
    /// IPBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    pub ip_block: Option<crate::api::extensions::v1beta1::IPBlock>,

    /// Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If PodSelector is also set, then the NetworkPolicyPeer as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    ///
    /// If NamespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.
    pub pod_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> crate::serde::Deserialize<'de> for NetworkPolicyPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip_block,
            Key_namespace_selector,
            Key_pod_selector,
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
                            "ipBlock" => Field::Key_ip_block,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "podSelector" => Field::Key_pod_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicyPeer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NetworkPolicyPeer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip_block: Option<crate::api::extensions::v1beta1::IPBlock> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_pod_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip_block => value_ip_block = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace_selector => value_namespace_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyPeer {
                    ip_block: value_ip_block,
                    namespace_selector: value_namespace_selector,
                    pod_selector: value_pod_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyPeer",
            &[
                "ipBlock",
                "namespaceSelector",
                "podSelector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NetworkPolicyPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPeer",
            self.ip_block.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.pod_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip_block {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipBlock", value)?;
        }
        if let Some(value) = &self.namespace_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.pod_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NetworkPolicyPeer {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DEPRECATED 1.9 - This group version of NetworkPolicyPeer is deprecated by networking/v1/NetworkPolicyPeer.",
          "properties": {
            "ipBlock": crate::schema_ref_with_description(crate::api::extensions::v1beta1::IPBlock::schema(), "IPBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be."),
            "namespaceSelector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.\n\nIf PodSelector is also set, then the NetworkPolicyPeer as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector."),
            "podSelector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.\n\nIf NamespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.")
          },
          "type": "object"
        })
    }
}
