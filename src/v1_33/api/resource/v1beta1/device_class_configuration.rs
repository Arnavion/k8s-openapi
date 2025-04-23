// Generated from definition io.k8s.api.resource.v1beta1.DeviceClassConfiguration

/// DeviceClassConfiguration is used in DeviceClass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceClassConfiguration {
    /// Opaque provides driver-specific configuration parameters.
    pub opaque: Option<crate::api::resource::v1beta1::OpaqueDeviceConfiguration>,
}

impl crate::DeepMerge for DeviceClassConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.opaque, other.opaque);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceClassConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_opaque,
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
                            "opaque" => Field::Key_opaque,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceClassConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceClassConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_opaque: Option<crate::api::resource::v1beta1::OpaqueDeviceConfiguration> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_opaque => value_opaque = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceClassConfiguration {
                    opaque: value_opaque,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceClassConfiguration",
            &[
                "opaque",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceClassConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceClassConfiguration",
            self.opaque.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.opaque {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "opaque", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceClassConfiguration {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.DeviceClassConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceClassConfiguration is used in DeviceClass.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "opaque".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::OpaqueDeviceConfiguration>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Opaque provides driver-specific configuration parameters.".into()),
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
