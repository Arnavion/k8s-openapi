// Generated from definition io.k8s.api.admissionregistration.v1beta1.RuleWithOperations

/// RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleWithOperations {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    pub api_groups: Vec<String>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    pub api_versions: Vec<String>,

    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, or * for all operations. If '*' is present, the length of the slice must be one. Required.
    pub operations: Vec<String>,

    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    pub resources: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for RuleWithOperations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_api_versions,
            Key_operations,
            Key_resources,
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
                            "apiGroups" => Field::Key_api_groups,
                            "apiVersions" => Field::Key_api_versions,
                            "operations" => Field::Key_operations,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RuleWithOperations;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RuleWithOperations")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_api_versions: Option<Vec<String>> = None;
                let mut value_operations: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_versions => value_api_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operations => value_operations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuleWithOperations {
                    api_groups: value_api_groups.unwrap_or_default(),
                    api_versions: value_api_versions.unwrap_or_default(),
                    operations: value_operations.unwrap_or_default(),
                    resources: value_resources.unwrap_or_default(),
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
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RuleWithOperations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RuleWithOperations",
            usize::from(!self.api_groups.is_empty()) +
            usize::from(!self.api_versions.is_empty()) +
            usize::from(!self.operations.is_empty()) +
            usize::from(!self.resources.is_empty()),
        )?;
        if !self.api_groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
        }
        if !self.api_versions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersions", &self.api_versions)?;
        }
        if !self.operations.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operations", &self.operations)?;
        }
        if !self.resources.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for RuleWithOperations {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.",
          "properties": {
            "apiGroups": {
              "description": "APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "apiVersions": {
              "description": "APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "operations": {
              "description": "Operations is the operations the admission hook cares about - CREATE, UPDATE, or * for all operations. If '*' is present, the length of the slice must be one. Required.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "resources": {
              "description": "Resources is a list of resources this rule applies to.\n\nFor example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.\n\nIf wildcard is present, the validation rule will ensure resources do not overlap with each other.\n\nDepending on the enclosing object, subresources might not be allowed. Required.",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
