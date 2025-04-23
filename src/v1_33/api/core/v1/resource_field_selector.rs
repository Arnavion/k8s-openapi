// Generated from definition io.k8s.api.core.v1.ResourceFieldSelector

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    pub container_name: Option<std::string::String>,

    /// Specifies the output format of the exposed resources, defaults to "1"
    pub divisor: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// Required: resource to select
    pub resource: std::string::String,
}

impl crate::DeepMerge for ResourceFieldSelector {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container_name, other.container_name);
        crate::DeepMerge::merge_from(&mut self.divisor, other.divisor);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceFieldSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_name,
            Key_divisor,
            Key_resource,
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
                            "containerName" => Field::Key_container_name,
                            "divisor" => Field::Key_divisor,
                            "resource" => Field::Key_resource,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceFieldSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceFieldSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_name: Option<std::string::String> = None;
                let mut value_divisor: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_resource: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_name => value_container_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_divisor => value_divisor = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceFieldSelector {
                    container_name: value_container_name,
                    divisor: value_divisor,
                    resource: value_resource.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceFieldSelector",
            &[
                "containerName",
                "divisor",
                "resource",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceFieldSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceFieldSelector",
            1 +
            self.container_name.as_ref().map_or(0, |_| 1) +
            self.divisor.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", value)?;
        }
        if let Some(value) = &self.divisor {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "divisor", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceFieldSelector {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ResourceFieldSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceFieldSelector represents container resources (cpu, memory) and their output format".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Container name: required for volumes, optional for env vars".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "divisor".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the output format of the exposed resources, defaults to \"1\"".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resource".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required: resource to select".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resource".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
