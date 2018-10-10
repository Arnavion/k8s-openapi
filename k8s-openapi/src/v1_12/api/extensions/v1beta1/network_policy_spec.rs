// Generated from definition io.k8s.api.extensions.v1beta1.NetworkPolicySpec

/// DEPRECATED 1.9 - This group version of NetworkPolicySpec is deprecated by networking/v1/NetworkPolicySpec.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicySpec {
    /// List of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
    pub egress: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyEgressRule>>,

    /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default).
    pub ingress: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyIngressRule>>,

    /// Selects the pods to which this NetworkPolicy object applies.  The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods.  In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
    pub pod_selector: ::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// List of rule types that the NetworkPolicy relates to. Valid options are Ingress, Egress, or Ingress,Egress. If this field is not specified, it will default based on the existence of Ingress or Egress rules; policies that contain an Egress section are assumed to affect Egress, and all policies (whether or not they contain an Ingress section) are assumed to affect Ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes \[ "Egress" \]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include "Egress" (since such a policy would not include an Egress section and would otherwise default to just \[ "Ingress" \]). This field is beta-level in 1.8
    pub policy_types: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for NetworkPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_egress,
            Key_ingress,
            Key_pod_selector,
            Key_policy_types,
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
                            "egress" => Field::Key_egress,
                            "ingress" => Field::Key_ingress,
                            "podSelector" => Field::Key_pod_selector,
                            "policyTypes" => Field::Key_policy_types,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicySpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NetworkPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_egress: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyEgressRule>> = None;
                let mut value_ingress: Option<Vec<::v1_12::api::extensions::v1beta1::NetworkPolicyIngressRule>> = None;
                let mut value_pod_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_policy_types: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_egress => value_egress = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ingress => value_ingress = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_policy_types => value_policy_types = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicySpec {
                    egress: value_egress,
                    ingress: value_ingress,
                    pod_selector: value_pod_selector.ok_or_else(|| ::serde::de::Error::missing_field("podSelector"))?,
                    policy_types: value_policy_types,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicySpec",
            &[
                "egress",
                "ingress",
                "podSelector",
                "policyTypes",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NetworkPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicySpec",
            0 +
            self.egress.as_ref().map_or(0, |_| 1) +
            self.ingress.as_ref().map_or(0, |_| 1) +
            1 +
            self.policy_types.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.egress {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "egress", value)?;
        }
        if let Some(value) = &self.ingress {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", &self.pod_selector)?;
        if let Some(value) = &self.policy_types {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "policyTypes", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
