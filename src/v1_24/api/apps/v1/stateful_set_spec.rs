// Generated from definition io.k8s.api.apps.v1.StatefulSetSpec

/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.
    pub min_ready_seconds: Option<i32>,

    /// persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down. This requires the StatefulSetAutoDeletePVC feature gate to be enabled, which is alpha.  +optional
    pub persistent_volume_claim_retention_policy: Option<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy>,

    /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    ///
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

#[cfg(feature = "dsl")]
impl StatefulSetSpec  {
    /// Set [`Self::min_ready_seconds`]
    pub  fn min_ready_seconds_set(&mut self, min_ready_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.min_ready_seconds = min_ready_seconds.into(); self
    }

    pub  fn min_ready_seconds(&mut self) -> &mut i32 {
        if self.min_ready_seconds.is_none() { self.min_ready_seconds = Some(Default::default()) }
        self.min_ready_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::min_ready_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_ready_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.min_ready_seconds.is_none() { self.min_ready_seconds = Some(Default::default()) };
        func(self.min_ready_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::persistent_volume_claim_retention_policy`]
    pub  fn persistent_volume_claim_retention_policy_set(&mut self, persistent_volume_claim_retention_policy: impl Into<Option<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy>>) -> &mut Self {
        self.persistent_volume_claim_retention_policy = persistent_volume_claim_retention_policy.into(); self
    }

    pub  fn persistent_volume_claim_retention_policy(&mut self) -> &mut crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
        if self.persistent_volume_claim_retention_policy.is_none() { self.persistent_volume_claim_retention_policy = Some(Default::default()) }
        self.persistent_volume_claim_retention_policy.as_mut().unwrap()
    }

    /// Modify [`Self::persistent_volume_claim_retention_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn persistent_volume_claim_retention_policy_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy)) -> &mut Self {
        if self.persistent_volume_claim_retention_policy.is_none() { self.persistent_volume_claim_retention_policy = Some(Default::default()) };
        func(self.persistent_volume_claim_retention_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::pod_management_policy`]
    pub  fn pod_management_policy_set(&mut self, pod_management_policy: impl Into<Option<String>>) -> &mut Self {
        self.pod_management_policy = pod_management_policy.into(); self
    }

    pub  fn pod_management_policy(&mut self) -> &mut String {
        if self.pod_management_policy.is_none() { self.pod_management_policy = Some(Default::default()) }
        self.pod_management_policy.as_mut().unwrap()
    }

    /// Modify [`Self::pod_management_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pod_management_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.pod_management_policy.is_none() { self.pod_management_policy = Some(Default::default()) };
        func(self.pod_management_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::replicas`]
    pub  fn replicas_set(&mut self, replicas: impl Into<Option<i32>>) -> &mut Self {
        self.replicas = replicas.into(); self
    }

    pub  fn replicas(&mut self) -> &mut i32 {
        if self.replicas.is_none() { self.replicas = Some(Default::default()) }
        self.replicas.as_mut().unwrap()
    }

    /// Modify [`Self::replicas`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn replicas_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.replicas.is_none() { self.replicas = Some(Default::default()) };
        func(self.replicas.as_mut().unwrap()); self
    }


    /// Set [`Self::revision_history_limit`]
    pub  fn revision_history_limit_set(&mut self, revision_history_limit: impl Into<Option<i32>>) -> &mut Self {
        self.revision_history_limit = revision_history_limit.into(); self
    }

    pub  fn revision_history_limit(&mut self) -> &mut i32 {
        if self.revision_history_limit.is_none() { self.revision_history_limit = Some(Default::default()) }
        self.revision_history_limit.as_mut().unwrap()
    }

    /// Modify [`Self::revision_history_limit`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn revision_history_limit_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.revision_history_limit.is_none() { self.revision_history_limit = Some(Default::default()) };
        func(self.revision_history_limit.as_mut().unwrap()); self
    }


    /// Set [`Self::selector`]
    pub  fn selector_set(&mut self, selector: impl Into<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>) -> &mut Self {
        self.selector = selector.into(); self
    }

    pub  fn selector(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector {
        &mut self.selector
    }

    /// Modify [`Self::selector`] with a `func`
    pub  fn selector_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        func(&mut self.selector); self
    }


    /// Set [`Self::service_name`]
    pub  fn service_name_set(&mut self, service_name: impl Into<String>) -> &mut Self {
        self.service_name = service_name.into(); self
    }

    pub  fn service_name(&mut self) -> &mut String {
        &mut self.service_name
    }

    /// Modify [`Self::service_name`] with a `func`
    pub  fn service_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.service_name); self
    }


    /// Set [`Self::template`]
    pub  fn template_set(&mut self, template: impl Into<crate::api::core::v1::PodTemplateSpec>) -> &mut Self {
        self.template = template.into(); self
    }

    pub  fn template(&mut self) -> &mut crate::api::core::v1::PodTemplateSpec {
        &mut self.template
    }

    /// Modify [`Self::template`] with a `func`
    pub  fn template_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodTemplateSpec)) -> &mut Self {
        func(&mut self.template); self
    }


    /// Set [`Self::update_strategy`]
    pub  fn update_strategy_set(&mut self, update_strategy: impl Into<Option<crate::api::apps::v1::StatefulSetUpdateStrategy>>) -> &mut Self {
        self.update_strategy = update_strategy.into(); self
    }

    pub  fn update_strategy(&mut self) -> &mut crate::api::apps::v1::StatefulSetUpdateStrategy {
        if self.update_strategy.is_none() { self.update_strategy = Some(Default::default()) }
        self.update_strategy.as_mut().unwrap()
    }

    /// Modify [`Self::update_strategy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn update_strategy_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::StatefulSetUpdateStrategy)) -> &mut Self {
        if self.update_strategy.is_none() { self.update_strategy = Some(Default::default()) };
        func(self.update_strategy.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_claim_templates`]
    pub  fn volume_claim_templates_set(&mut self, volume_claim_templates: impl Into<Option<Vec<crate::api::core::v1::PersistentVolumeClaim>>>) -> &mut Self {
        self.volume_claim_templates = volume_claim_templates.into(); self
    }

    pub  fn volume_claim_templates(&mut self) -> &mut Vec<crate::api::core::v1::PersistentVolumeClaim> {
        if self.volume_claim_templates.is_none() { self.volume_claim_templates = Some(Default::default()) }
        self.volume_claim_templates.as_mut().unwrap()
    }

    /// Modify [`Self::volume_claim_templates`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_claim_templates_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PersistentVolumeClaim>)) -> &mut Self {
        if self.volume_claim_templates.is_none() { self.volume_claim_templates = Some(Default::default()) };
        func(self.volume_claim_templates.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volume_claim_templates`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volume_claim_templates_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PersistentVolumeClaim)) -> &mut Self {
        if self.volume_claim_templates.is_none() {
            self.volume_claim_templates = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volume_claim_templates.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volume_claim_templates`]
    pub  fn volume_claim_templates_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PersistentVolumeClaim]>) -> &mut Self {
         if self.volume_claim_templates.is_none() { self.volume_claim_templates = Some(Vec::new()); }
         let volume_claim_templates = &mut self.volume_claim_templates.as_mut().unwrap();
         for item in other.borrow() {
             volume_claim_templates.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StatefulSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_persistent_volume_claim_retention_policy,
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

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "persistentVolumeClaimRetentionPolicy" => Field::Key_persistent_volume_claim_retention_policy,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatefulSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_persistent_volume_claim_retention_policy: Option<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy> = None;
                let mut value_pod_management_policy: Option<String> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_service_name: Option<String> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;
                let mut value_update_strategy: Option<crate::api::apps::v1::StatefulSetUpdateStrategy> = None;
                let mut value_volume_claim_templates: Option<Vec<crate::api::core::v1::PersistentVolumeClaim>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_claim_retention_policy => value_persistent_volume_claim_retention_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_management_policy => value_pod_management_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_name => value_service_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_strategy => value_update_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_claim_templates => value_volume_claim_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    persistent_volume_claim_retention_policy: value_persistent_volume_claim_retention_policy,
                    pod_management_policy: value_pod_management_policy,
                    replicas: value_replicas,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector.unwrap_or_default(),
                    service_name: value_service_name.unwrap_or_default(),
                    template: value_template.unwrap_or_default(),
                    update_strategy: value_update_strategy,
                    volume_claim_templates: value_volume_claim_templates,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetSpec",
            &[
                "minReadySeconds",
                "persistentVolumeClaimRetentionPolicy",
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

impl crate::serde::Serialize for StatefulSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetSpec",
            3 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_claim_retention_policy.as_ref().map_or(0, |_| 1) +
            self.pod_management_policy.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.update_strategy.as_ref().map_or(0, |_| 1) +
            self.volume_claim_templates.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.persistent_volume_claim_retention_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeClaimRetentionPolicy", value)?;
        }
        if let Some(value) = &self.pod_management_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podManagementPolicy", value)?;
        }
        if let Some(value) = &self.replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceName", &self.service_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.update_strategy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updateStrategy", value)?;
        }
        if let Some(value) = &self.volume_claim_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeClaimTemplates", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetSpec {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.StatefulSetSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("A StatefulSetSpec is the specification of a StatefulSet.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "minReadySeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "persistentVolumeClaimRetentionPolicy".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down. This requires the StatefulSetAutoDeletePVC feature gate to be enabled, which is alpha.  +optional".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "podManagementPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "replicas".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "revisionHistoryLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "serviceName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where \"pod-specific-string\" is managed by the StatefulSet controller.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "template".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "updateStrategy".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::StatefulSetUpdateStrategy>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumeClaimTemplates".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaim>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "selector".to_owned(),
                    "serviceName".to_owned(),
                    "template".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
