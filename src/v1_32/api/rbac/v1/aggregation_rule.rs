// Generated from definition io.k8s.api.rbac.v1.AggregationRule

/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
    pub cluster_role_selectors: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>,
}

impl crate::DeepMerge for AggregationRule {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.cluster_role_selectors, other.cluster_role_selectors);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AggregationRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cluster_role_selectors,
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
                            "clusterRoleSelectors" => Field::Key_cluster_role_selectors,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AggregationRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("AggregationRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cluster_role_selectors: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_role_selectors => value_cluster_role_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AggregationRule {
                    cluster_role_selectors: value_cluster_role_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "AggregationRule",
            &[
                "clusterRoleSelectors",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AggregationRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AggregationRule",
            self.cluster_role_selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cluster_role_selectors {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterRoleSelectors", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AggregationRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.rbac.v1.AggregationRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole",
            "type": "object",
            "properties": {
                "clusterRoleSelectors": {
                    "description": "ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>()),
                },
            },
        })
    }
}
