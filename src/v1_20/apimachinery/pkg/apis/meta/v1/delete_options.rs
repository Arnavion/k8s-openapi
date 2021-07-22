// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions

/// DeleteOptions may be provided when deleting an API object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteOptions {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    pub api_version: Option<String>,

    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Vec<String>,

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    pub preconditions: Option<crate::apimachinery::pkg::apis::meta::v1::Preconditions>,

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for DeleteOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_dry_run,
            Key_grace_period_seconds,
            Key_kind,
            Key_orphan_dependents,
            Key_preconditions,
            Key_propagation_policy,
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
                            "apiVersion" => Field::Key_api_version,
                            "dryRun" => Field::Key_dry_run,
                            "gracePeriodSeconds" => Field::Key_grace_period_seconds,
                            "kind" => Field::Key_kind,
                            "orphanDependents" => Field::Key_orphan_dependents,
                            "preconditions" => Field::Key_preconditions,
                            "propagationPolicy" => Field::Key_propagation_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeleteOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeleteOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_dry_run: Option<Vec<String>> = None;
                let mut value_grace_period_seconds: Option<i64> = None;
                let mut value_kind: Option<String> = None;
                let mut value_orphan_dependents: Option<bool> = None;
                let mut value_preconditions: Option<crate::apimachinery::pkg::apis::meta::v1::Preconditions> = None;
                let mut value_propagation_policy: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dry_run => value_dry_run = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_grace_period_seconds => value_grace_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_orphan_dependents => value_orphan_dependents = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preconditions => value_preconditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_propagation_policy => value_propagation_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeleteOptions {
                    api_version: value_api_version,
                    dry_run: value_dry_run.unwrap_or_default(),
                    grace_period_seconds: value_grace_period_seconds,
                    kind: value_kind,
                    orphan_dependents: value_orphan_dependents,
                    preconditions: value_preconditions,
                    propagation_policy: value_propagation_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeleteOptions",
            &[
                "apiVersion",
                "dryRun",
                "gracePeriodSeconds",
                "kind",
                "orphanDependents",
                "preconditions",
                "propagationPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeleteOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeleteOptions",
            self.api_version.as_ref().map_or(0, |_| 1) +
            usize::from(!self.dry_run.is_empty()) +
            self.grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.orphan_dependents.as_ref().map_or(0, |_| 1) +
            self.preconditions.as_ref().map_or(0, |_| 1) +
            self.propagation_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if !self.dry_run.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dryRun", &self.dry_run)?;
        }
        if let Some(value) = &self.grace_period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.orphan_dependents {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "orphanDependents", value)?;
        }
        if let Some(value) = &self.preconditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preconditions", value)?;
        }
        if let Some(value) = &self.propagation_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "propagationPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for DeleteOptions {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "DeleteOptions may be provided when deleting an API object.",
          "x-kubernetes-group-version-kind": [
            {
              "group": "",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "admission.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "admission.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "admissionregistration.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "admissionregistration.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "apiextensions.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "apiextensions.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "apiregistration.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "apiregistration.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "apps",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "apps",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "apps",
              "kind": "DeleteOptions",
              "version": "v1beta2"
            },
            {
              "group": "authentication.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "authentication.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "authorization.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "authorization.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "autoscaling",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "autoscaling",
              "kind": "DeleteOptions",
              "version": "v2beta1"
            },
            {
              "group": "autoscaling",
              "kind": "DeleteOptions",
              "version": "v2beta2"
            },
            {
              "group": "batch",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "batch",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "batch",
              "kind": "DeleteOptions",
              "version": "v2alpha1"
            },
            {
              "group": "certificates.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "certificates.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "coordination.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "coordination.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "discovery.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "discovery.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "events.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "events.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "extensions",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "flowcontrol.apiserver.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "flowcontrol.apiserver.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "imagepolicy.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "internal.apiserver.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "networking.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "networking.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "node.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "node.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "node.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "policy",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "rbac.authorization.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "rbac.authorization.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "rbac.authorization.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "scheduling.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "scheduling.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "scheduling.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            },
            {
              "group": "storage.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1"
            },
            {
              "group": "storage.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1alpha1"
            },
            {
              "group": "storage.k8s.io",
              "kind": "DeleteOptions",
              "version": "v1beta1"
            }
          ],
          "properties": {
            "apiVersion": {
              "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
              "type": "string"
            },
            "dryRun": {
              "description": "When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "gracePeriodSeconds": {
              "description": "The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.",
              "format": "int64",
              "type": "integer"
            },
            "kind": {
              "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
              "type": "string"
            },
            "orphanDependents": {
              "description": "Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.",
              "type": "boolean"
            },
            "preconditions": crate::schema_ref_with_values(crate::apimachinery::pkg::apis::meta::v1::Preconditions::schema(), serde_json::json!({"description": "Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned."})),
            "propagationPolicy": {
              "description": "Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
