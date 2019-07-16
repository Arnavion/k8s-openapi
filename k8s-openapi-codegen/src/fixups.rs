#![deny(unused)]

// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/66807
#[allow(clippy::if_same_then_else)]
pub(crate) fn connect_options_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if let Some(kubernetes_group_kind_version) = &mut operation.kubernetes_group_kind_version {
			if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "NodeProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Node".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodAttachOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodExecOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodPortForwardOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "ServiceProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Service".to_string();
				found = true;
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

// The spec says that `createAppsV1beta1NamespacedDeploymentRollback` returns `DeploymentRollback`, but it returns `Status`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/63837
pub(crate) fn deployment_rollback_create_response_type(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	if let Some(operation) = spec.operations.iter_mut().find(|o| o.id == "createAppsV1beta1NamespacedDeploymentRollback") {
		for response in operation.responses.values_mut() {
			if let crate::swagger20::Schema { kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }), .. } = response {
				if path == "io.k8s.api.apps.v1beta1.DeploymentRollback" {
					std::mem::replace(path, "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned());
					found = true;
				}
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied createAppsV1beta1NamespacedDeploymentRollback response type override".into())
	}
}

// Type is not annotated with "x-kubernetes-group-version-kind", which would make its associated functions end up in the mod root.
//
// Ref: https://github.com/kubernetes/kubernetes/issues/49465
// Ref: https://github.com/kubernetes/kubernetes/pull/64174
pub(crate) mod gvk {
	pub(crate) fn api_service_list_v1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIServiceList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIServiceList".to_string(),
					version: "v1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIServiceList v1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_list_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIServiceList".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIServiceList v1beta1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_v1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIService".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIService".to_string(),
					version: "v1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIService v1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIService".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIService v1beta1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn crd_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiextensions.k8s.io".to_string(),
					kind: "CustomResourceDefinition".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied CustomResourceDefinition kubernetes_group_kind_version override".into())
	}

	pub(crate) fn crd_list_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiextensions.k8s.io".to_string(),
					kind: "CustomResourceDefinitionList".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied CustomResourceDefinitionList v1beta1 kubernetes_group_kind_version override".into())
	}
}

// Fixes to the JSON types used in CRD validation
//
// Ref: https://github.com/kubernetes/kubernetes/pull/65256
pub(crate) mod json_ty {
	// The spec says that `JSON` is an object with a property `Raw` that's a byte-formatted string.
	// While the golang type is indeed a struct with a `Raw []byte` field, the type is serialized by just emitting the value of that field.
	// The value of that field is itself a JSON-serialized value.
	//
	// Thus `JSON` is really an arbitrary JSON value, and should be represented by `serde_json::Value`
	pub(crate) fn json(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSON".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any) = definition.kind {
			}
			else {
				definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any);
				return Ok(());
			}
		}

		Err("never applied JSON override".into())
	}

	// The spec says that `JSONSchemaPropsOrArray` is an object with properties `JSONSchemas` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or an array of `JSONSchemaProps`.
	pub(crate) fn json_schema_props_or_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrArray) = definition.kind {
			}
			else {
				definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrArray);
				return Ok(());
			}
		}

		Err("never applied JSONSchemaPropsOrArray override".into())
	}

	// The spec says that `JSONSchemaPropsOrBool` is an object with properties `Allows` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or a `bool`.
	pub(crate) fn json_schema_props_or_bool(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrBool) = definition.kind {
			}
			else {
				definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrBool);
				return Ok(());
			}
		}

		Err("never applied JSONSchemaPropsOrBool override".into())
	}

	// The spec says that `JSONSchemaPropsOrStringArray` is an object with properties `Property` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or a `Vec<String>`.
	pub(crate) fn json_schema_props_or_string_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrStringArray".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrStringArray) = definition.kind {
			}
			else {
				definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrStringArray);
				return Ok(());
			}
		}

		Err("never applied JSONSchemaPropsOrStringArray override".into())
	}
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
	// `APIGroup::serverAddressByClientCIDRs`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/61963
	pub(crate) fn apigroup(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("serverAddressByClientCIDRs".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied APIGroups optional properties override".into())
	}

	// `CustomResourceDefinitionStatus::conditions`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/64996
	pub(crate) fn crdstatus(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionStatus".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("conditions".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied CustomResourceDefinitionStatus optional properties override".into())
	}

	// `PodDisruptionBudgetStatus::disruptedPods`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/65041
	pub(crate) fn poddisruptionbudgetstatus(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.policy.v1beta1.PodDisruptionBudgetStatus".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("disruptedPods".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied PodDisruptionBudgetStatus optional properties override".into())
	}
}

// The spec says that `RawExtension` is an object with a property `raw` that's a byte-formatted string.
// While the golang type is indeed a struct with a `Raw []byte` field, the type is serialized by just emitting the value of that field.
// The value of that field is itself a JSON-serialized value. For example, a `WatchEvent` of `Pod`s has the `Pod` object serialized as
// the value of the `WatchEvent::object` property.
//
// Thus `RawExtension` is really an arbitrary JSON value, and should be represented by `serde_json::Value`
//
// Ref: https://github.com/kubernetes/kubernetes/issues/55890
//
// https://github.com/kubernetes/kubernetes/pull/56434 will remove RawExtension and replace it with `{ type: "object" }`,
// which would've already been mapped to `Ty(Any)` by `Ty::parse`, so just replicate that for `RawExtension` here.
pub(crate) fn raw_extension_ty(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.runtime.RawExtension".to_owned());
	if let Some(definition) = spec.definitions.get_mut(&definition_path) {
		if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any) = definition.kind {
		}
		else {
			definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any);
			return Ok(());
		}
	}

	Err("never applied RawExtension override".into())
}

// Remove `$ref`s under `io.k8s.kubernetes.pkg` since these are marked deprecated and point to corresponding definitions under `io.k8s.api`.
// They only exist for backward-compatibility with 1.7's spec.
pub(crate) fn remove_compat_refs(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	const COMPAT_NAMESPACE: &[&str] = &["io", "k8s", "kubernetes", "pkg"];

	let mut to_remove = vec![];

	for (definition_path, definition) in &spec.definitions {
		if let crate::swagger20::SchemaKind::Ref(_) = definition.kind {
			let parts: Vec<_> = definition_path.split('.').collect();
			if parts.starts_with(COMPAT_NAMESPACE) {
				to_remove.push(definition_path.clone());
			}
		}
	}

	if to_remove.is_empty() {
		return Err("never removed compat refs".into());
	}

	for to_remove in to_remove {
		spec.definitions.remove(&to_remove);
	}

	Ok(())
}

// DELETE operations duplicate all their path=query parameters with a path=body DeleteOptions parameter.
// Remove the path=body parameter.
pub(crate) fn remove_delete_options_body_parameter(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if operation.method == crate::swagger20::Method::Delete {
			if let Some((index, body_parameter)) = operation.parameters.iter().enumerate().find(|(_, p)| p.location == crate::swagger20::ParameterLocation::Body) {
				if let crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }) = &body_parameter.schema.kind {
					if path == "io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions" {
						operation.parameters.remove(index);
						found = true;
					}
				}
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied remove-body-parameters-for-get-and-delete fixup".into())
	}
}

/// Some watch and watchlist operations (eg `watchCoreV1NamespacedPod` and `watchCoreV1NamespacedPodList`) are deprecated in favor of the corresponding list operation
/// (eg `listCoreV1NamespacedPod`). The watch operation is equivalent to using the list operation with `watch=true` and `field_selector=...`, and the watchlist operation
/// to using the list operation with just `watch=true`.
///
/// This fixup removes such watch and watchlist operations from the parsed spec entirely. It then synthesizes two functions - a list operation and a watch operation.
/// Neither function has a `watch` parameter, but the `watch` operation sets `watch=true` in its URL's query string implicitly. It uses the list operation's URI and
/// parameters as a base.
///
/// This also helps solve the problem that the default list operation's response type is a list type, which would be incorrect if the user called the function
/// with the `watch` parameter set. Thus it's applied even to those list operations which don't have corresponding deprecated watch or watchlist operations.
///
/// This fixup also synthesizes mod-root-level `ListOptional` and `WatchOptional` types which have the common parameters of all list and watch operations respectively.
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
				if let Some(expected_parameter) = operation.parameters.iter().find(|p| p.name == *expected_parameter_name && !p.required) {
					&**expected_parameter
				}
				else {
					return Err(format!("operation {} is a list operation but doesn't have a {} parameter", operation.id, expected_parameter_name).into());
				};

			if expected_parameter_name != "watch" {
				list_optional_definition
					.entry(crate::swagger20::PropertyName(expected_parameter_name.to_owned()))
					.or_insert_with(|| expected_parameter.clone().schema);
			}

			if expected_parameter_name != "continue" && expected_parameter_name != "limit" && expected_parameter_name != "watch" {
				watch_optional_definition
					.entry(crate::swagger20::PropertyName(expected_parameter_name.to_owned()))
					.or_insert_with(|| expected_parameter.clone().schema);
			}
		}

		for parameter in &operation.parameters {
			if !parameter.required && !list_optional_parameters.contains(&*parameter.name) {
				return Err(format!("operation {} contains unexpected optional parameter {}", operation.id, parameter.name).into());
			}
		}

		list_operations.push(operation.id.to_owned());
	}

	if list_operations.is_empty() {
		return Err("never found any list-watch operations".into());
	}

	spec.definitions.insert(crate::swagger20::DefinitionPath("io.k8s.ListOptional".to_string()), crate::swagger20::Schema {
		description: Some("Common parameters for all list operations.".to_string()),
		kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::ListOptional(list_optional_definition)),
		kubernetes_group_kind_versions: None,
	});

	spec.definitions.insert(crate::swagger20::DefinitionPath("io.k8s.WatchOptional".to_string()), crate::swagger20::Schema {
		description: Some("Common parameters for all watch operations.".to_string()),
		kind: crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::WatchOptional(watch_optional_definition)),
		kubernetes_group_kind_versions: None,
	});

	let list_optional_parameter = std::sync::Arc::new(crate::swagger20::Parameter {
		location: crate::swagger20::ParameterLocation::Query,
		name: "optional".to_owned(),
		required: true,
		schema: crate::swagger20::Schema {
			description: None,
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.ListOptional".to_owned(),
				relative_to: crate::swagger20::RefPathRelativeTo::Crate,
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: None,
		},
	});

	let watch_optional_parameter = std::sync::Arc::new(crate::swagger20::Parameter {
		location: crate::swagger20::ParameterLocation::Query,
		name: "optional".to_owned(),
		required: true,
		schema: crate::swagger20::Schema {
			description: None,
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.WatchOptional".to_owned(),
				relative_to: crate::swagger20::RefPathRelativeTo::Crate,
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: None,
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
		watch_operation.responses.insert(reqwest::StatusCode::OK, crate::swagger20::Schema {
			description: Some("OK".to_owned()),
			kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath {
				path: "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent".to_owned(),
				relative_to: crate::swagger20::RefPathRelativeTo::Crate,
				can_be_default: None,
			}),
			kubernetes_group_kind_versions: None,
		});

		spec.operations[original_list_operation_index] = list_operation;
		spec.operations.push(watch_operation);
		converted_watch_operations.insert(watch_operation_id);
	}

	for operation in &spec.operations {
		match operation.kubernetes_action {
			Some(crate::swagger20::KubernetesAction::Watch) |
			Some(crate::swagger20::KubernetesAction::WatchList) =>
				if !converted_watch_operations.contains(&operation.id) {
					return Err(format!("found a watch operation that wasn't synthesized from a list operation: {:?}", operation).into());
				},

			_ => (),
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
							writeln!(type_description, "{}", property_description)?;
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
