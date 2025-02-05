// Generated from definition io.k8s.api.resource.v1alpha2.ResourceHandle

/// ResourceHandle holds opaque resource data for processing by a specific kubelet plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceHandle {
    /// Data contains the opaque data associated with this ResourceHandle. It is set by the controller component of the resource driver whose name matches the DriverName set in the ResourceClaimStatus this ResourceHandle is embedded in. It is set at allocation time and is intended for processing by the kubelet plugin whose name matches the DriverName set in this ResourceHandle.
    ///
    /// The maximum size of this field is 16KiB. This may get increased in the future, but not reduced.
    pub data: Option<std::string::String>,

    /// DriverName specifies the name of the resource driver whose kubelet plugin should be invoked to process this ResourceHandle's data once it lands on a node. This may differ from the DriverName set in ResourceClaimStatus this ResourceHandle is embedded in.
    pub driver_name: Option<std::string::String>,

    /// If StructuredData is set, then it needs to be used instead of Data.
    pub structured_data: Option<crate::api::resource::v1alpha2::StructuredResourceHandle>,
}

impl crate::DeepMerge for ResourceHandle {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.data, other.data);
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::DeepMerge::merge_from(&mut self.structured_data, other.structured_data);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_data,
            Key_driver_name,
            Key_structured_data,
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
                            "data" => Field::Key_data,
                            "driverName" => Field::Key_driver_name,
                            "structuredData" => Field::Key_structured_data,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceHandle;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceHandle")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_data: Option<std::string::String> = None;
                let mut value_driver_name: Option<std::string::String> = None;
                let mut value_structured_data: Option<crate::api::resource::v1alpha2::StructuredResourceHandle> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_data => value_data = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_structured_data => value_structured_data = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceHandle {
                    data: value_data,
                    driver_name: value_driver_name,
                    structured_data: value_structured_data,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceHandle",
            &[
                "data",
                "driverName",
                "structuredData",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceHandle",
            self.data.as_ref().map_or(0, |_| 1) +
            self.driver_name.as_ref().map_or(0, |_| 1) +
            self.structured_data.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.data {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "data", value)?;
        }
        if let Some(value) = &self.driver_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", value)?;
        }
        if let Some(value) = &self.structured_data {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "structuredData", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceHandle {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceHandle".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceHandle holds opaque resource data for processing by a specific kubelet plugin.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "data".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Data contains the opaque data associated with this ResourceHandle. It is set by the controller component of the resource driver whose name matches the DriverName set in the ResourceClaimStatus this ResourceHandle is embedded in. It is set at allocation time and is intended for processing by the kubelet plugin whose name matches the DriverName set in this ResourceHandle.\n\nThe maximum size of this field is 16KiB. This may get increased in the future, but not reduced.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "driverName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName specifies the name of the resource driver whose kubelet plugin should be invoked to process this ResourceHandle's data once it lands on a node. This may differ from the DriverName set in ResourceClaimStatus this ResourceHandle is embedded in.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "structuredData".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::StructuredResourceHandle>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If StructuredData is set, then it needs to be used instead of Data.".into()),
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
