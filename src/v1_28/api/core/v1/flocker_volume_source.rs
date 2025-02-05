// Generated from definition io.k8s.api.core.v1.FlockerVolumeSource

/// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlockerVolumeSource {
    /// datasetName is Name of the dataset stored as metadata -\> name on the dataset for Flocker should be considered as deprecated
    pub dataset_name: Option<std::string::String>,

    /// datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
    pub dataset_uuid: Option<std::string::String>,
}

impl crate::DeepMerge for FlockerVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.dataset_name, other.dataset_name);
        crate::DeepMerge::merge_from(&mut self.dataset_uuid, other.dataset_uuid);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FlockerVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_dataset_name,
            Key_dataset_uuid,
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
                            "datasetName" => Field::Key_dataset_name,
                            "datasetUUID" => Field::Key_dataset_uuid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlockerVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FlockerVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_dataset_name: Option<std::string::String> = None;
                let mut value_dataset_uuid: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_dataset_name => value_dataset_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dataset_uuid => value_dataset_uuid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlockerVolumeSource {
                    dataset_name: value_dataset_name,
                    dataset_uuid: value_dataset_uuid,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlockerVolumeSource",
            &[
                "datasetName",
                "datasetUUID",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlockerVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlockerVolumeSource",
            self.dataset_name.as_ref().map_or(0, |_| 1) +
            self.dataset_uuid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.dataset_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "datasetName", value)?;
        }
        if let Some(value) = &self.dataset_uuid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "datasetUUID", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FlockerVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.FlockerVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "datasetName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "datasetUUID".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset".into()),
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
