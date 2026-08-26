// Generated from definition io.k8s.api.scheduling.v1alpha3.CompositePodGroupSpec

/// CompositePodGroupSpec defines the desired state of CompositePodGroup.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompositePodGroupSpec {
    /// disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.
    pub disruption_mode: Option<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>,

    /// parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this composite pod group. It must be a DNS name. If it's nil, then this composite pod group is a root of a workload's hierarchy. This field is immutable.
    pub parent_composite_pod_group_name: Option<std::string::String>,

    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    pub preemption_policy: Option<std::string::String>,

    /// priority is the value of priority of this composite pod group. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.
    pub priority: Option<i32>,

    /// priorityClassName defines the priority that should be considered when scheduling this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. If left unspecified, it is validated and resolved similarly to the PriorityClassName field in Pods (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the composite pod group's priority will be zero). This field is immutable.
    pub priority_class_name: Option<std::string::String>,

    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.
    pub scheduling_constraints: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints>,

    /// schedulingPolicy defines the scheduling policy for this instance of the CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.
    pub scheduling_policy: crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy,

    /// workloadRef references an optional CompositePodGroup template within the Workload object that was used to create the CompositePodGroup. This field is required. This field is immutable.
    pub workload_ref: crate::api::scheduling::v1alpha3::WorkloadReference,
}

impl crate::DeepMerge for CompositePodGroupSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.disruption_mode, other.disruption_mode);
        crate::DeepMerge::merge_from(&mut self.parent_composite_pod_group_name, other.parent_composite_pod_group_name);
        crate::DeepMerge::merge_from(&mut self.preemption_policy, other.preemption_policy);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
        crate::DeepMerge::merge_from(&mut self.priority_class_name, other.priority_class_name);
        crate::DeepMerge::merge_from(&mut self.scheduling_constraints, other.scheduling_constraints);
        crate::DeepMerge::merge_from(&mut self.scheduling_policy, other.scheduling_policy);
        crate::DeepMerge::merge_from(&mut self.workload_ref, other.workload_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CompositePodGroupSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_disruption_mode,
            Key_parent_composite_pod_group_name,
            Key_preemption_policy,
            Key_priority,
            Key_priority_class_name,
            Key_scheduling_constraints,
            Key_scheduling_policy,
            Key_workload_ref,
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
                            "disruptionMode" => Field::Key_disruption_mode,
                            "parentCompositePodGroupName" => Field::Key_parent_composite_pod_group_name,
                            "preemptionPolicy" => Field::Key_preemption_policy,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "schedulingConstraints" => Field::Key_scheduling_constraints,
                            "schedulingPolicy" => Field::Key_scheduling_policy,
                            "workloadRef" => Field::Key_workload_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CompositePodGroupSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CompositePodGroupSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_disruption_mode: Option<crate::api::scheduling::v1alpha3::CompositeDisruptionMode> = None;
                let mut value_parent_composite_pod_group_name: Option<std::string::String> = None;
                let mut value_preemption_policy: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<std::string::String> = None;
                let mut value_scheduling_constraints: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints> = None;
                let mut value_scheduling_policy: Option<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy> = None;
                let mut value_workload_ref: Option<crate::api::scheduling::v1alpha3::WorkloadReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_disruption_mode => value_disruption_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent_composite_pod_group_name => value_parent_composite_pod_group_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preemption_policy => value_preemption_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_constraints => value_scheduling_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_policy => value_scheduling_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_workload_ref => value_workload_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CompositePodGroupSpec {
                    disruption_mode: value_disruption_mode,
                    parent_composite_pod_group_name: value_parent_composite_pod_group_name,
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    scheduling_constraints: value_scheduling_constraints,
                    scheduling_policy: value_scheduling_policy.unwrap_or_default(),
                    workload_ref: value_workload_ref.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CompositePodGroupSpec",
            &[
                "disruptionMode",
                "parentCompositePodGroupName",
                "preemptionPolicy",
                "priority",
                "priorityClassName",
                "schedulingConstraints",
                "schedulingPolicy",
                "workloadRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CompositePodGroupSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CompositePodGroupSpec",
            2 +
            self.disruption_mode.as_ref().map_or(0, |_| 1) +
            self.parent_composite_pod_group_name.as_ref().map_or(0, |_| 1) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.scheduling_constraints.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.disruption_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "disruptionMode", value)?;
        }
        if let Some(value) = &self.parent_composite_pod_group_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parentCompositePodGroupName", value)?;
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
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workloadRef", &self.workload_ref)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CompositePodGroupSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CompositePodGroupSpec defines the desired state of CompositePodGroup.",
            "type": "object",
            "properties": {
                "disruptionMode": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.".into());
                    schema_obj
                }),
                "parentCompositePodGroupName": {
                    "description": "parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this composite pod group. It must be a DNS name. If it's nil, then this composite pod group is a root of a workload's hierarchy. This field is immutable.",
                    "type": "string",
                },
                "preemptionPolicy": {
                    "description": "preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.",
                    "type": "string",
                },
                "priority": {
                    "description": "priority is the value of priority of this composite pod group. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.",
                    "type": "integer",
                    "format": "int32",
                },
                "priorityClassName": {
                    "description": "priorityClassName defines the priority that should be considered when scheduling this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. If left unspecified, it is validated and resolved similarly to the PriorityClassName field in Pods (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the composite pod group's priority will be zero). This field is immutable.",
                    "type": "string",
                },
                "schedulingConstraints": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.".into());
                    schema_obj
                }),
                "schedulingPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingPolicy defines the scheduling policy for this instance of the CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.".into());
                    schema_obj
                }),
                "workloadRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadReference>();
                    schema_obj.ensure_object().insert("description".into(), "workloadRef references an optional CompositePodGroup template within the Workload object that was used to create the CompositePodGroup. This field is required. This field is immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "schedulingPolicy",
                "workloadRef",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for CompositePodGroupSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.CompositePodGroupSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("CompositePodGroupSpec defines the desired state of CompositePodGroup.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "disruptionMode".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::CompositeDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "parentCompositePodGroupName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this composite pod group. It must be a DNS name. If it's nil, then this composite pod group is a root of a workload's hierarchy. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "preemptionPolicy".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.".into()),
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
                                description: Some("priority is the value of priority of this composite pod group. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.".into()),
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
                                description: Some("priorityClassName defines the priority that should be considered when scheduling this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. If left unspecified, it is validated and resolved similarly to the PriorityClassName field in Pods (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the composite pod group's priority will be zero). This field is immutable.".into()),
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
                                description: Some("schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.".into()),
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
                                description: Some("schedulingPolicy defines the scheduling policy for this instance of the CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "workloadRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("workloadRef references an optional CompositePodGroup template within the Workload object that was used to create the CompositePodGroup. This field is required. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "schedulingPolicy".into(),
                    "workloadRef".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
