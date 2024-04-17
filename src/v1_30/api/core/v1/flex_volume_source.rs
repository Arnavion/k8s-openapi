// Generated from definition io.k8s.api.core.v1.FlexVolumeSource

/// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexVolumeSource {
    /// driver is the name of the driver to use for this volume.
    pub driver: String,

    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    pub fs_type: Option<String>,

    /// options is Optional: this field holds extra command options if any.
    pub options: Option<std::collections::BTreeMap<String, String>>,

    /// readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretRef is Optional: secretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,
}

impl crate::DeepMerge for FlexVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::merge_strategies::map::granular(&mut self.options, other.options, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FlexVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_options,
            Key_read_only,
            Key_secret_ref,
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
                            "driver" => Field::Key_driver,
                            "fsType" => Field::Key_fs_type,
                            "options" => Field::Key_options,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlexVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlexVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_options: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlexVolumeSource {
                    driver: value_driver.unwrap_or_default(),
                    fs_type: value_fs_type,
                    options: value_options,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlexVolumeSource",
            &[
                "driver",
                "fsType",
                "options",
                "readOnly",
                "secretRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlexVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlexVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FlexVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.FlexVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "driver".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("driver is the name of the driver to use for this volume.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default filesystem depends on FlexVolume script.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "options".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("options is Optional: this field holds extra command options if any.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretRef is Optional: secretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "driver".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
