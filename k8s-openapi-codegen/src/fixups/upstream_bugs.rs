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

// `StatefulSetSpec::volumeClaimTemplates` should be a map list keyed by `metadata.name`.
//
// The upstream Go source annotates `StatefulSetSpec.volumeClaimTemplates` with `// +listType=atomic`,
// which propagates to `x-kubernetes-list-type: atomic` in the OpenAPI spec. Semantically it should
// be a map list keyed by `metadata.name`, because each PVC in a StatefulSet has a unique name and
// merging by identity is the correct behavior.
//
// One possible reason this is not fixed upstream yet could be a limitation of the Kubernetes
// annotation system: `// +listMapKey=<field>` only supports direct scalar fields on the item type,
// but `PersistentVolumeClaim` carries its name via an embedded `ObjectMeta` (i.e. `metadata.name`
// in JSON), not as a top-level field. There is no existing precedent or tooling support for a
// nested `listMapKey` path in the Kubernetes codebase.
//
// We therefore fix it here by overriding the merge type after parsing the spec.
//
// Ref: https://github.com/kubernetes/kubernetes/issues/74819
pub(crate) fn appsv1_statefulsetspec_volume_claim_templates_merge_strategy(
    spec: &mut crate::swagger20::Spec,
) -> Result<(), crate::Error> {
    let definition_path =
    crate::swagger20::DefinitionPath("io.k8s.api.apps.v1.StatefulSetSpec".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
            if let Some((property_schema, _required)) = properties.get_mut("volumeClaimTemplates") {
                if let crate::swagger20::MergeType::List { strategy, keys, .. } =
                    &mut property_schema.merge_type
                    {
                        if *strategy == crate::swagger20::KubernetesListType::Atomic && keys.is_empty()
                        {
                            *strategy = crate::swagger20::KubernetesListType::Map;
                            *keys = vec!["metadata.name".to_string()];
                            return Ok(());
                        }
                    }
            }
        }
    }

    Err(
        "never applied apps.k8s.io/v1.StatefulSetSpec volumeClaimTemplates list merge override"
        .into(),
    )
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
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

    // `NetworkPolicySpec::podSelector`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/131354
    pub(crate) fn networkingv1_networkpolicyspec(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.networking.v1.NetworkPolicySpec".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("podSelector") {
                    if property.1 {
                        property.1 = false;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied networking.k8s.io/v1.NetworkPolicySpec optional properties override".into())
    }

    // `StatefulSetSpec::serviceName`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/130233
    pub(crate) fn appsv1_statefulsetspec(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.apps.v1.StatefulSetSpec".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("serviceName") {
                    if property.1 {
                        property.1 = false;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied apps.k8s.io/v1.StatefulSetSpec optional properties override".into())
    }
}

// The spec says that this property is optional, but it's required.
//
// Override it to be required.
pub(crate) mod required_properties {
    // `ConfigMapEnvSource::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn config_map_env_source(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ConfigMapEnvSource".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied ConfigMapEnvSource required properties override".into())
    }

    // `ConfigMapKeySelector::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn config_map_key_selector(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ConfigMapKeySelector".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied ConfigMapKeySelector required properties override".into())
    }

    // `ConfigMapProjection::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn config_map_projection(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ConfigMapProjection".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied ConfigMapProjection required properties override".into())
    }

    // `ConfigMapVolumeSource::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn config_map_volume_source(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ConfigMapVolumeSource".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied ConfigMapVolumeSource required properties override".into())
    }

    // `LocalObjectReference::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn local_object_reference(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.LocalObjectReference".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied LocalObjectReference required properties override".into())
    }

    // `SecretEnvSource::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn secret_env_source(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.SecretEnvSource".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied SecretEnvSource required properties override".into())
    }

    // `SecretKeySelector::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn secret_key_selector(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.SecretKeySelector".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied SecretKeySelector required properties override".into())
    }

    // `SecretProjection::name`
    //
    // Ref: https://github.com/kubernetes/kubernetes/pull/124694
    pub(crate) fn secret_projection(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.SecretProjection".to_owned());
        if let Some(definition) = spec.definitions.get_mut(&definition_path) {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                if let Some(property) = properties.get_mut("name") {
                    if !property.1 {
                        property.1 = true;
                        return Ok(());
                    }
                }
            }
        }

        Err("never applied SecretProjection required properties override".into())
    }
}
