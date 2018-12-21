// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.NetworkPolicySpec

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicySpec {
    /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default).
    pub ingress: Option<Vec<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::NetworkPolicyIngressRule>>,

    /// Selects the pods to which this NetworkPolicy object applies.  The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods.  In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
    pub pod_selector: crate::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector,
}

impl<'de> serde::Deserialize<'de> for NetworkPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ingress,
            Key_pod_selector,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "ingress" => Field::Key_ingress,
                            "podSelector" => Field::Key_pod_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct NetworkPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ingress: Option<Vec<crate::v1_7::kubernetes::pkg::apis::extensions::v1beta1::NetworkPolicyIngressRule>> = None;
                let mut value_pod_selector: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicySpec {
                    ingress: value_ingress,
                    pod_selector: value_pod_selector.ok_or_else(|| serde::de::Error::missing_field("podSelector"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicySpec",
            &[
                "ingress",
                "podSelector",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for NetworkPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicySpec",
            0 +
            self.ingress.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.ingress {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", &self.pod_selector)?;
        serde::ser::SerializeStruct::end(state)
    }
}
