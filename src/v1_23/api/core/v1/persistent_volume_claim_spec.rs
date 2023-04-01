// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimSpec

/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<Vec<String>>,

    /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
    pub data_source: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef
    ///   allows any non-core object, as well as PersistentVolumeClaim objects.
    /// * While DataSource ignores disallowed values (dropping them), DataSourceRef
    ///   preserves all values, and generates an error if a disallowed value is
    ///   specified.
    /// (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
    pub data_source_ref: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// A label query over volumes to consider for binding.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    pub storage_class_name: Option<String>,

    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
    pub volume_mode: Option<String>,

    /// VolumeName is the binding reference to the PersistentVolume backing this claim.
    pub volume_name: Option<String>,
}

impl crate::DeepMerge for PersistentVolumeClaimSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.access_modes, other.access_modes);
        crate::DeepMerge::merge_from(&mut self.data_source, other.data_source);
        crate::DeepMerge::merge_from(&mut self.data_source_ref, other.data_source_ref);
        crate::DeepMerge::merge_from(&mut self.resources, other.resources);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.storage_class_name, other.storage_class_name);
        crate::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
        crate::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_data_source,
            Key_data_source_ref,
            Key_resources,
            Key_selector,
            Key_storage_class_name,
            Key_volume_mode,
            Key_volume_name,
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
                            "accessModes" => Field::Key_access_modes,
                            "dataSource" => Field::Key_data_source,
                            "dataSourceRef" => Field::Key_data_source_ref,
                            "resources" => Field::Key_resources,
                            "selector" => Field::Key_selector,
                            "storageClassName" => Field::Key_storage_class_name,
                            "volumeMode" => Field::Key_volume_mode,
                            "volumeName" => Field::Key_volume_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_data_source: Option<crate::api::core::v1::TypedLocalObjectReference> = None;
                let mut value_data_source_ref: Option<crate::api::core::v1::TypedLocalObjectReference> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_storage_class_name: Option<String> = None;
                let mut value_volume_mode: Option<String> = None;
                let mut value_volume_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source => value_data_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source_ref => value_data_source_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mode => value_volume_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimSpec {
                    access_modes: value_access_modes,
                    data_source: value_data_source,
                    data_source_ref: value_data_source_ref,
                    resources: value_resources,
                    selector: value_selector,
                    storage_class_name: value_storage_class_name,
                    volume_mode: value_volume_mode,
                    volume_name: value_volume_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimSpec",
            &[
                "accessModes",
                "dataSource",
                "dataSourceRef",
                "resources",
                "selector",
                "storageClassName",
                "volumeMode",
                "volumeName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeClaimSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimSpec",
            self.access_modes.as_ref().map_or(0, |_| 1) +
            self.data_source.as_ref().map_or(0, |_| 1) +
            self.data_source_ref.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.storage_class_name.as_ref().map_or(0, |_| 1) +
            self.volume_mode.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_modes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.data_source {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dataSource", value)?;
        }
        if let Some(value) = &self.data_source_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dataSourceRef", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.storage_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageClassName", value)?;
        }
        if let Some(value) = &self.volume_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMode", value)?;
        }
        if let Some(value) = &self.volume_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeClaimSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PersistentVolumeClaimSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "accessModes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "dataSource".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TypedLocalObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "dataSourceRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TypedLocalObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef\n  allows any non-core object, as well as PersistentVolumeClaim objects.\n* While DataSource ignores disallowed values (dropping them), DataSourceRef\n  preserves all values, and generates an error if a disallowed value is\n  specified.\n(Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resources".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ResourceRequirements>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over volumes to consider for binding.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageClassName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VolumeName is the binding reference to the PersistentVolume backing this claim.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
