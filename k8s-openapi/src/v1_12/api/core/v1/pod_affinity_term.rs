// Generated from definition io.k8s.api.core.v1.PodAffinityTerm

/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    pub label_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means "this pod's namespace"
    pub namespaces: Option<Vec<String>>,

    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topology_key: String,
}

impl<'de> ::serde::Deserialize<'de> for PodAffinityTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector,
            Key_namespaces,
            Key_topology_key,
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
                            "labelSelector" => Field::Key_label_selector,
                            "namespaces" => Field::Key_namespaces,
                            "topologyKey" => Field::Key_topology_key,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodAffinityTerm;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodAffinityTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_label_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_topology_key: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector => value_label_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaces => value_namespaces = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_key => value_topology_key = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodAffinityTerm {
                    label_selector: value_label_selector,
                    namespaces: value_namespaces,
                    topology_key: value_topology_key.ok_or_else(|| ::serde::de::Error::missing_field("topologyKey"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodAffinityTerm",
            &[
                "labelSelector",
                "namespaces",
                "topologyKey",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodAffinityTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodAffinityTerm",
            0 +
            self.label_selector.as_ref().map_or(0, |_| 1) +
            self.namespaces.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.label_selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        if let Some(value) = &self.namespaces {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKey", &self.topology_key)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
