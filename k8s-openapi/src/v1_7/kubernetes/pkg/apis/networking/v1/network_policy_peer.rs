// Generated from definition io.k8s.kubernetes.pkg.apis.networking.v1.NetworkPolicyPeer

/// NetworkPolicyPeer describes a peer to allow traffic from. Exactly one of its fields must be specified.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPeer {
    /// Selects Namespaces using cluster scoped-labels. This matches all pods in all namespaces selected by this label selector. This field follows standard label selector semantics. If present but empty, this selector selects all namespaces.
    pub namespace_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// This is a label selector which selects Pods in this namespace. This field follows standard label selector semantics. If present but empty, this selector selects all pods in this namespace.
    pub pod_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl<'de> ::serde::Deserialize<'de> for NetworkPolicyPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespace_selector,
            Key_pod_selector,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicyPeer;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicyPeer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_namespace_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_pod_selector: Option<::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespace_selector => value_namespace_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyPeer {
                    namespace_selector: value_namespace_selector,
                    pod_selector: value_pod_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyPeer",
            &[
                "namespaceSelector",
                "podSelector",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicyPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPeer",
            0 +
            (if self.namespace_selector.is_some() { 1 } else { 0 }) +
            (if self.pod_selector.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.namespace_selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.pod_selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
