// Generated from definition io.k8s.api.rbac.v1alpha1.PolicyRule

/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.
    pub api_groups: Option<Vec<String>>,

    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path This name is intentionally different than the internal type so that the DefaultConvert works nicely and because the ordering may be different. Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    pub non_resource_urls: Option<Vec<String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    pub resource_names: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
    pub resources: Option<Vec<String>>,

    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
    pub verbs: Vec<String>,
}

impl<'de> ::serde::Deserialize<'de> for PolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_non_resource_urls,
            Key_resource_names,
            Key_resources,
            Key_verbs,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_non_resource_urls: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_non_resource_urls => value_non_resource_urls = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRule {
                    api_groups: value_api_groups,
                    non_resource_urls: value_non_resource_urls,
                    resource_names: value_resource_names,
                    resources: value_resources,
                    verbs: value_verbs.ok_or_else(|| ::serde::de::Error::missing_field("verbs"))?,
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

impl ::serde::Serialize for PolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRule",
            0 +
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.non_resource_urls.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_groups {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.non_resource_urls {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceURLs", value)?;
        }
        if let Some(value) = &self.resource_names {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceNames", value)?;
        }
        if let Some(value) = &self.resources {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
