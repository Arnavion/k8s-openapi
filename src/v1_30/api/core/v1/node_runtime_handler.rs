// Generated from definition io.k8s.api.core.v1.NodeRuntimeHandler

/// NodeRuntimeHandler is a set of runtime handler information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeRuntimeHandler {
    /// Supported features.
    pub features: Option<crate::api::core::v1::NodeRuntimeHandlerFeatures>,

    /// Runtime handler name. Empty for the default runtime handler.
    pub name: Option<std::string::String>,
}

impl crate::DeepMerge for NodeRuntimeHandler {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.features, other.features);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeRuntimeHandler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_features,
            Key_name,
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
                            "features" => Field::Key_features,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeRuntimeHandler;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeRuntimeHandler")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_features: Option<crate::api::core::v1::NodeRuntimeHandlerFeatures> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_features => value_features = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeRuntimeHandler {
                    features: value_features,
                    name: value_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeRuntimeHandler",
            &[
                "features",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeRuntimeHandler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeRuntimeHandler",
            self.features.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.features {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "features", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeRuntimeHandler {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.NodeRuntimeHandler".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeRuntimeHandler is a set of runtime handler information.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "features".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeRuntimeHandlerFeatures>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Supported features.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Runtime handler name. Empty for the default runtime handler.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
