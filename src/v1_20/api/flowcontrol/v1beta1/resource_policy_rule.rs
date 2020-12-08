// Generated from definition io.k8s.api.flowcontrol.v1beta1.ResourcePolicyRule

/// ResourcePolicyRule is a predicate that matches some resource requests, testing the request's verb and the target resource. A ResourcePolicyRule matches a resource request if and only if: (a) at least one member of verbs matches the request, (b) at least one member of apiGroups matches the request, (c) at least one member of resources matches the request, and (d) least one member of namespaces matches the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePolicyRule {
    /// `apiGroups` is a list of matching API groups and may not be empty. "*" matches all API groups and, if present, must be the only entry. Required.
    pub api_groups: Vec<String>,

    /// `clusterScope` indicates whether to match requests that do not specify a namespace (which happens either because the resource is not namespaced or the request targets all namespaces). If this field is omitted or false then the `namespaces` field must contain a non-empty list.
    pub cluster_scope: Option<bool>,

    /// `namespaces` is a list of target namespaces that restricts matches.  A request that specifies a target namespace matches only if either (a) this list contains that target namespace or (b) this list contains "*".  Note that "*" matches any specified namespace but does not match a request that _does not specify_ a namespace (see the `clusterScope` field for that). This list may be empty, but only if `clusterScope` is true.
    pub namespaces: Option<Vec<String>>,

    /// `resources` is a list of matching resources (i.e., lowercase and plural) with, if desired, subresource.  For example, \[ "services", "nodes/status" \].  This list may not be empty. "*" matches all resources and, if present, must be the only entry. Required.
    pub resources: Vec<String>,

    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs and, if present, must be the only entry. Required.
    pub verbs: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ResourcePolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_cluster_scope,
            Key_namespaces,
            Key_resources,
            Key_verbs,
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
                            "clusterScope" => Field::Key_cluster_scope,
                            "namespaces" => Field::Key_namespaces,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourcePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_cluster_scope: Option<bool> = None;
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;
                let mut value_verbs: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_cluster_scope => value_cluster_scope = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaces => value_namespaces = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_verbs => value_verbs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourcePolicyRule {
                    api_groups: value_api_groups.ok_or_else(|| serde::de::Error::missing_field("apiGroups"))?,
                    cluster_scope: value_cluster_scope,
                    namespaces: value_namespaces,
                    resources: value_resources.ok_or_else(|| serde::de::Error::missing_field("resources"))?,
                    verbs: value_verbs.ok_or_else(|| serde::de::Error::missing_field("verbs"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourcePolicyRule",
            &[
                "apiGroups",
                "clusterScope",
                "namespaces",
                "resources",
                "verbs",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ResourcePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourcePolicyRule",
            3 +
            self.cluster_scope.as_ref().map_or(0, |_| 1) +
            self.namespaces.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", &self.api_groups)?;
        if let Some(value) = &self.cluster_scope {
            serde::ser::SerializeStruct::serialize_field(&mut state, "clusterScope", value)?;
        }
        if let Some(value) = &self.namespaces {
            serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        serde::ser::SerializeStruct::end(state)
    }
}
