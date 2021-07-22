// Generated from definition io.k8s.api.core.v1.ResourceQuotaSpec

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaSpec {
    /// hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    pub hard: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    pub scope_selector: Option<crate::api::core::v1::ScopeSelector>,

    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    pub scopes: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceQuotaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hard,
            Key_scope_selector,
            Key_scopes,
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
                            "hard" => Field::Key_hard,
                            "scopeSelector" => Field::Key_scope_selector,
                            "scopes" => Field::Key_scopes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceQuotaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hard: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_scope_selector: Option<crate::api::core::v1::ScopeSelector> = None;
                let mut value_scopes: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hard => value_hard = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope_selector => value_scope_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scopes => value_scopes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceQuotaSpec {
                    hard: value_hard.unwrap_or_default(),
                    scope_selector: value_scope_selector,
                    scopes: value_scopes.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceQuotaSpec",
            &[
                "hard",
                "scopeSelector",
                "scopes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceQuotaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaSpec",
            usize::from(!self.hard.is_empty()) +
            self.scope_selector.as_ref().map_or(0, |_| 1) +
            usize::from(!self.scopes.is_empty()),
        )?;
        if !self.hard.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hard", &self.hard)?;
        }
        if let Some(value) = &self.scope_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopeSelector", value)?;
        }
        if !self.scopes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", &self.scopes)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ResourceQuotaSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceQuotaSpec defines the desired hard limits to enforce for Quota.",
          "properties": {
            "hard": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/",
              "type": "object"
            },
            "scopeSelector": crate::schema_ref_with_values(crate::api::core::v1::ScopeSelector::schema(), serde_json::json!({"description": "scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched."})),
            "scopes": {
              "description": "A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.",
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
