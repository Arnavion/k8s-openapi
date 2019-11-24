// Generated from definition io.k8s.api.apps.v1.ReplicaSetSpec

/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicaSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    pub replicas: Option<i32>,

    /// Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    pub template: Option<crate::api::core::v1::PodTemplateSpec>,
}

impl<'de> serde::Deserialize<'de> for ReplicaSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_replicas,
            Key_selector,
            Key_template,
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
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "replicas" => Field::Key_replicas,
                            "selector" => Field::Key_selector,
                            "template" => Field::Key_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReplicaSetSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ReplicaSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_template => value_template = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicaSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    replicas: value_replicas,
                    selector: value_selector.ok_or_else(|| serde::de::Error::missing_field("selector"))?,
                    template: value_template,
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicaSetSpec",
            &[
                "minReadySeconds",
                "replicas",
                "selector",
                "template",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ReplicaSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicaSetSpec",
            1 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        if let Some(value) = &self.template {
            serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
