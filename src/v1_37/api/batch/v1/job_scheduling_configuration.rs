// Generated from definition io.k8s.api.batch.v1.JobSchedulingConfiguration

/// JobSchedulingConfiguration composes the reusable workload-aware scheduling building blocks.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobSchedulingConfiguration {
    /// DisruptionMode defines the mode in which the Job's pods can be disrupted. One of Single, All. This field is immutable after creation: it may not be added or removed, and the selected mode may not be changed.
    pub disruption_mode: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode>,

    /// ResourceClaims defines which ResourceClaims may be shared among Pods in the Job. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate. At most 4 claims may be set, matching the limit on the resulting PodGroup. This list is immutable after creation: entries may neither be added, removed, nor modified.
    pub resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim>>,

    /// SchedulingConstraints defines scheduling constraints (e.g. topology) for the Job's pods. This field is immutable after creation.
    pub scheduling_constraints: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints>,

    /// SchedulingPolicy defines the scheduling policy for this Job. Exactly one of Basic or Gang must be set. This field is immutable after creation: the policy may not be added or removed. The policy variant (basic/gang) is frozen by hand-written validation; only schedulingPolicy.gang.minCount may be changed.
    pub scheduling_policy: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy>,
}

impl crate::DeepMerge for JobSchedulingConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.disruption_mode, other.disruption_mode);
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

impl<'de> crate::serde::Deserialize<'de> for JobSchedulingConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_disruption_mode,
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
            type Value = JobSchedulingConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JobSchedulingConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_disruption_mode: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode> = None;
                let mut value_resource_claims: Option<std::vec::Vec<crate::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim>> = None;
                let mut value_scheduling_constraints: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints> = None;
                let mut value_scheduling_policy: Option<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_disruption_mode => value_disruption_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_claims => value_resource_claims = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_constraints => value_scheduling_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheduling_policy => value_scheduling_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JobSchedulingConfiguration {
                    disruption_mode: value_disruption_mode,
                    resource_claims: value_resource_claims,
                    scheduling_constraints: value_scheduling_constraints,
                    scheduling_policy: value_scheduling_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "JobSchedulingConfiguration",
            &[
                "disruptionMode",
                "resourceClaims",
                "schedulingConstraints",
                "schedulingPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JobSchedulingConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JobSchedulingConfiguration",
            self.disruption_mode.as_ref().map_or(0, |_| 1) +
            self.resource_claims.as_ref().map_or(0, |_| 1) +
            self.scheduling_constraints.as_ref().map_or(0, |_| 1) +
            self.scheduling_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.disruption_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "disruptionMode", value)?;
        }
        if let Some(value) = &self.resource_claims {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceClaims", value)?;
        }
        if let Some(value) = &self.scheduling_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingConstraints", value)?;
        }
        if let Some(value) = &self.scheduling_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schedulingPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JobSchedulingConfiguration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.batch.v1.JobSchedulingConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "JobSchedulingConfiguration composes the reusable workload-aware scheduling building blocks.",
            "type": "object",
            "properties": {
                "disruptionMode": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode>();
                    schema_obj.ensure_object().insert("description".into(), "DisruptionMode defines the mode in which the Job's pods can be disrupted. One of Single, All. This field is immutable after creation: it may not be added or removed, and the selected mode may not be changed.".into());
                    schema_obj
                }),
                "resourceClaims": {
                    "description": "ResourceClaims defines which ResourceClaims may be shared among Pods in the Job. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate. At most 4 claims may be set, matching the limit on the resulting PodGroup. This list is immutable after creation: entries may neither be added, removed, nor modified.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim>()),
                },
                "schedulingConstraints": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints>();
                    schema_obj.ensure_object().insert("description".into(), "SchedulingConstraints defines scheduling constraints (e.g. topology) for the Job's pods. This field is immutable after creation.".into());
                    schema_obj
                }),
                "schedulingPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "SchedulingPolicy defines the scheduling policy for this Job. Exactly one of Basic or Gang must be set. This field is immutable after creation: the policy may not be added or removed. The policy variant (basic/gang) is frozen by hand-written validation; only schedulingPolicy.gang.minCount may be changed.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for JobSchedulingConfiguration {
    fn schema_name() -> std::string::String {
        "io.k8s.api.batch.v1.JobSchedulingConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("JobSchedulingConfiguration composes the reusable workload-aware scheduling building blocks.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "disruptionMode".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("DisruptionMode defines the mode in which the Job's pods can be disrupted. One of Single, All. This field is immutable after creation: it may not be added or removed, and the selected mode may not be changed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceClaims".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("ResourceClaims defines which ResourceClaims may be shared among Pods in the Job. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate. At most 4 claims may be set, matching the limit on the resulting PodGroup. This list is immutable after creation: entries may neither be added, removed, nor modified.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars08::schema::ArrayValidation {
                                items: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "schedulingConstraints".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("SchedulingConstraints defines scheduling constraints (e.g. topology) for the Job's pods. This field is immutable after creation.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "schedulingPolicy".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("SchedulingPolicy defines the scheduling policy for this Job. Exactly one of Basic or Gang must be set. This field is immutable after creation: the policy may not be added or removed. The policy variant (basic/gang) is frozen by hand-written validation; only schedulingPolicy.gang.minCount may be changed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
