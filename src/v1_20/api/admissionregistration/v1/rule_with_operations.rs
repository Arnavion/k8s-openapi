// Generated from definition io.k8s.api.admissionregistration.v1.RuleWithOperations

/// RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleWithOperations {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    pub api_groups: Option<Vec<String>>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    pub api_versions: Option<Vec<String>>,

    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.
    pub operations: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    pub resources: Option<Vec<String>>,

    /// scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "*".
    pub scope: Option<String>,
}

impl<'de> serde::Deserialize<'de> for RuleWithOperations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_api_versions,
            Key_operations,
            Key_resources,
            Key_scope,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiGroups" => Field::Key_api_groups,
                            "apiVersions" => Field::Key_api_versions,
                            "operations" => Field::Key_operations,
                            "resources" => Field::Key_resources,
                            "scope" => Field::Key_scope,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleWithOperations;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RuleWithOperations")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_api_versions: Option<Vec<String>> = None;
                let mut value_operations: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_scope: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_versions => value_api_versions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operations => value_operations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuleWithOperations {
                    api_groups: value_api_groups,
                    api_versions: value_api_versions,
                    operations: value_operations,
                    resources: value_resources,
                    scope: value_scope,
                })
            }
        }

        deserializer.deserialize_struct(
            "RuleWithOperations",
            &[
                "apiGroups",
                "apiVersions",
                "operations",
                "resources",
                "scope",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RuleWithOperations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RuleWithOperations",
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.api_versions.as_ref().map_or(0, |_| 1) +
            self.operations.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.scope.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.api_versions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersions", value)?;
        }
        if let Some(value) = &self.operations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "operations", value)?;
        }
        if let Some(value) = &self.resources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.scope {
            serde::ser::SerializeStruct::serialize_field(&mut state, "scope", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
