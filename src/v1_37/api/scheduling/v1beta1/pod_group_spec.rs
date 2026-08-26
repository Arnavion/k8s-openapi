// Generated from definition io.k8s.api.scheduling.v1beta1.PodGroupSpec

/// PodGroupSpec defines the desired state of a PodGroup.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupSpec {
    /// disruptionMode defines the mode in which a given PodGroup can be disrupted. Controllers are expected to fill this field by copying it from a PodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.
    pub disruption_mode: Option<crate::api::scheduling::v1beta1::DisruptionMode>,

    /// parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this pod group. If it's nil, then this pod group is a root of a workload's hierarchy. This field is used only when the CompositePodGroup feature gate is enabled. This field is immutable.
    pub parent_composite_pod_group_name: Option<std::string::String>,

    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    pub preemption_policy: Option<std::string::String>,

    /// priority is the value of priority of this pod group. Various system components use this field to find the priority of the pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.
    pub priority: Option<i32>,

    /// priorityClassName defines the priority that should be considered when scheduling this pod group. Controllers are expected to fill this field by copying it from a PodGroupTemplate. Otherwise, it is validated and resolved similarly to the PriorityClassName on PodGroupTemplate (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the pod group's priority will be zero). This field is immutable.
    pub priority_class_name: Option<std::string::String>,

    /// resourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.
    ///
    /// This is a beta-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.
    ///
    /// This field is immutable.
    pub resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1beta1::PodGroupResourceClaim>>,

    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled.
    pub scheduling_constraints: Option<crate::api::scheduling::v1beta1::PodGroupSchedulingConstraints>,

    /// schedulingPolicy defines the scheduling policy for this instance of the PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate.
    pub scheduling_policy: crate::api::scheduling::v1beta1::PodGroupSchedulingPolicy,

    /// workloadRef references an optional PodGroup template within the Workload object that was used to create the PodGroup. This field is immutable.
    pub workload_ref: Option<crate::api::scheduling::v1beta1::WorkloadReference>,
}

impl crate::DeepMerge for PodGroupSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.disruption_mode, other.disruption_mode);
        crate::DeepMerge::merge_from(&mut self.parent_composite_pod_group_name, other.parent_composite_pod_group_name);
        crate::DeepMerge::merge_from(&mut self.preemption_policy, other.preemption_policy);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
        crate::DeepMerge::merge_from(&mut self.priority_class_name, other.priority_class_name);
        crate::merge_strategies::list::map(
            &mut self.resource_claims,
            other.resource_claims,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.scheduling_constraints, other.scheduling_constraints);
        crate::DeepMerge::merge_from(&mut self.scheduling_policy, other.scheduling_policy);
        crate::DeepMerge::merge_from(&mut self.workload_ref, other.workload_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_disruption_mode,
            Key_parent_composite_pod_group_name,
            Key_preemption_policy,
            Key_priority,
            Key_priority_class_name,
            Key_resource_claims,
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
                            "resourceClaims" => Field::Key_resource_claims,
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
            type Value = PodGroupSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_disruption_mode: Option<crate::api::scheduling::v1beta1::DisruptionMode> = None;
                let mut value_parent_composite_pod_group_name: Option<std::string::String> = None;
                let mut value_preemption_policy: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<std::string::String> = None;
                let mut value_resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1beta1::PodGroupResourceClaim>> = None;
                let mut value_scheduling_constraints: Option<crate::api::scheduling::v1beta1::PodGroupSchedulingConstraints> = None;
                let mut value_scheduling_policy: Option<crate::api::scheduling::v1beta1::PodGroupSchedulingPolicy> = None;
                let mut value_workload_ref: Option<crate::api::scheduling::v1beta1::WorkloadReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_disruption_mode => value_disruption_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent_composite_pod_group_name => value_parent_composite_pod_group_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preemption_policy => value_preemption_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claims => value_resource_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_constraints => value_scheduling_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_policy => value_scheduling_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_workload_ref => value_workload_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupSpec {
                    disruption_mode: value_disruption_mode,
                    parent_composite_pod_group_name: value_parent_composite_pod_group_name,
                    preemption_policy: value_preemption_policy,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    resource_claims: value_resource_claims,
                    scheduling_constraints: value_scheduling_constraints,
                    scheduling_policy: value_scheduling_policy.unwrap_or_default(),
                    workload_ref: value_workload_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupSpec",
            &[
                "disruptionMode",
                "parentCompositePodGroupName",
                "preemptionPolicy",
                "priority",
                "priorityClassName",
                "resourceClaims",
                "schedulingConstraints",
                "schedulingPolicy",
                "workloadRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodGroupSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodGroupSpec",
            1 +
            self.disruption_mode.as_ref().map_or(0, |_| 1) +
            self.parent_composite_pod_group_name.as_ref().map_or(0, |_| 1) +
            self.preemption_policy.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.resource_claims.as_ref().map_or(0, |_| 1) +
            self.scheduling_constraints.as_ref().map_or(0, |_| 1) +
            self.workload_ref.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.resource_claims {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaims", value)?;
        }
        if let Some(value) = &self.scheduling_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingConstraints", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingPolicy", &self.scheduling_policy)?;
        if let Some(value) = &self.workload_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workloadRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1beta1.PodGroupSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupSpec defines the desired state of a PodGroup.",
            "type": "object",
            "properties": {
                "disruptionMode": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::DisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "disruptionMode defines the mode in which a given PodGroup can be disrupted. Controllers are expected to fill this field by copying it from a PodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.".into());
                    schema_obj
                }),
                "parentCompositePodGroupName": {
                    "description": "parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this pod group. If it's nil, then this pod group is a root of a workload's hierarchy. This field is used only when the CompositePodGroup feature gate is enabled. This field is immutable.",
                    "type": "string",
                },
                "preemptionPolicy": {
                    "description": "preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.",
                    "type": "string",
                },
                "priority": {
                    "description": "priority is the value of priority of this pod group. Various system components use this field to find the priority of the pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.",
                    "type": "integer",
                    "format": "int32",
                },
                "priorityClassName": {
                    "description": "priorityClassName defines the priority that should be considered when scheduling this pod group. Controllers are expected to fill this field by copying it from a PodGroupTemplate. Otherwise, it is validated and resolved similarly to the PriorityClassName on PodGroupTemplate (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the pod group's priority will be zero). This field is immutable.",
                    "type": "string",
                },
                "resourceClaims": {
                    "description": "resourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.\n\nThis is a beta-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.\n\nThis field is immutable.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupResourceClaim>()),
                },
                "schedulingConstraints": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupSchedulingConstraints>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled.".into());
                    schema_obj
                }),
                "schedulingPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "schedulingPolicy defines the scheduling policy for this instance of the PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate.".into());
                    schema_obj
                }),
                "workloadRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::WorkloadReference>();
                    schema_obj.ensure_object().insert("description".into(), "workloadRef references an optional PodGroup template within the Workload object that was used to create the PodGroup. This field is immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "schedulingPolicy",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for PodGroupSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1beta1.PodGroupSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("PodGroupSpec defines the desired state of a PodGroup.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "disruptionMode".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::DisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("disruptionMode defines the mode in which a given PodGroup can be disrupted. Controllers are expected to fill this field by copying it from a PodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "parentCompositePodGroupName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this pod group. If it's nil, then this pod group is a root of a workload's hierarchy. This field is used only when the CompositePodGroup feature gate is enabled. This field is immutable.".into()),
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
                                description: Some("priority is the value of priority of this pod group. Various system components use this field to find the priority of the pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.".into()),
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
                                description: Some("priorityClassName defines the priority that should be considered when scheduling this pod group. Controllers are expected to fill this field by copying it from a PodGroupTemplate. Otherwise, it is validated and resolved similarly to the PriorityClassName on PodGroupTemplate (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the pod group's priority will be zero). This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceClaims".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("resourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.\n\nThis is a beta-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.\n\nThis field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupResourceClaim>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "schedulingConstraints".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupSchedulingConstraints>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("schedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "schedulingPolicy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupSchedulingPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("schedulingPolicy defines the scheduling policy for this instance of the PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "workloadRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::WorkloadReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("workloadRef references an optional PodGroup template within the Workload object that was used to create the PodGroup. This field is immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "schedulingPolicy".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
