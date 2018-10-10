// Generated from definition io.k8s.api.authorization.v1.ResourceRule

/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
    pub api_groups: Option<Vec<String>>,

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    pub resource_names: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    pub resources: Option<Vec<String>>,

    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    pub verbs: Vec<String>,
}

impl<'de> ::serde::Deserialize<'de> for ResourceRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
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
            type Value = ResourceRule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ResourceRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_resource_names: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_names => value_resource_names = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verbs => value_verbs = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRule {
                    api_groups: value_api_groups,
                    resource_names: value_resource_names,
                    resources: value_resources,
                    verbs: value_verbs.ok_or_else(|| ::serde::de::Error::missing_field("verbs"))?,
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

impl ::serde::Serialize for ResourceRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRule",
            0 +
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.resource_names.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_groups {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
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
