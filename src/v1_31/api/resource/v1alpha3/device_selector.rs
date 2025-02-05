// Generated from definition io.k8s.api.resource.v1alpha3.DeviceSelector

/// DeviceSelector must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceSelector {
    /// CEL contains a CEL expression for selecting a device.
    pub cel: Option<crate::api::resource::v1alpha3::CELDeviceSelector>,
}

impl crate::DeepMerge for DeviceSelector {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.cel, other.cel);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cel,
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
                            "cel" => Field::Key_cel,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cel: Option<crate::api::resource::v1alpha3::CELDeviceSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cel => value_cel = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceSelector {
                    cel: value_cel,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceSelector",
            &[
                "cel",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceSelector",
            self.cel.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cel {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cel", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceSelector {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceSelector must have exactly one field set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "cel".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::CELDeviceSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("CEL contains a CEL expression for selecting a device.".into()),
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
