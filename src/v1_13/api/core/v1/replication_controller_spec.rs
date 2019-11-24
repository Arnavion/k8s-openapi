// Generated from definition io.k8s.api.core.v1.ReplicationControllerSpec

/// ReplicationControllerSpec is the specification of a replication controller.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicationControllerSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    pub replicas: Option<i32>,

    /// Selector is a label query over pods that should match the Replicas count. If Selector is empty, it is defaulted to the labels present on the Pod template. Label keys and values that must match in order to be controlled by this replication controller, if empty defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: Option<std::collections::BTreeMap<String, String>>,

    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. This takes precedence over a TemplateRef. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    pub template: Option<crate::api::core::v1::PodTemplateSpec>,
}

impl<'de> serde::Deserialize<'de> for ReplicationControllerSpec {
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
            type Value = ReplicationControllerSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ReplicationControllerSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicationControllerSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    replicas: value_replicas,
                    selector: value_selector,
                    template: value_template,
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicationControllerSpec",
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

impl serde::Serialize for ReplicationControllerSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicationControllerSpec",
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.template {
            serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
