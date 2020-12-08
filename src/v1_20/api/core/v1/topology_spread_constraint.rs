// Generated from definition io.k8s.api.core.v1.TopologySpreadConstraint

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySpreadConstraint {
    /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
    pub label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 1/1/0: | zone1 | zone2 | zone3 | |   P   |   P   |       | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1; scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
    pub max_skew: i32,

    /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each \<key, value\> as a "bucket", and try to put balanced number of pods into each bucket. It's a required field.
    pub topology_key: String,

    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location,
    ///   but giving higher precedence to topologies that would help reduce the
    ///   skew.
    /// A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assigment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
    pub when_unsatisfiable: String,
}

impl<'de> serde::Deserialize<'de> for TopologySpreadConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector,
            Key_max_skew,
            Key_topology_key,
            Key_when_unsatisfiable,
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
                            "labelSelector" => Field::Key_label_selector,
                            "maxSkew" => Field::Key_max_skew,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TopologySpreadConstraint;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TopologySpreadConstraint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_max_skew: Option<i32> = None;
                let mut value_topology_key: Option<String> = None;
                let mut value_when_unsatisfiable: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector => value_label_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_skew => value_max_skew = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_topology_key => value_topology_key = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_when_unsatisfiable => value_when_unsatisfiable = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TopologySpreadConstraint {
                    label_selector: value_label_selector,
                    max_skew: value_max_skew.ok_or_else(|| serde::de::Error::missing_field("maxSkew"))?,
                    topology_key: value_topology_key.ok_or_else(|| serde::de::Error::missing_field("topologyKey"))?,
                    when_unsatisfiable: value_when_unsatisfiable.ok_or_else(|| serde::de::Error::missing_field("whenUnsatisfiable"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TopologySpreadConstraint",
            &[
                "labelSelector",
                "maxSkew",
                "topologyKey",
                "whenUnsatisfiable",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for TopologySpreadConstraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySpreadConstraint",
            3 +
            self.label_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.label_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "maxSkew", &self.max_skew)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKey", &self.topology_key)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "whenUnsatisfiable", &self.when_unsatisfiable)?;
        serde::ser::SerializeStruct::end(state)
    }
}
