// Generated from definition io.k8s.api.scheduling.v1alpha3.CompositePodGroupTemplate

/// CompositePodGroupTemplate represents a template for a CompositePodGroup with a scheduling policy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositePodGroupTemplate {
    /// compositePodGroupTemplates is the list of templates for children CompositePodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.
    pub composite_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::CompositePodGroupTemplate>>,

    /// disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. One of Single, All. This field is immutable.
    pub disruption_mode: Option<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>,

    /// name is a unique identifier for the CompositePodGroupTemplate within the Workload. It must be a DNS label. This field is required.
    pub name: std::string::String,

    /// podGroupTemplates is the list of templates for children PodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.
    pub pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::PodGroupTemplate>>,

    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    pub preemption_policy: Option<std::string::String>,

    /// priority is the value of priority of composite pod groups created from this template. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.
    pub priority: Option<i32>,

    /// priorityClassName indicates the priority that should be considered when scheduling a composite pod group created from this template. If no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, composite pod groups created from this template will have the priority set to zero. This field is immutable.
    pub priority_class_name: Option<std::string::String>,

    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroupTemplate. This field is immutable.
    pub scheduling_constraints: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints>,

    /// schedulingPolicy defines the scheduling policy for this template.
    pub scheduling_policy: crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy,
}

impl crate::DeepMerge for CompositePodGroupTemplate {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.composite_pod_group_templates,
            other.composite_pod_group_templates,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.disruption_mode, other.disruption_mode);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::map(
            &mut self.pod_group_templates,
            other.pod_group_templates,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.preemption_policy, other.preemption_policy);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
        crate::DeepMerge::merge_from(&mut self.priority_class_name, other.priority_class_name);
        crate::DeepMerge::merge_from(&mut self.scheduling_constraints, other.scheduling_constraints);
        crate::DeepMerge::merge_from(&mut self.scheduling_policy, other.scheduling_policy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CompositePodGroupTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_composite_pod_group_templates,
            Key_disruption_mode,
            Key_name,
            Key_pod_group_templates,
            Key_preemption_policy,
            Key_priority,
            Key_priority_class_name,
            Key_scheduling_constraints,
            Key_scheduling_policy,
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
                            "compositePodGroupTemplates" => Field::Key_composite_pod_group_templates,
                            "disruptionMode" => Field::Key_disruption_mode,
                            "name" => Field::Key_name,
                            "podGroupTemplates" => Field::Key_pod_group_templates,
                            "preemptionPolicy" => Field::Key_preemption_policy,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "schedulingConstraints" => Field::Key_scheduling_constraints,
                            "schedulingPolicy" => Field::Key_scheduling_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CompositePodGroupTemplate;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CompositePodGroupTemplate")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_composite_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::CompositePodGroupTemplate>> = None;
                let mut value_disruption_mode: Option<crate::api::scheduling::v1alpha3::CompositeDisruptionMode> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::PodGroupTemplate>> = None;
                let mut value_preemption_policy: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<std::string::String> = None;
                let mut value_scheduling_constraints: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints> = None;
                let mut value_scheduling_policy: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_composite_pod_group_templates => value_composite_pod_group_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_disruption_mode => value_disruption_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_group_templates => value_pod_group_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preemption_policy => value_preemption_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_constraints => value_scheduling_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_policy => value_scheduling_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CompositePodGroupTemplate {
                    composite_pod_group_templates: value_composite_pod_group_templates,
                    disruption_mode: value_disruption_mode,
                    name: value_name.unwrap_or_default(),
                    pod_group_templates: value_pod_group_templates,
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    scheduling_constraints: value_scheduling_constraints,
                    scheduling_policy: value_scheduling_policy.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CompositePodGroupTemplate",
            &[
                "compositePodGroupTemplates",
                "disruptionMode",
                "name",
                "podGroupTemplates",
                "preemptionPolicy",
                "priority",
                "priorityClassName",
                "schedulingConstraints",
                "schedulingPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CompositePodGroupTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CompositePodGroupTemplate",
            2 +
            self.composite_pod_group_templates.as_ref().map_or(0, |_| 1) +
            self.disruption_mode.as_ref().map_or(0, |_| 1) +
            self.pod_group_templates.as_ref().map_or(0, |_| 1) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.scheduling_constraints.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.composite_pod_group_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "compositePodGroupTemplates", value)?;
        }
        if let Some(value) = &self.disruption_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "disruptionMode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.pod_group_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroupTemplates", value)?;
        }
        if let Some(value) = &self.preemption_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preemptionPolicy", value)?;
        }
        if let Some(value) = &self.priority {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        if let Some(value) = &self.priority_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priorityClassName", value)?;
        }
        if let Some(value) = &self.scheduling_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingConstraints", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingPolicy", &self.scheduling_policy)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CompositePodGroupTemplate {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupTemplate".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CompositePodGroupTemplate represents a template for a CompositePodGroup with a scheduling policy.",
            "type": "object",
            "properties": {
                "compositePodGroupTemplates": {
                    "description": "compositePodGroupTemplates is the list of templates for children CompositePodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupTemplate>()),
                },
                "disruptionMode": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. One of Single, All. This field is immutable.".into());
                    schema_obj
                }),
                "name": {
                    "description": "name is a unique identifier for the CompositePodGroupTemplate within the Workload. It must be a DNS label. This field is required.",
                    "type": "string",
                },
                "podGroupTemplates": {
                    "description": "podGroupTemplates is the list of templates for children PodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha3::PodGroupTemplate>()),
                },
                "preemptionPolicy": {
                    "description": "preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.",
                    "type": "string",
                },
                "priority": {
                    "description": "priority is the value of priority of composite pod groups created from this template. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.",
                    "type": "integer",
                    "format": "int32",
                },
                "priorityClassName": {
                    "description": "priorityClassName indicates the priority that should be considered when scheduling a composite pod group created from this template. If no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, composite pod groups created from this template will have the priority set to zero. This field is immutable.",
                    "type": "string",
                },
                "schedulingConstraints": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroupTemplate. This field is immutable.".into());
                    schema_obj
                }),
                "schedulingPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingPolicy defines the scheduling policy for this template.".into());
                    schema_obj
                }),
            },
            "required": [
                "name",
                "schedulingPolicy",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CompositePodGroupTemplate {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupTemplate".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CompositePodGroupTemplate represents a template for a CompositePodGroup with a scheduling policy.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "compositePodGroupTemplates".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("compositePodGroupTemplates is the list of templates for children CompositePodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupTemplate>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "disruptionMode".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. One of Single, All. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name is a unique identifier for the CompositePodGroupTemplate within the Workload. It must be a DNS label. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podGroupTemplates".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("podGroupTemplates is the list of templates for children PodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1alpha3::PodGroupTemplate>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "preemptionPolicy".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priority".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("priority is the value of priority of composite pod groups created from this template. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priorityClassName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("priorityClassName indicates the priority that should be considered when scheduling a composite pod group created from this template. If no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, composite pod groups created from this template will have the priority set to zero. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "schedulingConstraints".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroupTemplate. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "schedulingPolicy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("schedulingPolicy defines the scheduling policy for this template.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "schedulingPolicy".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
