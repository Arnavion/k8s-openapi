// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIPersistentVolumeSource {
    /// controllerExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerExpandVolume call. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub controller_expand_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// controllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// driver is the name of the driver to use for this volume. Required.
    pub driver: std::string::String,

    /// fsType to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs".
    pub fs_type: Option<std::string::String>,

    /// nodeExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeExpandVolume call. This field is optional, may be omitted if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_expand_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_publish_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// nodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    pub node_stage_secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// readOnly value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// volumeAttributes of the volume to publish.
    pub volume_attributes: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// volumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    pub volume_handle: std::string::String,
}

impl crate::DeepMerge for CSIPersistentVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.controller_expand_secret_ref, other.controller_expand_secret_ref);
        crate::DeepMerge::merge_from(&mut self.controller_publish_secret_ref, other.controller_publish_secret_ref);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.node_expand_secret_ref, other.node_expand_secret_ref);
        crate::DeepMerge::merge_from(&mut self.node_publish_secret_ref, other.node_publish_secret_ref);
        crate::DeepMerge::merge_from(&mut self.node_stage_secret_ref, other.node_stage_secret_ref);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::merge_strategies::map::granular(&mut self.volume_attributes, other.volume_attributes, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.volume_handle, other.volume_handle);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CSIPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller_expand_secret_ref,
            Key_controller_publish_secret_ref,
            Key_driver,
            Key_fs_type,
            Key_node_expand_secret_ref,
            Key_node_publish_secret_ref,
            Key_node_stage_secret_ref,
            Key_read_only,
            Key_volume_attributes,
            Key_volume_handle,
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
                            "controllerExpandSecretRef" => Field::Key_controller_expand_secret_ref,
                            "controllerPublishSecretRef" => Field::Key_controller_publish_secret_ref,
                            "driver" => Field::Key_driver,
                            "fsType" => Field::Key_fs_type,
                            "nodeExpandSecretRef" => Field::Key_node_expand_secret_ref,
                            "nodePublishSecretRef" => Field::Key_node_publish_secret_ref,
                            "nodeStageSecretRef" => Field::Key_node_stage_secret_ref,
                            "readOnly" => Field::Key_read_only,
                            "volumeAttributes" => Field::Key_volume_attributes,
                            "volumeHandle" => Field::Key_volume_handle,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSIPersistentVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_controller_expand_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_controller_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_node_expand_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_node_publish_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_node_stage_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_attributes: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_volume_handle: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller_expand_secret_ref => value_controller_expand_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller_publish_secret_ref => value_controller_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_expand_secret_ref => value_node_expand_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_publish_secret_ref => value_node_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_stage_secret_ref => value_node_stage_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes => value_volume_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_handle => value_volume_handle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIPersistentVolumeSource {
                    controller_expand_secret_ref: value_controller_expand_secret_ref,
                    controller_publish_secret_ref: value_controller_publish_secret_ref,
                    driver: value_driver.unwrap_or_default(),
                    fs_type: value_fs_type,
                    node_expand_secret_ref: value_node_expand_secret_ref,
                    node_publish_secret_ref: value_node_publish_secret_ref,
                    node_stage_secret_ref: value_node_stage_secret_ref,
                    read_only: value_read_only,
                    volume_attributes: value_volume_attributes,
                    volume_handle: value_volume_handle.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIPersistentVolumeSource",
            &[
                "controllerExpandSecretRef",
                "controllerPublishSecretRef",
                "driver",
                "fsType",
                "nodeExpandSecretRef",
                "nodePublishSecretRef",
                "nodeStageSecretRef",
                "readOnly",
                "volumeAttributes",
                "volumeHandle",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSIPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIPersistentVolumeSource",
            2 +
            self.controller_expand_secret_ref.as_ref().map_or(0, |_| 1) +
            self.controller_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.node_expand_secret_ref.as_ref().map_or(0, |_| 1) +
            self.node_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.node_stage_secret_ref.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.volume_attributes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller_expand_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controllerExpandSecretRef", value)?;
        }
        if let Some(value) = &self.controller_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controllerPublishSecretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.node_expand_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeExpandSecretRef", value)?;
        }
        if let Some(value) = &self.node_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodePublishSecretRef", value)?;
        }
        if let Some(value) = &self.node_stage_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeStageSecretRef", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.volume_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributes", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeHandle", &self.volume_handle)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSIPersistentVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.CSIPersistentVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents storage that is managed by an external CSI volume driver (Beta feature)".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "controllerExpandSecretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("controllerExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerExpandVolume call. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "controllerPublishSecretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("controllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "driver".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("driver is the name of the driver to use for this volume. Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fsType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeExpandSecretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeExpandVolume call. This field is optional, may be omitted if no secret is required. If the secret object contains more than one secret, all secrets are passed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nodePublishSecretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nodeStageSecretRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "readOnly".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeAttributes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeAttributes of the volume to publish.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeHandle".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "driver".into(),
                    "volumeHandle".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
