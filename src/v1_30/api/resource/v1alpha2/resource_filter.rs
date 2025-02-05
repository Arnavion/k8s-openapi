// Generated from definition io.k8s.api.resource.v1alpha2.ResourceFilter

/// ResourceFilter is a filter for resources from one particular driver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFilter {
    /// DriverName is the name used by the DRA driver kubelet plugin.
    pub driver_name: Option<std::string::String>,

    /// NamedResources describes a resource filter using the named resources model.
    pub named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesFilter>,
}

impl crate::DeepMerge for ResourceFilter {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::DeepMerge::merge_from(&mut self.named_resources, other.named_resources);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver_name,
            Key_named_resources,
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
                            "driverName" => Field::Key_driver_name,
                            "namedResources" => Field::Key_named_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceFilter;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceFilter")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver_name: Option<std::string::String> = None;
                let mut value_named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesFilter> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_named_resources => value_named_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceFilter {
                    driver_name: value_driver_name,
                    named_resources: value_named_resources,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceFilter",
            &[
                "driverName",
                "namedResources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceFilter",
            self.driver_name.as_ref().map_or(0, |_| 1) +
            self.named_resources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.driver_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", value)?;
        }
        if let Some(value) = &self.named_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namedResources", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceFilter {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceFilter".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceFilter is a filter for resources from one particular driver.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "driverName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName is the name used by the DRA driver kubelet plugin.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namedResources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesFilter>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NamedResources describes a resource filter using the named resources model.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
