// Generated from definition io.k8s.api.networking.v1.NetworkPolicyPeer

/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPeer {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    pub ip_block: Option<crate::api::networking::v1::IPBlock>,

    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    ///
    /// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
    pub pod_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl crate::DeepMerge for NetworkPolicyPeer {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ip_block, other.ip_block);
        crate::DeepMerge::merge_from(&mut self.namespace_selector, other.namespace_selector);
        crate::DeepMerge::merge_from(&mut self.pod_selector, other.pod_selector);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NetworkPolicyPeer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip_block: Option<crate::api::networking::v1::IPBlock> = None;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NetworkPolicyPeer {
    fn schema_name() -> std::string::String {
        "io.k8s.api.networking.v1.NetworkPolicyPeer".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipBlock".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::IPBlock>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "namespaceSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.\n\nIf podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "podSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods.\n\nIf namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
