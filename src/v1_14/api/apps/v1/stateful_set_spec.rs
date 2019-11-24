// Generated from definition io.k8s.api.apps.v1.StatefulSetSpec

/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetSpec {
    /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    pub pod_management_policy: Option<String>,

    /// replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
    pub replicas: Option<i32>,

    /// revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
    pub revision_history_limit: Option<i32>,

    /// selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where "pod-specific-string" is managed by the StatefulSet controller.
    pub service_name: String,

    /// template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet.
    pub template: crate::api::core::v1::PodTemplateSpec,

    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.
    pub update_strategy: Option<crate::api::apps::v1::StatefulSetUpdateStrategy>,

    /// volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
    pub volume_claim_templates: Option<Vec<crate::api::core::v1::PersistentVolumeClaim>>,
}

impl<'de> serde::Deserialize<'de> for StatefulSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_management_policy,
            Key_replicas,
            Key_revision_history_limit,
            Key_selector,
            Key_service_name,
            Key_template,
            Key_update_strategy,
            Key_volume_claim_templates,
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
                            "podManagementPolicy" => Field::Key_pod_management_policy,
                            "replicas" => Field::Key_replicas,
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "selector" => Field::Key_selector,
                            "serviceName" => Field::Key_service_name,
                            "template" => Field::Key_template,
                            "updateStrategy" => Field::Key_update_strategy,
                            "volumeClaimTemplates" => Field::Key_volume_claim_templates,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatefulSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_pod_management_policy: Option<String> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_service_name: Option<String> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;
                let mut value_update_strategy: Option<crate::api::apps::v1::StatefulSetUpdateStrategy> = None;
                let mut value_volume_claim_templates: Option<Vec<crate::api::core::v1::PersistentVolumeClaim>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_management_policy => value_pod_management_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_service_name => value_service_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_template => value_template = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_update_strategy => value_update_strategy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_claim_templates => value_volume_claim_templates = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetSpec {
                    pod_management_policy: value_pod_management_policy,
                    replicas: value_replicas,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector.ok_or_else(|| serde::de::Error::missing_field("selector"))?,
                    service_name: value_service_name.ok_or_else(|| serde::de::Error::missing_field("serviceName"))?,
                    template: value_template.ok_or_else(|| serde::de::Error::missing_field("template"))?,
                    update_strategy: value_update_strategy,
                    volume_claim_templates: value_volume_claim_templates,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetSpec",
            &[
                "podManagementPolicy",
                "replicas",
                "revisionHistoryLimit",
                "selector",
                "serviceName",
                "template",
                "updateStrategy",
                "volumeClaimTemplates",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for StatefulSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetSpec",
            3 +
            self.pod_management_policy.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.update_strategy.as_ref().map_or(0, |_| 1) +
            self.volume_claim_templates.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod_management_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podManagementPolicy", value)?;
        }
        if let Some(value) = &self.replicas {
            serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "serviceName", &self.service_name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.update_strategy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "updateStrategy", value)?;
        }
        if let Some(value) = &self.volume_claim_templates {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeClaimTemplates", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
