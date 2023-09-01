//! This module contains types related to the OpenAPI types used in the Kubernetes spec.

mod definitions;
pub use self::definitions::*;

mod info;
pub use self::info::*;

mod paths;
pub use self::paths::*;

/// The value of an `x-kubernetes-group-version-kind` annotation on a type or an operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct KubernetesGroupKindVersion {
    pub group: String,
    pub kind: String,
    pub version: String,
}

impl std::cmp::Ord for KubernetesGroupKindVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.group.cmp(&other.group)
        .then_with(|| self.version.cmp(&other.version))
        .then_with(|| self.kind.cmp(&other.kind))
    }
}

impl std::cmp::PartialOrd for KubernetesGroupKindVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The whole `Spec` object. An OpenAPI spec JSON file can be deserialized into this type.
#[derive(Debug)]
pub struct Spec {
    pub info: Info,
    pub definitions: std::collections::BTreeMap<DefinitionPath, Schema>,
    pub operations: Vec<Operation>,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for Spec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[derive(Debug, serde::Deserialize)]
        pub struct InnerSpec {
            info: Info,
            definitions: std::collections::BTreeMap<DefinitionPath, Schema>,
            paths: std::collections::BTreeMap<Path, InnerPathItem>,
            swagger: String,
        }

        #[derive(Debug, serde::Deserialize)]
        struct InnerPathItem {
            delete: Option<InnerOperation>,
            get: Option<InnerOperation>,
            patch: Option<InnerOperation>,
            post: Option<InnerOperation>,
            put: Option<InnerOperation>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct InnerOperation {
            #[serde(rename = "operationId")]
            id: String,
            #[serde(rename = "x-kubernetes-action")]
            kubernetes_action: Option<KubernetesAction>,
            #[serde(rename = "x-kubernetes-group-version-kind")]
            kubernetes_group_kind_version: Option<KubernetesGroupKindVersion>,
        }

        let result: InnerSpec = serde::Deserialize::deserialize(deserializer)?;

        if result.swagger != "2.0" {
            return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&result.swagger), &"2.0"));
        }

        let mut operations = vec![];

        for (path, path_item) in result.paths {
            if let Some(delete) = path_item.delete {
                operations.push(Operation {
                    id: delete.id,
                    kubernetes_action: delete.kubernetes_action,
                    kubernetes_group_kind_version: delete.kubernetes_group_kind_version,
                    path: path.clone(),
                });
            }

            if let Some(get) = path_item.get {
                operations.push(Operation {
                    id: get.id,
                    kubernetes_action: get.kubernetes_action,
                    kubernetes_group_kind_version: get.kubernetes_group_kind_version,
                    path: path.clone(),
                });
            }

            if let Some(patch) = path_item.patch {
                operations.push(Operation {
                    id: patch.id,
                    kubernetes_action: patch.kubernetes_action,
                    kubernetes_group_kind_version: patch.kubernetes_group_kind_version,
                    path: path.clone(),
                });
            }

            if let Some(post) = path_item.post {
                operations.push(Operation {
                    id: post.id,
                    kubernetes_action: post.kubernetes_action,
                    kubernetes_group_kind_version: post.kubernetes_group_kind_version,
                    path: path.clone(),
                });
            }

            if let Some(put) = path_item.put {
                operations.push(Operation {
                    id: put.id,
                    kubernetes_action: put.kubernetes_action,
                    kubernetes_group_kind_version: put.kubernetes_group_kind_version,
                    path,
                });
            }
        }

        let mut operation_ids: std::collections::BTreeSet<_> = Default::default();
        for operation in &operations {
            assert!(operation_ids.insert(&operation.id));
        }

        Ok(Spec {
            info: result.info,
            definitions: result.definitions,
            operations,
        })
    }
}
