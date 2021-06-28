// Generated from definition io.k8s.api.authorization.v1beta1.ResourceRule

/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
    pub api_groups: Vec<String>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    pub resource_names: Vec<String>,

    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    pub resources: Vec<String>,

    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verbs: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
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
            type Value = ResourceRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRule {
                    api_groups: value_api_groups.unwrap_or_default(),
                    resource_names: value_resource_names.unwrap_or_default(),
                    resources: value_resources.unwrap_or_default(),
                    verbs: value_verbs.ok_or_else(|| crate::serde::de::Error::missing_field("verbs"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceRule",
            &[
                "apiGroups",
                "resourceNames",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRule",
            1 +
            usize::from(!self.api_groups.is_empty()) +
            usize::from(!self.resource_names.is_empty()) +
            usize::from(!self.resources.is_empty()),
        )?;
        if !self.api_groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
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
impl crate::Schema for ResourceRule {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.",
          "properties": {
            "apiGroups": {
              "description": "APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  \"*\" means all.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "resourceNames": {
              "description": "ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  \"*\" means all.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "resources": {
              "description": "Resources is a list of resources this rule applies to.  \"*\" means all in the specified apiGroups.\n \"*/foo\" represents the subresource 'foo' for all resources in the specified apiGroups.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "verbs": {
              "description": "Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.",
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
