// Generated from definition io.k8s.api.core.v1.TopologySpreadConstraint

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySpreadConstraint {
    /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
    pub label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// MatchLabelKeys is a set of pod label keys to select the pods over which spreading will be calculated. The keys are used to lookup values from the incoming pod labels, those key-value labels are ANDed with labelSelector to select the group of existing pods over which spreading will be calculated for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector. MatchLabelKeys cannot be set when LabelSelector isn't set. Keys that don't exist in the incoming pod labels will be ignored. A null or empty list means only match against labelSelector.
    ///
    /// This is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).
    pub match_label_keys: Option<std::vec::Vec<std::string::String>>,

    /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. The global minimum is the minimum number of matching pods in an eligible domain or zero if the number of eligible domains is less than MinDomains. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 2/2/1: In this case, the global minimum is 1. | zone1 | zone2 | zone3 | |  P P  |  P P  |   P   | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2; scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
    pub max_skew: i32,

    /// MinDomains indicates a minimum number of eligible domains. When the number of eligible domains with matching topology keys is less than minDomains, Pod Topology Spread treats "global minimum" as 0, and then the calculation of Skew is performed. And when the number of eligible domains with matching topology keys equals or greater than minDomains, this value has no effect on scheduling. As a result, when the number of eligible domains is less than minDomains, scheduler won't schedule more than maxSkew Pods to those domains. If value is nil, the constraint behaves as if MinDomains is equal to 1. Valid values are integers greater than 0. When value is not nil, WhenUnsatisfiable must be DoNotSchedule.
    ///
    /// For example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same labelSelector spread as 2/2/2: | zone1 | zone2 | zone3 | |  P P  |  P P  |  P P  | The number of domains is less than 5(MinDomains), so "global minimum" is treated as 0. In this situation, new pod with the same labelSelector cannot be scheduled, because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones, it will violate MaxSkew.
    pub min_domains: Option<i32>,

    /// NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector when calculating pod topology spread skew. Options are: - Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations. - Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.
    ///
    /// If this value is nil, the behavior is equivalent to the Honor policy. This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.
    pub node_affinity_policy: Option<std::string::String>,

    /// NodeTaintsPolicy indicates how we will treat node taints when calculating pod topology spread skew. Options are: - Honor: nodes without taints, along with tainted nodes for which the incoming pod has a toleration, are included. - Ignore: node taints are ignored. All nodes are included.
    ///
    /// If this value is nil, the behavior is equivalent to the Ignore policy. This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.
    pub node_taints_policy: Option<std::string::String>,

    /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each \<key, value\> as a "bucket", and try to put balanced number of pods into each bucket. We define a domain as a particular instance of a topology. Also, we define an eligible domain as a domain whose nodes meet the requirements of nodeAffinityPolicy and nodeTaintsPolicy. e.g. If TopologyKey is "kubernetes.io/hostname", each Node is a domain of that topology. And, if TopologyKey is "topology.kubernetes.io/zone", each zone is a domain of that topology. It's a required field.
    pub topology_key: std::string::String,

    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location,
    ///   but giving higher precedence to topologies that would help reduce the
    ///   skew.
    /// A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
    pub when_unsatisfiable: std::string::String,
}

impl crate::DeepMerge for TopologySpreadConstraint {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.label_selector, other.label_selector);
        crate::merge_strategies::list::atomic(&mut self.match_label_keys, other.match_label_keys);
        crate::DeepMerge::merge_from(&mut self.max_skew, other.max_skew);
        crate::DeepMerge::merge_from(&mut self.min_domains, other.min_domains);
        crate::DeepMerge::merge_from(&mut self.node_affinity_policy, other.node_affinity_policy);
        crate::DeepMerge::merge_from(&mut self.node_taints_policy, other.node_taints_policy);
        crate::DeepMerge::merge_from(&mut self.topology_key, other.topology_key);
        crate::DeepMerge::merge_from(&mut self.when_unsatisfiable, other.when_unsatisfiable);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TopologySpreadConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector,
            Key_match_label_keys,
            Key_max_skew,
            Key_min_domains,
            Key_node_affinity_policy,
            Key_node_taints_policy,
            Key_topology_key,
            Key_when_unsatisfiable,
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
                            "labelSelector" => Field::Key_label_selector,
                            "matchLabelKeys" => Field::Key_match_label_keys,
                            "maxSkew" => Field::Key_max_skew,
                            "minDomains" => Field::Key_min_domains,
                            "nodeAffinityPolicy" => Field::Key_node_affinity_policy,
                            "nodeTaintsPolicy" => Field::Key_node_taints_policy,
                            "topologyKey" => Field::Key_topology_key,
                            "whenUnsatisfiable" => Field::Key_when_unsatisfiable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TopologySpreadConstraint;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TopologySpreadConstraint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_match_label_keys: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_max_skew: Option<i32> = None;
                let mut value_min_domains: Option<i32> = None;
                let mut value_node_affinity_policy: Option<std::string::String> = None;
                let mut value_node_taints_policy: Option<std::string::String> = None;
                let mut value_topology_key: Option<std::string::String> = None;
                let mut value_when_unsatisfiable: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector => value_label_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_label_keys => value_match_label_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_skew => value_max_skew = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_domains => value_min_domains = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_affinity_policy => value_node_affinity_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_taints_policy => value_node_taints_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_key => value_topology_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_when_unsatisfiable => value_when_unsatisfiable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TopologySpreadConstraint {
                    label_selector: value_label_selector,
                    match_label_keys: value_match_label_keys,
                    max_skew: value_max_skew.unwrap_or_default(),
                    min_domains: value_min_domains,
                    node_affinity_policy: value_node_affinity_policy,
                    node_taints_policy: value_node_taints_policy,
                    topology_key: value_topology_key.unwrap_or_default(),
                    when_unsatisfiable: value_when_unsatisfiable.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "TopologySpreadConstraint",
            &[
                "labelSelector",
                "matchLabelKeys",
                "maxSkew",
                "minDomains",
                "nodeAffinityPolicy",
                "nodeTaintsPolicy",
                "topologyKey",
                "whenUnsatisfiable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TopologySpreadConstraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySpreadConstraint",
            3 +
            self.label_selector.as_ref().map_or(0, |_| 1) +
            self.match_label_keys.as_ref().map_or(0, |_| 1) +
            self.min_domains.as_ref().map_or(0, |_| 1) +
            self.node_affinity_policy.as_ref().map_or(0, |_| 1) +
            self.node_taints_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.label_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        if let Some(value) = &self.match_label_keys {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchLabelKeys", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxSkew", &self.max_skew)?;
        if let Some(value) = &self.min_domains {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minDomains", value)?;
        }
        if let Some(value) = &self.node_affinity_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeAffinityPolicy", value)?;
        }
        if let Some(value) = &self.node_taints_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeTaintsPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKey", &self.topology_key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "whenUnsatisfiable", &self.when_unsatisfiable)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TopologySpreadConstraint {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.TopologySpreadConstraint".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("TopologySpreadConstraint specifies how to spread matching pods among the given topology.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "labelSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "matchLabelKeys".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchLabelKeys is a set of pod label keys to select the pods over which spreading will be calculated. The keys are used to lookup values from the incoming pod labels, those key-value labels are ANDed with labelSelector to select the group of existing pods over which spreading will be calculated for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector. MatchLabelKeys cannot be set when LabelSelector isn't set. Keys that don't exist in the incoming pod labels will be ignored. A null or empty list means only match against labelSelector.\n\nThis is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxSkew".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. The global minimum is the minimum number of matching pods in an eligible domain or zero if the number of eligible domains is less than MinDomains. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 2/2/1: In this case, the global minimum is 1. | zone1 | zone2 | zone3 | |  P P  |  P P  |   P   | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2; scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minDomains".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MinDomains indicates a minimum number of eligible domains. When the number of eligible domains with matching topology keys is less than minDomains, Pod Topology Spread treats \"global minimum\" as 0, and then the calculation of Skew is performed. And when the number of eligible domains with matching topology keys equals or greater than minDomains, this value has no effect on scheduling. As a result, when the number of eligible domains is less than minDomains, scheduler won't schedule more than maxSkew Pods to those domains. If value is nil, the constraint behaves as if MinDomains is equal to 1. Valid values are integers greater than 0. When value is not nil, WhenUnsatisfiable must be DoNotSchedule.\n\nFor example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same labelSelector spread as 2/2/2: | zone1 | zone2 | zone3 | |  P P  |  P P  |  P P  | The number of domains is less than 5(MinDomains), so \"global minimum\" is treated as 0. In this situation, new pod with the same labelSelector cannot be scheduled, because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones, it will violate MaxSkew.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeAffinityPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector when calculating pod topology spread skew. Options are: - Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations. - Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.\n\nIf this value is nil, the behavior is equivalent to the Honor policy. This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeTaintsPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeTaintsPolicy indicates how we will treat node taints when calculating pod topology spread skew. Options are: - Honor: nodes without taints, along with tainted nodes for which the incoming pod has a toleration, are included. - Ignore: node taints are ignored. All nodes are included.\n\nIf this value is nil, the behavior is equivalent to the Ignore policy. This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "topologyKey".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a \"bucket\", and try to put balanced number of pods into each bucket. We define a domain as a particular instance of a topology. Also, we define an eligible domain as a domain whose nodes meet the requirements of nodeAffinityPolicy and nodeTaintsPolicy. e.g. If TopologyKey is \"kubernetes.io/hostname\", each Node is a domain of that topology. And, if TopologyKey is \"topology.kubernetes.io/zone\", each zone is a domain of that topology. It's a required field.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "whenUnsatisfiable".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location,\n  but giving higher precedence to topologies that would help reduce the\n  skew.\nA constraint is considered \"Unsatisfiable\" for an incoming pod if and only if every possible node assignment for that pod would violate \"MaxSkew\" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "maxSkew".into(),
                    "topologyKey".into(),
                    "whenUnsatisfiable".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
