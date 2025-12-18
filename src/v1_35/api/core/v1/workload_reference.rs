// Generated from definition io.k8s.api.core.v1.WorkloadReference

/// WorkloadReference identifies the Workload object and PodGroup membership that a Pod belongs to. The scheduler uses this information to apply workload-aware scheduling semantics.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadReference {
    /// Name defines the name of the Workload object this Pod belongs to. Workload must be in the same namespace as the Pod. If it doesn't match any existing Workload, the Pod will remain unschedulable until a Workload object is created and observed by the kube-scheduler. It must be a DNS subdomain.
    pub name: std::string::String,

    /// PodGroup is the name of the PodGroup within the Workload that this Pod belongs to. If it doesn't match any existing PodGroup within the Workload, the Pod will remain unschedulable until the Workload object is recreated and observed by the kube-scheduler. It must be a DNS label.
    pub pod_group: std::string::String,

    /// PodGroupReplicaKey specifies the replica key of the PodGroup to which this Pod belongs. It is used to distinguish pods belonging to different replicas of the same pod group. The pod group policy is applied separately to each replica. When set, it must be a DNS label.
    pub pod_group_replica_key: Option<std::string::String>,
}

impl crate::DeepMerge for WorkloadReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.pod_group, other.pod_group);
        crate::DeepMerge::merge_from(&mut self.pod_group_replica_key, other.pod_group_replica_key);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_pod_group,
            Key_pod_group_replica_key,
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
                            "name" => Field::Key_name,
                            "podGroup" => Field::Key_pod_group,
                            "podGroupReplicaKey" => Field::Key_pod_group_replica_key,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_pod_group: Option<std::string::String> = None;
                let mut value_pod_group_replica_key: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_group => value_pod_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_group_replica_key => value_pod_group_replica_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadReference {
                    name: value_name.unwrap_or_default(),
                    pod_group: value_pod_group.unwrap_or_default(),
                    pod_group_replica_key: value_pod_group_replica_key,
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadReference",
            &[
                "name",
                "podGroup",
                "podGroupReplicaKey",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadReference",
            2 +
            self.pod_group_replica_key.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroup", &self.pod_group)?;
        if let Some(value) = &self.pod_group_replica_key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroupReplicaKey", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.WorkloadReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadReference identifies the Workload object and PodGroup membership that a Pod belongs to. The scheduler uses this information to apply workload-aware scheduling semantics.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "Name defines the name of the Workload object this Pod belongs to. Workload must be in the same namespace as the Pod. If it doesn't match any existing Workload, the Pod will remain unschedulable until a Workload object is created and observed by the kube-scheduler. It must be a DNS subdomain.",
                    "type": "string",
                },
                "podGroup": {
                    "description": "PodGroup is the name of the PodGroup within the Workload that this Pod belongs to. If it doesn't match any existing PodGroup within the Workload, the Pod will remain unschedulable until the Workload object is recreated and observed by the kube-scheduler. It must be a DNS label.",
                    "type": "string",
                },
                "podGroupReplicaKey": {
                    "description": "PodGroupReplicaKey specifies the replica key of the PodGroup to which this Pod belongs. It is used to distinguish pods belonging to different replicas of the same pod group. The pod group policy is applied separately to each replica. When set, it must be a DNS label.",
                    "type": "string",
                },
            },
            "required": [
                "name",
                "podGroup",
            ],
        })
    }
}
