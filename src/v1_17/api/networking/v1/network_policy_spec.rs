// Generated from definition io.k8s.api.networking.v1.NetworkPolicySpec

/// NetworkPolicySpec provides the specification of a NetworkPolicy
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicySpec {
    /// List of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
    pub egress: Vec<crate::api::networking::v1::NetworkPolicyEgressRule>,

    /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default)
    pub ingress: Vec<crate::api::networking::v1::NetworkPolicyIngressRule>,

    /// Selects the pods to which this NetworkPolicy object applies. The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
    pub pod_selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// List of rule types that the NetworkPolicy relates to. Valid options are "Ingress", "Egress", or "Ingress,Egress". If this field is not specified, it will default based on the existence of Ingress or Egress rules; policies that contain an Egress section are assumed to affect Egress, and all policies (whether or not they contain an Ingress section) are assumed to affect Ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes \[ "Egress" \]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include "Egress" (since such a policy would not include an Egress section and would otherwise default to just \[ "Ingress" \]). This field is beta-level in 1.8
    pub policy_types: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for NetworkPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_egress,
            Key_ingress,
            Key_pod_selector,
            Key_policy_types,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NetworkPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_egress: Option<Vec<crate::api::networking::v1::NetworkPolicyEgressRule>> = None;
                let mut value_ingress: Option<Vec<crate::api::networking::v1::NetworkPolicyIngressRule>> = None;
                let mut value_pod_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_policy_types: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_egress => value_egress = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ingress => value_ingress = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_policy_types => value_policy_types = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicySpec {
                    egress: value_egress.unwrap_or_default(),
                    ingress: value_ingress.unwrap_or_default(),
                    pod_selector: value_pod_selector.ok_or_else(|| crate::serde::de::Error::missing_field("podSelector"))?,
                    policy_types: value_policy_types.unwrap_or_default(),
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

impl crate::serde::Serialize for NetworkPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicySpec",
            1 +
            usize::from(!self.egress.is_empty()) +
            usize::from(!self.ingress.is_empty()) +
            usize::from(!self.policy_types.is_empty()),
        )?;
        if !self.egress.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "egress", &self.egress)?;
        }
        if !self.ingress.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", &self.ingress)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", &self.pod_selector)?;
        if !self.policy_types.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "policyTypes", &self.policy_types)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NetworkPolicySpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "NetworkPolicySpec provides the specification of a NetworkPolicy",
          "properties": {
            "egress": {
              "description": "List of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8",
              "items": crate::api::networking::v1::NetworkPolicyEgressRule::schema(),
              "type": "array"
            },
            "ingress": {
              "description": "List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default)",
              "items": crate::api::networking::v1::NetworkPolicyIngressRule::schema(),
              "type": "array"
            },
            "podSelector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "Selects the pods to which this NetworkPolicy object applies. The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace."),
            "policyTypes": {
              "description": "List of rule types that the NetworkPolicy relates to. Valid options are \"Ingress\", \"Egress\", or \"Ingress,Egress\". If this field is not specified, it will default based on the existence of Ingress or Egress rules; policies that contain an Egress section are assumed to affect Egress, and all policies (whether or not they contain an Ingress section) are assumed to affect Ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes [ \"Egress\" ]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include \"Egress\" (since such a policy would not include an Egress section and would otherwise default to just [ \"Ingress\" ]). This field is beta-level in 1.8",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "required": [
            "podSelector"
          ],
          "type": "object"
        })
    }
}
