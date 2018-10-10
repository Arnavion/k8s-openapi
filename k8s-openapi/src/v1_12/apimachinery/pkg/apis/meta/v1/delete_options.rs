// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.DeleteOptions

/// DeleteOptions may be provided when deleting an API object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteOptions {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<Vec<String>>,

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    pub preconditions: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Preconditions>,

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for DeleteOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeleteOptions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DeleteOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_dry_run: Option<Vec<String>> = None;
                let mut value_grace_period_seconds: Option<i64> = None;
                let mut value_kind: Option<String> = None;
                let mut value_orphan_dependents: Option<bool> = None;
                let mut value_preconditions: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Preconditions> = None;
                let mut value_propagation_policy: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dry_run => value_dry_run = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_grace_period_seconds => value_grace_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_orphan_dependents => value_orphan_dependents = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preconditions => value_preconditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_propagation_policy => value_propagation_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeleteOptions {
                    api_version: value_api_version,
                    dry_run: value_dry_run,
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

impl ::serde::Serialize for DeleteOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeleteOptions",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.dry_run.as_ref().map_or(0, |_| 1) +
            self.grace_period_seconds.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.orphan_dependents.as_ref().map_or(0, |_| 1) +
            self.preconditions.as_ref().map_or(0, |_| 1) +
            self.propagation_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.dry_run {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "dryRun", value)?;
        }
        if let Some(value) = &self.grace_period_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "gracePeriodSeconds", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.orphan_dependents {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "orphanDependents", value)?;
        }
        if let Some(value) = &self.preconditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "preconditions", value)?;
        }
        if let Some(value) = &self.propagation_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "propagationPolicy", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
