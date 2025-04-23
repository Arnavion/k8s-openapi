// Generated from definition io.k8s.api.core.v1.ContainerResizePolicy

/// ContainerResizePolicy represents resource resize policy for the container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerResizePolicy {
    /// Name of the resource to which this resource resize policy applies. Supported values: cpu, memory.
    pub resource_name: std::string::String,

    /// Restart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.
    pub restart_policy: std::string::String,
}

impl crate::DeepMerge for ContainerResizePolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.resource_name, other.resource_name);
        crate::DeepMerge::merge_from(&mut self.restart_policy, other.restart_policy);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerResizePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource_name,
            Key_restart_policy,
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
                            "resourceName" => Field::Key_resource_name,
                            "restartPolicy" => Field::Key_restart_policy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerResizePolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerResizePolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_resource_name: Option<std::string::String> = None;
                let mut value_restart_policy: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource_name => value_resource_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_policy => value_restart_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerResizePolicy {
                    resource_name: value_resource_name.unwrap_or_default(),
                    restart_policy: value_restart_policy.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerResizePolicy",
            &[
                "resourceName",
                "restartPolicy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerResizePolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerResizePolicy",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceName", &self.resource_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartPolicy", &self.restart_policy)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerResizePolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ContainerResizePolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerResizePolicy represents resource resize policy for the container.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "resourceName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the resource to which this resource resize policy applies. Supported values: cpu, memory.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "restartPolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Restart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resourceName".into(),
                    "restartPolicy".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
