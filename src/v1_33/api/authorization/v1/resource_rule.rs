// Generated from definition io.k8s.api.authorization.v1.ResourceRule

/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
    pub api_groups: Option<std::vec::Vec<std::string::String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    pub resource_names: Option<std::vec::Vec<std::string::String>>,

    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    pub resources: Option<std::vec::Vec<std::string::String>>,

    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verbs: std::vec::Vec<std::string::String>,
}

impl crate::DeepMerge for ResourceRule {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.api_groups, other.api_groups);
        crate::merge_strategies::list::atomic(&mut self.resource_names, other.resource_names);
        crate::merge_strategies::list::atomic(&mut self.resources, other.resources);
        crate::merge_strategies::list::atomic(&mut self.verbs, other.verbs);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_resource_names: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_resources: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_verbs: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRule {
                    api_groups: value_api_groups,
                    resource_names: value_resource_names,
                    resources: value_resources,
                    verbs: value_verbs.unwrap_or_default(),
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
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.resource_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authorization.v1.ResourceRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.",
            "type": "object",
            "properties": {
                "apiGroups": {
                    "description": "APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  \"*\" means all.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "resourceNames": {
                    "description": "ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  \"*\" means all.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "resources": {
                    "description": "Resources is a list of resources this rule applies to.  \"*\" means all in the specified apiGroups.\n \"*/foo\" represents the subresource 'foo' for all resources in the specified apiGroups.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "verbs": {
                    "description": "Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "verbs",
            ],
        })
    }
}
