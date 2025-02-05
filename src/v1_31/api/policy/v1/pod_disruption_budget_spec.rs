// Generated from definition io.k8s.api.policy.v1.PodDisruptionBudgetSpec

/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at most "maxUnavailable" pods selected by "selector" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with "minAvailable".
    pub max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// An eviction is allowed if at least "minAvailable" pods selected by "selector" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying "100%".
    pub min_available: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// Label query over pods whose evictions are managed by the disruption budget. A null selector will match no pods, while an empty ({}) selector will select all pods within the namespace.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type="Ready",status="True".
    ///
    /// Valid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.
    ///
    /// IfHealthyBudget policy means that running pods (status.phase="Running"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.
    ///
    /// AlwaysAllow policy means that all running pods (status.phase="Running"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.
    ///
    /// Additional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.
    ///
    /// This field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).
    pub unhealthy_pod_eviction_policy: Option<std::string::String>,
}

impl crate::DeepMerge for PodDisruptionBudgetSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.max_unavailable, other.max_unavailable);
        crate::DeepMerge::merge_from(&mut self.min_available, other.min_available);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.unhealthy_pod_eviction_policy, other.unhealthy_pod_eviction_policy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodDisruptionBudgetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_unavailable,
            Key_min_available,
            Key_selector,
            Key_unhealthy_pod_eviction_policy,
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
                            "maxUnavailable" => Field::Key_max_unavailable,
                            "minAvailable" => Field::Key_min_available,
                            "selector" => Field::Key_selector,
                            "unhealthyPodEvictionPolicy" => Field::Key_unhealthy_pod_eviction_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodDisruptionBudgetSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodDisruptionBudgetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_min_available: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_unhealthy_pod_eviction_policy: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_unavailable => value_max_unavailable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_available => value_min_available = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unhealthy_pod_eviction_policy => value_unhealthy_pod_eviction_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDisruptionBudgetSpec {
                    max_unavailable: value_max_unavailable,
                    min_available: value_min_available,
                    selector: value_selector,
                    unhealthy_pod_eviction_policy: value_unhealthy_pod_eviction_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDisruptionBudgetSpec",
            &[
                "maxUnavailable",
                "minAvailable",
                "selector",
                "unhealthyPodEvictionPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodDisruptionBudgetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudgetSpec",
            self.max_unavailable.as_ref().map_or(0, |_| 1) +
            self.min_available.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.unhealthy_pod_eviction_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_unavailable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        if let Some(value) = &self.min_available {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minAvailable", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.unhealthy_pod_eviction_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unhealthyPodEvictionPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodDisruptionBudgetSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.policy.v1.PodDisruptionBudgetSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "maxUnavailable".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\".".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "minAvailable".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\".".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "selector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Label query over pods whose evictions are managed by the disruption budget. A null selector will match no pods, while an empty ({}) selector will select all pods within the namespace.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "unhealthyPodEvictionPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type=\"Ready\",status=\"True\".\n\nValid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.\n\nIfHealthyBudget policy means that running pods (status.phase=\"Running\"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.\n\nAlwaysAllow policy means that all running pods (status.phase=\"Running\"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.\n\nAdditional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.\n\nThis field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
