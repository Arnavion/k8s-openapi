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

// This fixup copies the `io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions` type to `io.k8s.DeleteOptional` and modifies its parameters to be optional borrows.
// This makes the new type consistent with `io.k8s.ListOptional` and `io.k8s.WatchOptional` and allows it to be used as a common parameter for
// delete and delete-collection API operations.
//
// The original `DeleteOptions` type is still kept since it's used as a field of `io.k8s.api.policy.v1beta1.Eviction`
pub(crate) fn create_delete_optional(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let delete_options_schema =
		spec.definitions.get("io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions")
		.ok_or("could not find io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions")?;
	let crate::swagger20::SchemaKind::Properties(delete_options_properties) = &delete_options_schema.kind else {
		return Err("io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions is not a SchemaKind::Properties".into());
	};
	let delete_optional_properties = delete_options_properties.iter().map(|(name, (schema, _))| (name.clone(), schema.clone())).collect();

	spec.definitions.insert(crate::swagger20::DefinitionPath("io.k8s.DeleteOptional".to_owned()), crate::swagger20::Schema {
		description: Some("Common parameters for all delete and delete-collection operations.".to_owned()),
		kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::DeleteOptional(delete_optional_properties)),
		kubernetes_group_kind_versions: vec![],
		list_kind: None,
		merge_type: crate::swagger20::MergeType::Default,
		impl_deep_merge: true,
	});

	Ok(())
}

// This fixup extracts the common optional parameters of some operations into common types.
pub(crate) fn create_optionals(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	#[allow(clippy::type_complexity)]
	const OPTIONALS: &[(
		&str,
		&str,
		crate::swagger20::KubernetesAction,
		fn(std::collections::BTreeMap<crate::swagger20::PropertyName, crate::swagger20::Schema>) -> crate::swagger20::Type,
	)] = &[
		("create", "io.k8s.CreateOptional", crate::swagger20::KubernetesAction::Post, crate::swagger20::Type::CreateOptional),
		("patch", "io.k8s.PatchOptional", crate::swagger20::KubernetesAction::Patch, crate::swagger20::Type::PatchOptional),
		("replace", "io.k8s.ReplaceOptional", crate::swagger20::KubernetesAction::Put, crate::swagger20::Type::ReplaceOptional),
	];

	for &(description, type_name, kubernetes_action, ty) in OPTIONALS {
		let mut optional_parameters: Option<std::collections::HashSet<String>> = None;
		let mut optional_definition: std::collections::BTreeMap<crate::swagger20::PropertyName, crate::swagger20::Schema> = Default::default();

		let optional_parameter = std::sync::Arc::new(crate::swagger20::Parameter {
			location: crate::swagger20::ParameterLocation::Query,
			name: "optional".to_owned(),
			required: true,
			schema: crate::swagger20::Schema {
				description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
				kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
					path: type_name.to_owned(),
					can_be_default: None,
				}),
				kubernetes_group_kind_versions: vec![],
				list_kind: None,
				merge_type: crate::swagger20::MergeType::Default,
				impl_deep_merge: true,
			},
		});

		for operation in &mut spec.operations {
			if operation.kubernetes_action != Some(kubernetes_action) {
				continue;
			}

			if !operation.id.starts_with(description) {
				return Err(format!(
					"operation {} has action {:?} but isn't a {description} operation",
					operation.id, operation.kubernetes_action).into());
			}

			{
				let optional_parameters =
					&*optional_parameters
					.get_or_insert_with(||
						operation.parameters.iter()
						.filter_map(|p| if p.required { None } else { Some(p.name.clone()) })
						.collect());

				for expected_parameter_name in optional_parameters {
					let expected_parameter =
						operation.parameters.iter()
						.find(|p| p.name == *expected_parameter_name && !p.required)
						.ok_or_else(|| format!( "operation {} is a {description} operation but doesn't have a {expected_parameter_name} parameter", operation.id))?;

					optional_definition
						.entry(crate::swagger20::PropertyName(expected_parameter_name.clone()))
						.or_insert_with(|| expected_parameter.schema.clone());
				}

				for parameter in &operation.parameters {
					if !parameter.required && !optional_parameters.contains(&*parameter.name) {
						return Err(format!("operation {} contains unexpected optional parameter {}", operation.id, parameter.name).into());
					}
				}
			}

			operation.parameters =
				operation.parameters.drain(..)
				.filter(|p| p.required)
				.chain(std::iter::once(optional_parameter.clone()))
				.collect();
		}

		if optional_definition.is_empty() {
			return Err(format!("never found any {description} operations").into());
		}

		// Not useful for programmatic clients.
		let _ = optional_definition.remove("pretty");

		spec.definitions.insert(crate::swagger20::DefinitionPath(type_name.to_string()), crate::swagger20::Schema {
			description: Some(format!("Common parameters for all {description} operations.")),
			kind: crate::swagger20::SchemaKind::Ty(ty(optional_definition)),
			kubernetes_group_kind_versions: vec![],
			list_kind: None,
			merge_type: crate::swagger20::MergeType::Default,
			impl_deep_merge: true,
		});
	}

	Ok(())
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

// The spec describes delete-collection operations as having query parameters that are a union of parameters of list and delete operations.
// In particular, they have watch-specific parameters that shouldn't be there, and get removed for regular list operations by
// the `separate_watch_from_list_operations` fixup below.
//
// So replace these path=query parameters with `ListOptional` and `DeleteOptions` parameters.
pub(crate) fn remove_delete_collection_operations_query_parameters(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::DeleteCollection) {
			operation.parameters = operation.parameters.drain(..).filter(|p| p.location == crate::swagger20::ParameterLocation::Path).collect();
			operation.parameters.push(std::sync::Arc::new(crate::swagger20::Parameter {
				location: crate::swagger20::ParameterLocation::Body,
				name: "deleteOptional".to_owned(),
				required: true,
				schema: crate::swagger20::Schema {
					description: Some("Delete options. Use `Default::default()` to not pass any.".to_owned()),
					kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
						path: "io.k8s.DeleteOptional".to_owned(),
						can_be_default: None,
					}),
					kubernetes_group_kind_versions: vec![],
					list_kind: None,
					merge_type: crate::swagger20::MergeType::Default,
					impl_deep_merge: true,
				},
			}));
			operation.parameters.push(std::sync::Arc::new(crate::swagger20::Parameter {
				location: crate::swagger20::ParameterLocation::Query,
				name: "listOptional".to_owned(),
				required: true,
				schema: crate::swagger20::Schema {
					description: Some("List options. Use `Default::default()` to not pass any.".to_owned()),
					kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
						path: "io.k8s.ListOptional".to_owned(),
						can_be_default: None,
					}),
					kubernetes_group_kind_versions: vec![],
					list_kind: None,
					merge_type: crate::swagger20::MergeType::Default,
					impl_deep_merge: true,
				},
			}));

			found = true;
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied remove-delete-collection-operations-query-parameters fixup".into())
	}
}

// Delete operations duplicate some of the properties of their path=body `DeleteOptions` parameter with path=query parameters.
//
// Remove the path=query parameters and replace the path=body parameter with an `io.k8s.DeleteOptional` parameter.
pub(crate) fn remove_delete_operations_query_parameters(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Delete) {
			if let Some(body_parameter) = operation.parameters.iter().find(|p| p.location == crate::swagger20::ParameterLocation::Body) {
				if let crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }) = &body_parameter.schema.kind {
					if path == "io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions" {
						operation.parameters = operation.parameters.drain(..).filter(|p| p.location == crate::swagger20::ParameterLocation::Path).collect();
						operation.parameters.push(std::sync::Arc::new(crate::swagger20::Parameter {
							location: crate::swagger20::ParameterLocation::Body,
							name: "optional".to_owned(),
							required: true,
							schema: crate::swagger20::Schema {
								description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
								kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
									path: "io.k8s.DeleteOptional".to_owned(),
									can_be_default: None,
								}),
								kubernetes_group_kind_versions: vec![],
								list_kind: None,
								merge_type: crate::swagger20::MergeType::Default,
								impl_deep_merge: true,
							},
						}));
						found = true;
						continue;
					}
				}

				return Err(format!("DELETE operation {} does not have a DeleteOptions body parameter", operation.id).into());
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied remove-delete-operations-query-parameters fixup".into())
	}
}

// Read operations have `exact` and `export` parameters used for server-side export, and a `pretty` parameter to pretty-print the output.
// The first two were removed in 1.21, were deprecated and known to be broken before that, and would've caused the response to not be able to be parsed
// with the corresponding response type anyway. And the third is not useful for programmatic clients. So this fixup removes these parameters.
//
// Since almost all read operations have only these parameters, such operations have their optional parameters type eliminated entirely as a result.
// To be precise, the only read operation that still has optional parameters is `readCoreV1NamespacedPodLog`.
pub(crate) fn remove_read_operations_query_parameters(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Get) {
			operation.parameters.retain(|p| match &*p.name {
				"exact" | "export" | "pretty" => { found = true; false }
				_ => true,
			});
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied remove-pretty-parameter fixup".into())
	}
}

// Some watch and watchlist operations (eg `watchCoreV1NamespacedPod` and `watchCoreV1NamespacedPodList`) are deprecated in favor of the corresponding list operation
// (eg `listCoreV1NamespacedPod`). The watch operation is equivalent to using the list operation with `watch=true` and `field_selector=...`, and the watchlist operation
// to using the list operation with just `watch=true`.
//
// This fixup removes such watch and watchlist operations from the parsed spec entirely. It then synthesizes two functions - a list operation and a watch operation.
// Neither function has a `watch` parameter, but the `watch` operation sets `watch=true` in its URL's query string implicitly. It uses the list operation's URI and
// parameters as a base.
//
// This also helps solve the problem that the default list operation's response type is a list type, which would be incorrect if the user called the function
// with the `watch` parameter set. Thus it's applied even to those list operations which don't have corresponding deprecated watch or watchlist operations.
//
// This fixup also synthesizes mod-root-level `ListOptional` and `WatchOptional` types which have the common parameters of all list and watch operations respectively.
pub(crate) fn separate_watch_from_list_operations(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let mut list_optional_parameters: Option<std::collections::HashSet<String>> = None;
	let mut list_optional_definition: std::collections::BTreeMap<crate::swagger20::PropertyName, crate::swagger20::Schema> = Default::default();
	let mut watch_optional_definition: std::collections::BTreeMap<crate::swagger20::PropertyName, crate::swagger20::Schema> = Default::default();
	let mut list_operations = vec![];

	for operation in &mut spec.operations {
		if operation.kubernetes_action != Some(crate::swagger20::KubernetesAction::List) {
			continue;
		}

		if !operation.id.starts_with("list") {
			return Err(format!(r#"operation {} is a list operation but doesn't start with "list""#, operation.id).into());
		}

		let list_optional_parameters =
			&*list_optional_parameters.get_or_insert_with(|| operation.parameters.iter().map(|p| p.name.clone()).collect());

		for expected_parameter_name in list_optional_parameters {
			let expected_parameter =
				operation.parameters.iter()
				.find(|p| p.name == *expected_parameter_name && !p.required)
				.ok_or_else(|| format!("operation {} is a list operation but doesn't have a {expected_parameter_name} parameter", operation.id))?;

			if expected_parameter_name != "allowWatchBookmarks" && expected_parameter_name != "watch" {
				list_optional_definition
					.entry(crate::swagger20::PropertyName(expected_parameter_name.clone()))
					.or_insert_with(|| expected_parameter.schema.clone());
			}

			if
				expected_parameter_name != "continue" &&
				expected_parameter_name != "limit" &&
				expected_parameter_name != "resourceVersionMatch" &&
				expected_parameter_name != "watch"
			{
				watch_optional_definition
					.entry(crate::swagger20::PropertyName(expected_parameter_name.clone()))
					.or_insert_with(|| expected_parameter.schema.clone());
			}
		}

		for parameter in &operation.parameters {
			if !parameter.required && !list_optional_parameters.contains(&*parameter.name) {
				return Err(format!("operation {} contains unexpected optional parameter {}", operation.id, parameter.name).into());
			}
		}

		list_operations.push(operation.id.clone());
	}

	if list_operations.is_empty() {
		return Err("never found any list-watch operations".into());
	}

	// Not useful for programmatic clients.
	let _ = list_optional_definition.remove("pretty");
	let _ = watch_optional_definition.remove("pretty");

	spec.definitions.insert(crate::swagger20::DefinitionPath("io.k8s.ListOptional".to_string()), crate::swagger20::Schema {
		description: Some("Common parameters for all list operations.".to_string()),
		kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListOptional(list_optional_definition)),
		kubernetes_group_kind_versions: vec![],
		list_kind: None,
		merge_type: crate::swagger20::MergeType::Default,
		impl_deep_merge: true,
	});

	spec.definitions.insert(crate::swagger20::DefinitionPath("io.k8s.WatchOptional".to_string()), crate::swagger20::Schema {
		description: Some("Common parameters for all watch operations.".to_string()),
		kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::WatchOptional(watch_optional_definition)),
		kubernetes_group_kind_versions: vec![],
		list_kind: None,
		merge_type: crate::swagger20::MergeType::Default,
		impl_deep_merge: true,
	});

	let list_optional_parameter = std::sync::Arc::new(crate::swagger20::Parameter {
		location: crate::swagger20::ParameterLocation::Query,
		name: "optional".to_owned(),
		required: true,
		schema: crate::swagger20::Schema {
			description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.ListOptional".to_owned(),
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: vec![],
			list_kind: None,
			merge_type: crate::swagger20::MergeType::Default,
			impl_deep_merge: true,
		},
	});

	let watch_optional_parameter = std::sync::Arc::new(crate::swagger20::Parameter {
		location: crate::swagger20::ParameterLocation::Query,
		name: "optional".to_owned(),
		required: true,
		schema: crate::swagger20::Schema {
			description: Some("Optional parameters. Use `Default::default()` to not pass any.".to_owned()),
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.WatchOptional".to_owned(),
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: vec![],
			list_kind: None,
			merge_type: crate::swagger20::MergeType::Default,
			impl_deep_merge: true,
		},
	});

	let mut converted_watch_operations: std::collections::HashSet<_> = Default::default();

	for list_operation_id in list_operations {
		let watch_operation_id = list_operation_id.replacen("list", "watch", 1);
		let watch_list_operation_id =
			if watch_operation_id.ends_with("ForAllNamespaces") {
				watch_operation_id[..(watch_operation_id.len() - "ForAllNamespaces".len())].to_owned() + "ListForAllNamespaces"
			}
			else {
				watch_operation_id.clone() + "List"
			};

		if let Some(watch_operation_index) = spec.operations.iter().position(|o| o.id == watch_operation_id) {
			spec.operations.swap_remove(watch_operation_index);
		}
		if let Some(watch_list_operation_index) = spec.operations.iter().position(|o| o.id == watch_list_operation_id) {
			spec.operations.swap_remove(watch_list_operation_index);
		}

		let (original_list_operation_index, original_list_operation) = spec.operations.iter().enumerate().find(|(_, o)| o.id == list_operation_id).unwrap();

		let mut base_description = original_list_operation.description.as_ref().map_or("", std::ops::Deref::deref).to_owned();
		if !base_description.is_empty() {
			writeln!(base_description)?;
			writeln!(base_description)?;
		}

		let mut list_operation = crate::swagger20::Operation {
			description: Some({
				let mut description = base_description.clone();
				writeln!(description, "This operation only supports listing all items of this type.")?;
				description
			}),
			..original_list_operation.clone()
		};
		list_operation.parameters =
			list_operation.parameters.into_iter()
			.filter(|parameter| parameter.required)
			.chain(std::iter::once(list_optional_parameter.clone()))
			.collect();

		let mut watch_operation = crate::swagger20::Operation {
			description: Some({
				let mut description = base_description.clone();
				writeln!(description, "This operation only supports watching one item, or a list of items, of this type for changes.")?;
				description
			}),
			id: watch_operation_id.clone(),
			kubernetes_action: Some(crate::swagger20::KubernetesAction::Watch),
			..original_list_operation.clone()
		};
		watch_operation.parameters =
			watch_operation.parameters.into_iter()
			.filter(|parameter| parameter.required)
			.chain(std::iter::once(watch_optional_parameter.clone()))
			.collect();
		let crate::swagger20::OperationResponses::Map(responses) = &mut watch_operation.responses else { unreachable!(); };
		responses.insert(http::StatusCode::OK, crate::swagger20::Schema {
			description: Some("OK".to_owned()),
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent".to_owned(),
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: vec![],
			list_kind: None,
			merge_type: crate::swagger20::MergeType::Default,
			impl_deep_merge: true,
		});

		spec.operations[original_list_operation_index] = list_operation;
		spec.operations.push(watch_operation);
		converted_watch_operations.insert(watch_operation_id);
	}

	for operation in &spec.operations {
		if let Some(crate::swagger20::KubernetesAction::Watch | crate::swagger20::KubernetesAction::WatchList) = operation.kubernetes_action {
			if !converted_watch_operations.contains(&operation.id) {
				return Err(format!("found a watch operation that wasn't synthesized from a list operation: {operation:?}").into());
			}
		}
	}

	Ok(())
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

		for operation in &mut spec.operations {
			let crate::swagger20::OperationResponses::Map(responses) = &mut operation.responses else { unreachable!(); };
			for response in responses.values_mut() {
				let response_schema_kind = &mut response.kind;
				if let crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }) = response_schema_kind {
					if path == &definition_path.0 {
						*response_schema_kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListRef {
							items: Box::new(crate::swagger20::SchemaKind::Ref(item_ref_path.clone())),
						});
					}
				}
			}
		}
	}

	Ok(())
}

// Define the common types for API responses as `swagger20::Type::<>Def`, and replace all references to the original types with `swagger20::Type::<>Ref` for special codegen.
pub(crate) fn response_types(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	#[allow(clippy::type_complexity)]
	const TYPES: &[(&str, fn(&mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error>)] = &[
		("io.k8s.CreateResponse", create_response),
		("io.k8s.DeleteResponse", delete_and_delete_collection_response),
		("io.k8s.ListResponse", list_response),
		("io.k8s.PatchResponse", patch_response),
		("io.k8s.ReplaceResponse", replace_response),
		("io.k8s.WatchResponse", watch_response),
	];

	fn create_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found = false;

		for operation in &mut spec.operations {
			if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Post) {
				let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

				let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
				response_status_codes.sort();
				if response_status_codes != [http::StatusCode::OK, http::StatusCode::CREATED, http::StatusCode::ACCEPTED] {
					return Err(format!("operation {} does not have the expected response status codes of a create operation: {response_status_codes:?}",
						operation.id).into());
				}

				let group_version_kind =
					operation.kubernetes_group_kind_version.as_ref()
					.ok_or_else(|| format!("operation {} looks like a create but doesn't have a group-version-kind", operation.id))?;
				let expected_ref_path_suffix = format!(".{}", group_version_kind.kind);

				let should_use_common_response_type =
					responses.values()
					.all(|crate::swagger20::Schema { kind, .. }|
						if let crate::swagger20::SchemaKind::Ref(ref_path) = kind {
							ref_path.path.ends_with(&expected_ref_path_suffix)
						}
						else {
							false
						});
				if !should_use_common_response_type {
					continue;
				}

				operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::CreateResponse);

				found = true;
			}
		}

		if !found {
			return Err("did not find any create operations".into());
		}

		Ok((
			"The common response type for all create API operations.",
			crate::swagger20::Type::CreateResponse,
		))
	}

	fn delete_and_delete_collection_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found_delete = false;
		let mut found_delete_collection = false;

		for operation in &mut spec.operations {
			match operation.kubernetes_action {
				Some(crate::swagger20::KubernetesAction::Delete) => {
					let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

					let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
					response_status_codes.sort();
					if response_status_codes != [http::StatusCode::OK, http::StatusCode::ACCEPTED] {
						return Err(format!("operation {} does not have the expected response status codes of a delete operation: {response_status_codes:?}",
							operation.id).into());
					}

					// Prevent the Iterator::all closure below capturing all of `self` and complain about conflicting borrows
					let definitions = &spec.definitions;

					let should_use_common_response_type =
						responses.values()
						.all(|crate::swagger20::Schema { kind, .. }| {
							let crate::swagger20::SchemaKind::Ref(ref_path) = kind else { return false; };

							if ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.Status" {
								return true;
							}

							if let Some(kubernetes_group_kind_version) = &operation.kubernetes_group_kind_version {
								let response_schema =
									definitions.get(&*ref_path.path)
									.unwrap_or_else(|| panic!("operation {} returns undefined type {kubernetes_group_kind_version:?}", operation.id));
								if response_schema.kubernetes_group_kind_versions.contains(kubernetes_group_kind_version) {
									return true;
								}
							}

							false
						});
					if !should_use_common_response_type {
						continue;
					}

					operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse);

					found_delete = true;
				},

				Some(crate::swagger20::KubernetesAction::DeleteCollection) => {
					let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

					let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
					response_status_codes.sort();
					if response_status_codes != [http::StatusCode::OK] // delete-collection does not have 202, but we'll synthesize it anyway
					{
						return Err(format!("operation {} does not have the expected response status codes of a delete-collection operation: {response_status_codes:?}",
							operation.id).into());
					}

					let should_use_common_response_type =
						responses.values()
						.all(|crate::swagger20::Schema { kind, .. }|
							if let crate::swagger20::SchemaKind::Ref(ref_path) = kind {
								ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.Status"
							}
							else {
								false
							});
					if !should_use_common_response_type {
						continue;
					}

					operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::DeleteResponse);

					found_delete_collection = true;
				},

				_ => (),
			}
		}

		if !found_delete {
			return Err("did not find any delete operations".into());
		}

		if !found_delete_collection {
			return Err("did not find any delete-collection operations".into());
		}

		Ok((
			"The common response type for all delete API operations and delete-collection API operations.",
			crate::swagger20::Type::DeleteResponse,
		))
	}

	fn list_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found = false;

		for operation in &mut spec.operations {
			if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::List) {
				let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

				let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
				response_status_codes.sort();
				if response_status_codes != [http::StatusCode::OK] {
					return Err(format!("operation {} does not have the expected response status codes of a list operation: {response_status_codes:?}",
						operation.id).into());
				}

				let group_version_kind =
					operation.kubernetes_group_kind_version.as_ref()
					.ok_or_else(|| format!("operation {} looks like a list but doesn't have a group-version-kind", operation.id))?;
				let expected_ref_path_suffix = format!(".{}", group_version_kind.kind);

				let should_use_common_response_type =
					responses.values()
					.all(|crate::swagger20::Schema { kind, .. }|
						if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListRef { items }) = kind {
							if let crate::swagger20::SchemaKind::Ref(ref_path) = &**items {
								ref_path.path.ends_with(&expected_ref_path_suffix)
							}
							else {
								false
							}
						}
						else {
							false
						});
				if !should_use_common_response_type {
					continue;
				}

				operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ListResponse);

				found = true;
			}
		}

		if !found {
			return Err("did not find any list operations".into());
		}

		Ok((
			"The common response type for all list API operations.",
			crate::swagger20::Type::ListResponse,
		))
	}

	fn patch_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found = false;

		for operation in &mut spec.operations {
			if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Patch) {
				let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

				let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
				response_status_codes.sort();
				if
					response_status_codes != [http::StatusCode::OK, http::StatusCode::CREATED] &&
					response_status_codes != [http::StatusCode::OK] // 1.21 and earlier did not have 201
				{
					return Err(format!("operation {} does not have the expected response status codes of a patch operation: {response_status_codes:?}",
						operation.id).into());
				}

				let group_version_kind =
					operation.kubernetes_group_kind_version.as_ref()
					.ok_or_else(|| format!("operation {} looks like a patch but doesn't have a group-version-kind", operation.id))?;
				let expected_ref_path_suffix = format!(".{}", group_version_kind.kind);

				let should_use_common_response_type =
					responses.values()
					.all(|crate::swagger20::Schema { kind, .. }|
						if let crate::swagger20::SchemaKind::Ref(ref_path) = kind {
							ref_path.path.ends_with(&expected_ref_path_suffix)
						}
						else {
							false
						});
				if !should_use_common_response_type {
					continue;
				}

				operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::PatchResponse);

				found = true;
			}
		}

		if !found {
			return Err("did not find any patch operations".into());
		}

		Ok((
			"The common response type for all patch API operations.",
			crate::swagger20::Type::PatchResponse,
		))
	}

	fn replace_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found = false;

		for operation in &mut spec.operations {
			if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Put) {
				let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

				let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
				response_status_codes.sort();
				if response_status_codes != [http::StatusCode::OK, http::StatusCode::CREATED] {
					return Err(format!("operation {} does not have the expected response status codes of a replace operation: {response_status_codes:?}",
						operation.id).into());
				}

				let group_version_kind =
					operation.kubernetes_group_kind_version.as_ref()
					.ok_or_else(|| format!("operation {} looks like a replace but doesn't have a group-version-kind", operation.id))?;
				let expected_ref_path_suffix = format!(".{}", group_version_kind.kind);

				let should_use_common_response_type =
					responses.values()
					.all(|crate::swagger20::Schema { kind, .. }|
						if let crate::swagger20::SchemaKind::Ref(ref_path) = kind {
							ref_path.path.ends_with(&expected_ref_path_suffix)
						}
						else {
							false
						});
				if !should_use_common_response_type {
					continue;
				}

				operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::ReplaceResponse);

				found = true;
			}
		}

		if !found {
			return Err("did not find any replace operations".into());
		}

		Ok((
			"The common response type for all replace API operations.",
			crate::swagger20::Type::ReplaceResponse,
		))
	}

	fn watch_response(spec: &mut crate::swagger20::Spec) -> Result<(&'static str, crate::swagger20::Type), crate::Error> {
		let mut found = false;

		for operation in &mut spec.operations {
			if operation.kubernetes_action == Some(crate::swagger20::KubernetesAction::Watch) {
				let crate::swagger20::OperationResponses::Map(responses) = &operation.responses else { unreachable!(); };

				let mut response_status_codes: Vec<_> = responses.keys().copied().collect();
				response_status_codes.sort();
				if response_status_codes != [http::StatusCode::OK] {
					return Err(format!("operation {} does not have the expected response status codes of a watch operation: {response_status_codes:?}",
						operation.id).into());
				}

				let should_use_common_response_type =
					responses.values()
					.all(|crate::swagger20::Schema { kind, .. }|
						if let crate::swagger20::SchemaKind::Ref(ref_path) = kind {
							ref_path.path == "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent"
						}
						else {
							false
						});
				if !should_use_common_response_type {
					continue;
				}

				operation.responses = crate::swagger20::OperationResponses::Common(crate::swagger20::Type::WatchResponse);

				found = true;
			}
		}

		if !found {
			return Err("did not find any watch operations".into());
		}

		Ok((
			"The common response type for all watch API operations.",
			crate::swagger20::Type::WatchResponse,
		))
	}

	for &(definition_path, run) in TYPES {
		let (description, ty) = run(spec)?;

		spec.definitions.insert(
			crate::swagger20::DefinitionPath(definition_path.to_owned()),
			crate::swagger20::Schema {
				description: Some(description.to_owned()),
				kind: crate::swagger20::SchemaKind::Ty(ty),
				kubernetes_group_kind_versions: vec![],
				list_kind: None,
				merge_type: crate::swagger20::MergeType::Default,
				impl_deep_merge: true,
			});
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
