#![deny(unused)]

//! These fixups correspond to bugs in the upstream swagger spec.

use crate::fixups::Fixup;


// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/66807
pub(crate) struct ConnectOptionsGvk;

impl Fixup for ConnectOptionsGvk {
	#[allow(clippy::if_same_then_else)]
	fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
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

	fn requires_client_generation(&self) -> bool {
		true
	}
}

// The spec says that `createAppsV1beta1NamespacedDeploymentRollback` returns `DeploymentRollback`, but it returns `Status`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/63837
pub(crate) struct DeploymentRollbackCreateResponseType;

impl Fixup for DeploymentRollbackCreateResponseType {
	fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let mut found = false;

		if let Some(operation) = spec.operations.iter_mut().find(|o| o.id == "createAppsV1beta1NamespacedDeploymentRollback") {
			if let crate::swagger20::OperationResponses::Map(responses) = &mut operation.responses {
				for response in responses.values_mut() {
					if let crate::swagger20::Schema { kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }), .. } = response {
						if path == "io.k8s.api.apps.v1beta1.DeploymentRollback" {
							*path = "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned();
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
			Err("never applied createAppsV1beta1NamespacedDeploymentRollback response type override".into())
		}
	}

	fn requires_client_generation(&self) -> bool {
		true
	}
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
	use crate::fixups::Fixup;

	// `ContainerImage::names`
	//
	// Ref: https://github.com/kubernetes/kubernetes/issues/93606
	pub(crate) struct ContainerImage;

	impl Fixup for ContainerImage {
		fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
			let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.core.v1.ContainerImage".to_owned());
			if let Some(definition) = spec.definitions.get_mut(&definition_path) {
				if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
					if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("names".to_string())) {
						if property.1 {
							property.1 = false;
							return Ok(());
						}
					}
				}
			}
	
			Err("never applied ContainerImage optional properties override".into())
		}

		fn requires_client_generation(&self) -> bool {
			false
		}
	}

	// `CustomResourceDefinitionStatus::conditions`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/64996
	pub(crate) struct CrdStatus;

	impl Fixup for CrdStatus {
		fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
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

		fn requires_client_generation(&self) -> bool {
			false
		}
	}

	// `PodDisruptionBudgetStatus::disruptedPods`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/65041
	pub(crate) struct PdbStatus;

	impl Fixup for PdbStatus {
		fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
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

		fn requires_client_generation(&self) -> bool {
			false
		}
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
pub(crate) struct RawExtensionTy;

impl Fixup for RawExtensionTy {
	fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.runtime.RawExtension".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if !matches!(definition.kind, crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any)) {
				definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any);
				return Ok(());
			}
		}
	
		Err("never applied RawExtension override".into())
	}

	fn requires_client_generation(&self) -> bool {
		false
	}
}

// Remove `$ref`s under `io.k8s.kubernetes.pkg` since these are marked deprecated and point to corresponding definitions under `io.k8s.api`.
// They only exist for backward-compatibility with 1.7's spec.
pub(crate) struct RemoveCompatRefs;

impl Fixup for RemoveCompatRefs {
	fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
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

	fn requires_client_generation(&self) -> bool {
		false
	}
}