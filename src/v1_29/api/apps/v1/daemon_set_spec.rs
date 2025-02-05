// Generated from definition io.k8s.api.apps.v1.DaemonSetSpec

/// DaemonSetSpec is the specification of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetSpec {
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
    pub min_ready_seconds: Option<i32>,

    /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    pub revision_history_limit: Option<i32>,

    /// A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). The only allowed template.spec.restartPolicy value is "Always". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    pub template: crate::api::core::v1::PodTemplateSpec,

    /// An update strategy to replace existing DaemonSet pods with new pods.
    pub update_strategy: Option<crate::api::apps::v1::DaemonSetUpdateStrategy>,
}

impl crate::DeepMerge for DaemonSetSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.min_ready_seconds, other.min_ready_seconds);
        crate::DeepMerge::merge_from(&mut self.revision_history_limit, other.revision_history_limit);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.template, other.template);
        crate::DeepMerge::merge_from(&mut self.update_strategy, other.update_strategy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DaemonSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_revision_history_limit,
            Key_selector,
            Key_template,
            Key_update_strategy,
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
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "selector" => Field::Key_selector,
                            "template" => Field::Key_template,
                            "updateStrategy" => Field::Key_update_strategy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DaemonSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;
                let mut value_update_strategy: Option<crate::api::apps::v1::DaemonSetUpdateStrategy> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_strategy => value_update_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector.unwrap_or_default(),
                    template: value_template.unwrap_or_default(),
                    update_strategy: value_update_strategy,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSetSpec",
            &[
                "minReadySeconds",
                "revisionHistoryLimit",
                "selector",
                "template",
                "updateStrategy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DaemonSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetSpec",
            2 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.update_strategy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.update_strategy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updateStrategy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DaemonSetSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.apps.v1.DaemonSetSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DaemonSetSpec is the specification of a daemon set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "minReadySeconds".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "revisionHistoryLimit".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "template".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). The only allowed template.spec.restartPolicy value is \"Always\". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "updateStrategy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::DaemonSetUpdateStrategy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("An update strategy to replace existing DaemonSet pods with new pods.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "selector".into(),
                    "template".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
