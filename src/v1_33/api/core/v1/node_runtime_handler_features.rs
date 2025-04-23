// Generated from definition io.k8s.api.core.v1.NodeRuntimeHandlerFeatures

/// NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeRuntimeHandlerFeatures {
    /// RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
    pub recursive_read_only_mounts: Option<bool>,

    /// UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
    pub user_namespaces: Option<bool>,
}

impl crate::DeepMerge for NodeRuntimeHandlerFeatures {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.recursive_read_only_mounts, other.recursive_read_only_mounts);
        crate::DeepMerge::merge_from(&mut self.user_namespaces, other.user_namespaces);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeRuntimeHandlerFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_recursive_read_only_mounts,
            Key_user_namespaces,
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
                            "recursiveReadOnlyMounts" => Field::Key_recursive_read_only_mounts,
                            "userNamespaces" => Field::Key_user_namespaces,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeRuntimeHandlerFeatures;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeRuntimeHandlerFeatures")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_recursive_read_only_mounts: Option<bool> = None;
                let mut value_user_namespaces: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_recursive_read_only_mounts => value_recursive_read_only_mounts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user_namespaces => value_user_namespaces = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeRuntimeHandlerFeatures {
                    recursive_read_only_mounts: value_recursive_read_only_mounts,
                    user_namespaces: value_user_namespaces,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeRuntimeHandlerFeatures",
            &[
                "recursiveReadOnlyMounts",
                "userNamespaces",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeRuntimeHandlerFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeRuntimeHandlerFeatures",
            self.recursive_read_only_mounts.as_ref().map_or(0, |_| 1) +
            self.user_namespaces.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.recursive_read_only_mounts {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "recursiveReadOnlyMounts", value)?;
        }
        if let Some(value) = &self.user_namespaces {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "userNamespaces", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeRuntimeHandlerFeatures {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodeRuntimeHandlerFeatures".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "recursiveReadOnlyMounts".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "userNamespaces".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
