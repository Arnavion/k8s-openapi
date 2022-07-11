// Generated from definition io.k8s.api.core.v1.ResourceFieldSelector

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    pub container_name: Option<String>,

    /// Specifies the output format of the exposed resources, defaults to "1"
    pub divisor: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// Required: resource to select
    pub resource: String,

}

#[cfg(feature = "dsl")]
impl ResourceFieldSelector  {
    /// Set [`Self::container_name`]
    pub  fn container_name_set(&mut self, container_name: impl Into<Option<String>>) -> &mut Self {
        self.container_name = container_name.into(); self
    }

    pub  fn container_name(&mut self) -> &mut String {
        if self.container_name.is_none() { self.container_name = Some(Default::default()) }
        self.container_name.as_mut().unwrap()
    }

    /// Modify [`Self::container_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn container_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.container_name.is_none() { self.container_name = Some(Default::default()) };
        func(self.container_name.as_mut().unwrap()); self
    }


    /// Set [`Self::divisor`]
    pub  fn divisor_set(&mut self, divisor: impl Into<Option<crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
        self.divisor = divisor.into(); self
    }

    pub  fn divisor(&mut self) -> &mut crate::apimachinery::pkg::api::resource::Quantity {
        if self.divisor.is_none() { self.divisor = Some(Default::default()) }
        self.divisor.as_mut().unwrap()
    }

    /// Modify [`Self::divisor`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn divisor_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.divisor.is_none() { self.divisor = Some(Default::default()) };
        func(self.divisor.as_mut().unwrap()); self
    }


    /// Set [`Self::resource`]
    pub  fn resource_set(&mut self, resource: impl Into<String>) -> &mut Self {
        self.resource = resource.into(); self
    }

    pub  fn resource(&mut self) -> &mut String {
        &mut self.resource
    }

    /// Modify [`Self::resource`] with a `func`
    pub  fn resource_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.resource); self
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceFieldSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_name: Option<String> = None;
                let mut value_divisor: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_resource: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ResourceFieldSelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceFieldSelector represents container resources (cpu, memory) and their output format".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Container name: required for volumes, optional for env vars".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "divisor".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the output format of the exposed resources, defaults to \"1\"".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resource".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required: resource to select".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resource".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
