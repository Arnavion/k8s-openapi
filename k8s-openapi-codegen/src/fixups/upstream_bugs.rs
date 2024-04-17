#![deny(unused)]

//! These fixups correspond to bugs in the upstream swagger spec.

// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/66807
#[allow(clippy::if_same_then_else)]
pub(crate) fn connect_options_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let mut found = false;

    for operation in &mut spec.operations {
        if let Some(kubernetes_group_kind_version) = &mut operation.kubernetes_group_kind_version {
            if kubernetes_group_kind_version.group.is_empty() && kubernetes_group_kind_version.version == "v1" {
                let kind = &mut kubernetes_group_kind_version.kind;
                if &*kind == "NodeProxyOptions" {
                    *kind = "Node".to_string();
                    found = true;
                }
                else if &*kind == "PodAttachOptions" {
                    *kind = "Pod".to_string();
                    found = true;
                }
                else if &*kind == "PodExecOptions" {
                    *kind = "Pod".to_string();
                    found = true;
                }
                else if &*kind == "PodPortForwardOptions" {
                    *kind = "Pod".to_string();
                    found = true;
                }
                else if &*kind == "PodProxyOptions" {
                    *kind = "Pod".to_string();
                    found = true;
                }
                else if &*kind == "ServiceProxyOptions" {
                    *kind = "Service".to_string();
                    found = true;
                }
            }
        }
    }

    if found {
        Ok(())
    }
    else {
        Err("never applied connect options kubernetes_group_kind_version override".into())
    }
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
    // `Event::eventTime`
    pub(crate) fn eventsv1beta1_event(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.events.v1beta1.Event".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("eventTime") {
                    if property.1 {
                        property.1 = false;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied events.k8s.io/v1beta1.Event optional properties override".into())
    }

    // `Event::eventTime`
    pub(crate) fn eventsv1_event(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.events.v1.Event".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("eventTime") {
                    if property.1 {
                        property.1 = false;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied events.k8s.io/v1.Event optional properties override".into())
    }
}

// The spec says that this property is optional, but it's required.
//
// Override it to be required.
pub(crate) mod required_properties {
    // `ValidatingAdmissionPolicyBindingList::items`
    pub(crate) fn alpha1_validating_admission_policy_binding_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicyBindingList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1alpha1.ValidatingAdmissionPolicyBindingList required properties override".into())
    }

    // `ValidatingAdmissionPolicyList::items`
    pub(crate) fn alpha1_validating_admission_policy_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicyList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1alpha1.ValidatingAdmissionPolicyList required properties override".into())
    }

    // `ValidatingAdmissionPolicyBindingList::items`
    pub(crate) fn beta1_validating_admission_policy_binding_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1beta1.ValidatingAdmissionPolicyBindingList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1beta1.ValidatingAdmissionPolicyBindingList required properties override".into())
    }

    // `ValidatingAdmissionPolicyList::items`
    pub(crate) fn beta1_validating_admission_policy_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1beta1.ValidatingAdmissionPolicyList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1beta1.ValidatingAdmissionPolicyList required properties override".into())
    }

    // `ValidatingAdmissionPolicyBindingList::items`
    pub(crate) fn validating_admission_policy_binding_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicyBindingList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1.ValidatingAdmissionPolicyBindingList required properties override".into())
    }

    // `ValidatingAdmissionPolicyList::items`
    pub(crate) fn validating_admission_policy_list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicyList".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("items") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied admissionregistration.k8s.io/v1.ValidatingAdmissionPolicyList required properties override".into())
    }
}

// `Status` has extra group-version-kind entries than the original `"":v1:Status` that cause it to not be detected as a `Resource`.
// Remove the extras.
pub(crate) fn status_extra_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        if definition.kubernetes_group_kind_versions.len() > 1 {
            #[allow(clippy::comparison_to_empty)]
            definition.kubernetes_group_kind_versions.retain(|gvk| gvk.group == "" && gvk.kind == "Status" && gvk.version == "v1");
            if definition.kubernetes_group_kind_versions.len() == 1 {
                return Ok(());
            }

            return Err(format!(
                "Status extra group-version-kinds override did not retain the expected group-version-kind: {:?}",
                definition.kubernetes_group_kind_versions).into());
        }
    }

    Err("never applied Status extra group-version-kinds override".into())
}

// `PodSchedulingContextSpec` has `set` merge strategy in versions before v1.29
// but v1.29 decided it would like clients to treat it as `atomic`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/119962
pub(crate) fn pod_scheduling_context_spec_potential_nodes_merge_strategy(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.resource.v1alpha2.PodSchedulingContextSpec".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
            if let Some(property) = properties.get_mut("potentialNodes") {
                if let crate::swagger20::MergeType::List { strategy, .. } = &mut property.0.merge_type {
                    if *strategy == crate::swagger20::KubernetesListType::Set {
                        *strategy = crate::swagger20::KubernetesListType::Atomic;
                        return Ok(());
                    }
                }
            }
        }
    }

    Err("never applied PodSchedulingContextSpec.potentialNodes merge strategy override".into())
}

// `ResourceClaimSchedulingStatus` has `set` merge strategy in versions before v1.29
// but v1.29 decided it would like clients to treat it as `atomic`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/119962
pub(crate) fn v1alpha1_resource_claim_scheduling_status_unsuitable_nodes_merge_strategy(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.resource.v1alpha1.ResourceClaimSchedulingStatus".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
            if let Some(property) = properties.get_mut("unsuitableNodes") {
                if let crate::swagger20::MergeType::List { strategy, .. } = &mut property.0.merge_type {
                    if *strategy == crate::swagger20::KubernetesListType::Set {
                        *strategy = crate::swagger20::KubernetesListType::Atomic;
                        return Ok(());
                    }
                }
            }
        }
    }

    Err("never applied ResourceClaimSchedulingStatus.unsuitableNodes merge strategy override".into())
}

// `ResourceClaimSchedulingStatus` has `set` merge strategy in versions before v1.29
// but v1.29 decided it would like clients to treat it as `atomic`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/119962
pub(crate) fn v1alpha2_resource_claim_scheduling_status_unsuitable_nodes_merge_strategy(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.resource.v1alpha2.ResourceClaimSchedulingStatus".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
            if let Some(property) = properties.get_mut("unsuitableNodes") {
                if let crate::swagger20::MergeType::List { strategy, .. } = &mut property.0.merge_type {
                    if *strategy == crate::swagger20::KubernetesListType::Set {
                        *strategy = crate::swagger20::KubernetesListType::Atomic;
                        return Ok(());
                    }
                }
            }
        }
    }

    Err("never applied ResourceClaimSchedulingStatus.unsuitableNodes merge strategy override".into())
}
