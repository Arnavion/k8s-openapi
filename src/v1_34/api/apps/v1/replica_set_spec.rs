// Generated from definition io.k8s.api.apps.v1.ReplicaSetSpec

/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicaSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// Replicas is the number of desired pods. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset
    pub replicas: Option<i32>,

    /// Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/#pod-template
    pub template: Option<crate::api::core::v1::PodTemplateSpec>,
}

impl crate::DeepMerge for ReplicaSetSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.min_ready_seconds, other.min_ready_seconds);
        crate::DeepMerge::merge_from(&mut self.replicas, other.replicas);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.template, other.template);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ReplicaSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_replicas,
            Key_selector,
            Key_template,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicaSetSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ReplicaSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicaSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    replicas: value_replicas,
                    selector: value_selector.unwrap_or_default(),
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

impl crate::serde::Serialize for ReplicaSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicaSetSpec",
            1 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.template.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        if let Some(value) = &self.template {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ReplicaSetSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apps.v1.ReplicaSetSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ReplicaSetSpec is the specification of a ReplicaSet.",
            "type": "object",
            "properties": {
                "minReadySeconds": {
                    "description": "Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)",
                    "type": "integer",
                    "format": "int32",
                },
                "replicas": {
                    "description": "Replicas is the number of desired pods. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset",
                    "type": "integer",
                    "format": "int32",
                },
                "selector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>();
                    schema_obj.ensure_object().insert("description".into(), "Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".into());
                    schema_obj
                }),
                "template": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>();
                    schema_obj.ensure_object().insert("description".into(), "Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/#pod-template".into());
                    schema_obj
                }),
            },
            "required": [
                "selector",
            ],
        })
    }
}
