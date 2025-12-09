#![deny(unused)]

//! These fixups are for special adjustments to the upstream swagger spec that are needed for the codegen of k8s-openapi.

// The composite JSONSchemaProps types need special codegen.
pub(crate) mod json_ty {
    pub(crate) fn json_schema_props_or_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let mut found = false;

        for namespace in ["v1beta1", "v1"] {
            let definition_path = crate::swagger20::DefinitionPath(format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{namespace}.JSONSchemaPropsOrArray"));

            if let Some(definition) = spec.definitions.get_mut(&definition_path) {
                if !matches!(
                    definition.kind,
                    crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(_, crate::swagger20::JsonSchemaPropsOr::Array)),
                ) {
                    definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(namespace, crate::swagger20::JsonSchemaPropsOr::Array));
                    found = true;
                }
            }
        }

        if found {
            Ok(())
        }
        else {
            Err("never applied JSONSchemaPropsOrArray override".into())
        }
    }

    pub(crate) fn json_schema_props_or_bool(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let mut found = false;

        for namespace in ["v1beta1", "v1"] {
            let definition_path = crate::swagger20::DefinitionPath(format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{namespace}.JSONSchemaPropsOrBool"));

            if let Some(definition) = spec.definitions.get_mut(&definition_path) {
                if !matches!(
                    definition.kind,
                    crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(_, crate::swagger20::JsonSchemaPropsOr::Bool)),
                ) {
                    definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(namespace, crate::swagger20::JsonSchemaPropsOr::Bool));
                    found = true;
                }
            }
        }

        if found {
            Ok(())
        }
        else {
            Err("never applied JSONSchemaPropsOrBool override".into())
        }
    }

    pub(crate) fn json_schema_props_or_string_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        let mut found = false;

        for namespace in ["v1beta1", "v1"] {
            let definition_path = crate::swagger20::DefinitionPath(format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{namespace}.JSONSchemaPropsOrStringArray"));

            if let Some(definition) = spec.definitions.get_mut(&definition_path) {
                if !matches!(
                    definition.kind,
                    crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(_, crate::swagger20::JsonSchemaPropsOr::StringArray)),
                ) {
                    definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JsonSchemaPropsOr(namespace, crate::swagger20::JsonSchemaPropsOr::StringArray));
                    found = true;
                }
            }
        }

        if found {
            Ok(())
        }
        else {
            Err("never applied JSONSchemaPropsOrStringArray override".into())
        }
    }
}

// Annotate the `patch` type as `swagger20::Type::Patch` for special codegen.
pub(crate) fn patch(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.apis.meta.v1.Patch".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Patch);
        return Ok(());
    }

    Err("never applied Patch override".into())
}

// Annotate the `WatchEvent` type as `swagger20::Type::WatchEvent` for special codegen.
pub(crate) fn watch_event(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    use std::fmt::Write;

    let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent".to_owned());
    if let Some(mut definition) = spec.definitions.remove(&definition_path) {
        if let crate::swagger20::SchemaKind::Properties(mut properties) = definition.kind {
            let object_property_name = crate::swagger20::PropertyName("object".to_owned());
            if let Some((object_property, true)) = properties.remove(&object_property_name) {
                if let crate::swagger20::SchemaKind::Ref(raw_extension_ref_path) = object_property.kind {
                    definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::WatchEvent(raw_extension_ref_path));
                    if let Some(type_description) = &mut definition.description {
                        if let Some(property_description) = &object_property.description {
                            writeln!(type_description)?;
                            writeln!(type_description)?;
                            writeln!(type_description, "{property_description}")?;
                        }
                    }
                    spec.definitions.insert(definition_path, definition);
                    return Ok(());
                }
            }
        }
    }

    Err("never applied WatchEvent override".into())
}

// Define the `swagger20::Type::ListDef` list type, and replace all list types in the spec with `swagger20::Type::ListRef` for special codegen.
pub(crate) fn list(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let items_property_name = crate::swagger20::PropertyName("items".to_owned());
    let metadata_property_name = crate::swagger20::PropertyName("metadata".to_owned());

    let mut list_definition_paths = vec![];
    let mut list_properties = None;

    for (definition_path, definition) in &spec.definitions {
        if !definition_path.ends_with("List") {
            continue;
        }

        let crate::swagger20::SchemaKind::Properties(properties) = &definition.kind else { continue; };

        if !matches!(properties.get(&items_property_name), Some((_, true))) {
            continue;
        }

        let Some((metadata_schema, _)) = properties.get(&metadata_property_name) else { continue; };

        let crate::swagger20::SchemaKind::Ref(metadata_ref_path) = &metadata_schema.kind else { continue; };
        if !metadata_ref_path.path.ends_with(".ListMeta") {
            continue;
        }

        let Some((
            crate::swagger20::Schema {
                kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Array { items: item_schema }),
                ..
            },
            true,
        )) = properties.get(&items_property_name) else {
            return Err(format!("definition {definition_path} looks like a list but doesn't have an items property").into());
        };

        let crate::swagger20::SchemaKind::Ref(item_ref_path) = &item_schema.kind else {
            return Err(format!("definition {definition_path} looks like a list but its items property is not a ref").into());
        };

        let item_schema =
            spec.definitions.get(&*item_ref_path.path)
            .ok_or_else(|| format!("definition {definition_path} looks like a list but its item's definition does not exist in the spec"))?;

        let [item_kubernetes_group_kind_version] = &item_schema.kubernetes_group_kind_versions[..] else {
            return Err(format!(
                "definition {definition_path} looks like a list but its item's definition does not have a single group-version-kind").into());
        };

        let [list_kubernetes_group_kind_version] = &definition.kubernetes_group_kind_versions[..] else {
            return Err(format!(
                "definition {definition_path} looks like a list but it does not have a single group-version-kind").into());
        };

        let item_gkv_corresponds_to_list_gkv =
            list_kubernetes_group_kind_version.group == item_kubernetes_group_kind_version.group &&
            list_kubernetes_group_kind_version.version == item_kubernetes_group_kind_version.version &&
            list_kubernetes_group_kind_version.kind == format!("{}List", item_kubernetes_group_kind_version.kind);
        if !item_gkv_corresponds_to_list_gkv {
            return Err(format!(
                "defintion {definition_path} looks like a list but its group-version-kind does not correspond to its item's group-version-kind").into());
        }

        let list_kind = list_kubernetes_group_kind_version.kind.clone();

        list_definition_paths.push((definition_path.clone(), item_ref_path.clone(), list_kind));

        if let Some((_, list_property_names)) = &list_properties {
            let property_names: std::collections::BTreeSet<_> = properties.keys().cloned().collect();
            if &property_names != list_property_names {
                return Err(format!("Definition {definition_path} looks like a list but doesn't have the expected properties: {properties:?}").into());
            }
        }
        else {
            let mut properties = properties.clone();
            properties.insert(
                items_property_name.clone(),
                (
                    crate::swagger20::Schema {
                        description: Some("List of objects".to_owned()),
                        kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Array {
                            items: Box::new(crate::swagger20::Schema {
                                description: None,
                                kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
                                    path: "T".to_owned(),
                                    can_be_default: None,
                                }),
                                kubernetes_group_kind_versions: vec![],
                                list_kind: None,
                                merge_type: crate::swagger20::MergeType::Default,
                                impl_deep_merge: true,
                            }),
                        }),
                        kubernetes_group_kind_versions: vec![],
                        list_kind: None,
                        merge_type: crate::swagger20::MergeType::Default,
                        impl_deep_merge: true,
                    },
                    true,
                ));
            let property_names: std::collections::BTreeSet<_> = properties.keys().cloned().collect();

            list_properties = Some((metadata_schema.kind.clone(), property_names));
        }
    }

    let (metadata_schema_kind, _) = list_properties.ok_or("did not find any types that looked like a list")?;


    // Synthesize `k8s_openapi::List<T>`

    spec.definitions.insert(
        crate::swagger20::DefinitionPath("io.k8s.List".to_owned()),
        crate::swagger20::Schema {
            description: Some("List is a list of resources.".to_owned()),
            kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListDef { metadata: Box::new(metadata_schema_kind) }),
            kubernetes_group_kind_versions: vec![],
            list_kind: None,
            merge_type: crate::swagger20::MergeType::Default,
            impl_deep_merge: true,
        });


    // Remove all list types

    for (definition_path, item_ref_path, list_kind) in &list_definition_paths {
        let item_definition = spec.definitions.get_mut(&*item_ref_path.path).unwrap();
        item_definition.list_kind = Some(list_kind.clone());

        let _ = spec.definitions.remove(definition_path);
    }


    // Replace references to all list types with refs to `k8s_openapi::List<T>`

    for (definition_path, item_ref_path, _) in list_definition_paths {
        for definition in spec.definitions.values_mut() {
            if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
                for (field_value_schema, _) in properties.values_mut() {
                    let field_value_schema_kind = &mut field_value_schema.kind;
                    if let crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }) = field_value_schema_kind {
                        if path == &definition_path.0 {
                            *field_value_schema_kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListRef {
                                items: Box::new(crate::swagger20::SchemaKind::Ref(item_ref_path.clone())),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// The `metadata` property of resource types is generally not optional, so override the property to be required.
// For situations like PATCH requests where the property *is* optional, sending an empty object works the same.
pub(crate) fn resource_metadata_not_optional(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let mut found = false;

    for definition in spec.definitions.values_mut() {
        if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
            let mut has_api_version = false;
            let mut has_kind = false;
            let mut has_optional_metadata = false;

            for (name, (_, required)) in &mut *properties {
                has_api_version = has_api_version || name.0 == "apiVersion";
                has_kind = has_kind || name.0 == "kind";
                has_optional_metadata = has_optional_metadata || (name.0 == "metadata" && !*required);
                if has_api_version && has_kind && has_optional_metadata {
                    break;
                }
            }

            if has_api_version && has_kind && has_optional_metadata {
                let metadata = properties.get_mut("metadata").unwrap();
                metadata.1 = true;
                found = true;
            }
        }
    }

    if found {
        Ok(())
    }
    else {
        Err("never applied override to make resource metadata non-optional".into())
    }
}

// The quantity type can be either an integer or string, even though the spec says it should just be a string.
pub(crate) fn quantity(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
    let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.api.resource.Quantity".to_owned());
    if let Some(definition) = spec.definitions.get_mut(&definition_path) {
        definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::IntOrString);
        return Ok(());
    }

    Err("never applied Quantity override".into())
}
