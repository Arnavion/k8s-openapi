// Generated from definition io.k8s.api.scheduling.v1alpha2.PodGroupSpec

/// PodGroupSpec defines the desired state of a PodGroup.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupSpec {
    /// DisruptionMode defines the mode in which a given PodGroup can be disrupted. Controllers are expected to fill this field by copying it from a PodGroupTemplate. One of Pod, PodGroup. Defaults to Pod if unset. This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.
    pub disruption_mode: Option<std::string::String>,

    /// PodGroupTemplateRef references an optional PodGroup template within other object (e.g. Workload) that was used to create the PodGroup. This field is immutable.
    pub pod_group_template_ref: Option<crate::api::scheduling::v1alpha2::PodGroupTemplateReference>,

    /// Priority is the value of priority of this pod group. Various system components use this field to find the priority of the pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.
    pub priority: Option<i32>,

    /// PriorityClassName defines the priority that should be considered when scheduling this pod group. Controllers are expected to fill this field by copying it from a PodGroupTemplate. Otherwise, it is validated and resolved similarly to the PriorityClassName on PodGroupTemplate (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the pod group's priority will be zero). This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.
    pub priority_class_name: Option<std::string::String>,

    /// ResourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.
    ///
    /// This is an alpha-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.
    ///
    /// This field is immutable.
    pub resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1alpha2::PodGroupResourceClaim>>,

    /// SchedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled.
    pub scheduling_constraints: Option<crate::api::scheduling::v1alpha2::PodGroupSchedulingConstraints>,

    /// SchedulingPolicy defines the scheduling policy for this instance of the PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable.
    pub scheduling_policy: crate::api::scheduling::v1alpha2::PodGroupSchedulingPolicy,
}

impl crate::DeepMerge for PodGroupSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.disruption_mode, other.disruption_mode);
        crate::DeepMerge::merge_from(&mut self.pod_group_template_ref, other.pod_group_template_ref);
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
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_disruption_mode,
            Key_pod_group_template_ref,
            Key_priority,
            Key_priority_class_name,
            Key_resource_claims,
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
                            "disruptionMode" => Field::Key_disruption_mode,
                            "podGroupTemplateRef" => Field::Key_pod_group_template_ref,
                            "priority" => Field::Key_priority,
                            "priorityClassName" => Field::Key_priority_class_name,
                            "resourceClaims" => Field::Key_resource_claims,
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
            type Value = PodGroupSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_disruption_mode: Option<std::string::String> = None;
                let mut value_pod_group_template_ref: Option<crate::api::scheduling::v1alpha2::PodGroupTemplateReference> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_priority_class_name: Option<std::string::String> = None;
                let mut value_resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1alpha2::PodGroupResourceClaim>> = None;
                let mut value_scheduling_constraints: Option<crate::api::scheduling::v1alpha2::PodGroupSchedulingConstraints> = None;
                let mut value_scheduling_policy: Option<crate::api::scheduling::v1alpha2::PodGroupSchedulingPolicy> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_disruption_mode => value_disruption_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_group_template_ref => value_pod_group_template_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_class_name => value_priority_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claims => value_resource_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_constraints => value_scheduling_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_policy => value_scheduling_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupSpec {
                    disruption_mode: value_disruption_mode,
                    pod_group_template_ref: value_pod_group_template_ref,
                    priority: value_priority,
                    priority_class_name: value_priority_class_name,
                    resource_claims: value_resource_claims,
                    scheduling_constraints: value_scheduling_constraints,
                    scheduling_policy: value_scheduling_policy.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupSpec",
            &[
                "disruptionMode",
                "podGroupTemplateRef",
                "priority",
                "priorityClassName",
                "resourceClaims",
                "schedulingConstraints",
                "schedulingPolicy",
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
            self.pod_group_template_ref.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1) +
            self.priority_class_name.as_ref().map_or(0, |_| 1) +
            self.resource_claims.as_ref().map_or(0, |_| 1) +
            self.scheduling_constraints.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.disruption_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "disruptionMode", value)?;
        }
        if let Some(value) = &self.pod_group_template_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroupTemplateRef", value)?;
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
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha2.PodGroupSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupSpec defines the desired state of a PodGroup.",
            "type": "object",
            "properties": {
                "disruptionMode": {
                    "description": "DisruptionMode defines the mode in which a given PodGroup can be disrupted. Controllers are expected to fill this field by copying it from a PodGroupTemplate. One of Pod, PodGroup. Defaults to Pod if unset. This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.",
                    "type": "string",
                },
                "podGroupTemplateRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupTemplateReference>();
                    schema_obj.ensure_object().insert("description".into(), "PodGroupTemplateRef references an optional PodGroup template within other object (e.g. Workload) that was used to create the PodGroup. This field is immutable.".into());
                    schema_obj
                }),
                "priority": {
                    "description": "Priority is the value of priority of this pod group. Various system components use this field to find the priority of the pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.",
                    "type": "integer",
                    "format": "int32",
                },
                "priorityClassName": {
                    "description": "PriorityClassName defines the priority that should be considered when scheduling this pod group. Controllers are expected to fill this field by copying it from a PodGroupTemplate. Otherwise, it is validated and resolved similarly to the PriorityClassName on PodGroupTemplate (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the pod group's priority will be zero). This field is immutable. This field is available only when the WorkloadAwarePreemption feature gate is enabled.",
                    "type": "string",
                },
                "resourceClaims": {
                    "description": "ResourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.\n\nThis is an alpha-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.\n\nThis field is immutable.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupResourceClaim>()),
                },
                "schedulingConstraints": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupSchedulingConstraints>();
                    schema_obj.ensure_object().insert("description".into(), "SchedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled.".into());
                    schema_obj
                }),
                "schedulingPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::PodGroupSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "SchedulingPolicy defines the scheduling policy for this instance of the PodGroup. Controllers are expected to fill this field by copying it from a PodGroupTemplate. This field is immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "schedulingPolicy",
            ],
        })
    }
}
