// Generated from definition io.k8s.api.core.v1.NodeFeatures

/// NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeFeatures {
    /// SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
    pub supplemental_groups_policy: Option<bool>,
}

impl crate::DeepMerge for NodeFeatures {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.supplemental_groups_policy, other.supplemental_groups_policy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_supplemental_groups_policy,
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
                            "supplementalGroupsPolicy" => Field::Key_supplemental_groups_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeFeatures;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeFeatures")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_supplemental_groups_policy: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_supplemental_groups_policy => value_supplemental_groups_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeFeatures {
                    supplemental_groups_policy: value_supplemental_groups_policy,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeFeatures",
            &[
                "supplementalGroupsPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeFeatures",
            self.supplemental_groups_policy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.supplemental_groups_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroupsPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeFeatures {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodeFeatures".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.",
            "type": "object",
            "properties": {
                "supplementalGroupsPolicy": {
                    "description": "SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.",
                    "type": "boolean",
                },
            },
        })
    }
}
