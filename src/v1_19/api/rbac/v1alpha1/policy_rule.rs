// Generated from definition io.k8s.api.rbac.v1alpha1.PolicyRule

/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.
    pub api_groups: Vec<String>,

    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    pub non_resource_urls: Vec<String>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    pub resource_names: Vec<String>,

    /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
    pub resources: Vec<String>,

    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
    pub verbs: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for PolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_non_resource_urls,
            Key_resource_names,
            Key_resources,
            Key_verbs,
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
                            "nonResourceURLs" => Field::Key_non_resource_urls,
                            "resourceNames" => Field::Key_resource_names,
                            "resources" => Field::Key_resources,
                            "verbs" => Field::Key_verbs,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_urls => value_non_resource_urls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRule {
                    api_groups: value_api_groups.unwrap_or_default(),
                    non_resource_urls: value_non_resource_urls.unwrap_or_default(),
                    resource_names: value_resource_names.unwrap_or_default(),
                    resources: value_resources.unwrap_or_default(),
                    verbs: value_verbs.ok_or_else(|| crate::serde::de::Error::missing_field("verbs"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PolicyRule",
            &[
                "apiGroups",
                "nonResourceURLs",
                "resourceNames",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRule",
            1 +
            usize::from(!self.api_groups.is_empty()) +
            usize::from(!self.non_resource_urls.is_empty()) +
            usize::from(!self.resource_names.is_empty()) +
            usize::from(!self.resources.is_empty()),
        )?;
        if !self.api_groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
        }
        if !self.non_resource_urls.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", &self.non_resource_urls)?;
        }
        if !self.resource_names.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", &self.resource_names)?;
        }
        if !self.resources.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PolicyRule {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.",
          "properties": {
            "apiGroups": {
              "description": "APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "nonResourceURLs": {
              "description": "NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "resourceNames": {
              "description": "ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "resources": {
              "description": "Resources is a list of resources this rule applies to.  ResourceAll represents all resources.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "verbs": {
              "description": "Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "required": [
            "verbs"
          ],
          "type": "object"
        })
    }
}
