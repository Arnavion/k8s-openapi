// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimSpec

/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimSpec {
    /// accessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<std::vec::Vec<std::string::String>>,

    /// dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
    pub data_source: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef
    ///   allows any non-core object, as well as PersistentVolumeClaim objects.
    /// * While dataSource ignores disallowed values (dropping them), dataSourceRef
    ///   preserves all values, and generates an error if a disallowed value is
    ///   specified.
    /// * While dataSource only allows local objects, dataSourceRef allows objects
    ///   in any namespaces.
    /// (Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
    pub data_source_ref: Option<crate::api::core::v1::TypedObjectReference>,

    /// resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    pub resources: Option<crate::api::core::v1::VolumeResourceRequirements>,

    /// selector is a label query over volumes to consider for binding.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// storageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    pub storage_class_name: Option<std::string::String>,

    /// volumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim. If specified, the CSI driver will create or update the volume with the attributes defined in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName, it can be changed after the claim is created. An empty string value means that no VolumeAttributesClass will be applied to the claim but it's not allowed to reset this field to empty string once it is set. If unspecified and the PersistentVolumeClaim is unbound, the default VolumeAttributesClass will be set by the persistentvolume controller if it exists. If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource exists. More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/ (Beta) Using this field requires the VolumeAttributesClass feature gate to be enabled (off by default).
    pub volume_attributes_class_name: Option<std::string::String>,

    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
    pub volume_mode: Option<std::string::String>,

    /// volumeName is the binding reference to the PersistentVolume backing this claim.
    pub volume_name: Option<std::string::String>,
}

impl crate::DeepMerge for PersistentVolumeClaimSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.access_modes, other.access_modes);
        crate::DeepMerge::merge_from(&mut self.data_source, other.data_source);
        crate::DeepMerge::merge_from(&mut self.data_source_ref, other.data_source_ref);
        crate::DeepMerge::merge_from(&mut self.resources, other.resources);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.storage_class_name, other.storage_class_name);
        crate::DeepMerge::merge_from(&mut self.volume_attributes_class_name, other.volume_attributes_class_name);
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
            Key_volume_attributes_class_name,
            Key_volume_mode,
            Key_volume_name,
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
                            "accessModes" => Field::Key_access_modes,
                            "dataSource" => Field::Key_data_source,
                            "dataSourceRef" => Field::Key_data_source_ref,
                            "resources" => Field::Key_resources,
                            "selector" => Field::Key_selector,
                            "storageClassName" => Field::Key_storage_class_name,
                            "volumeAttributesClassName" => Field::Key_volume_attributes_class_name,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PersistentVolumeClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_data_source: Option<crate::api::core::v1::TypedLocalObjectReference> = None;
                let mut value_data_source_ref: Option<crate::api::core::v1::TypedObjectReference> = None;
                let mut value_resources: Option<crate::api::core::v1::VolumeResourceRequirements> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_storage_class_name: Option<std::string::String> = None;
                let mut value_volume_attributes_class_name: Option<std::string::String> = None;
                let mut value_volume_mode: Option<std::string::String> = None;
                let mut value_volume_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source => value_data_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source_ref => value_data_source_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes_class_name => value_volume_attributes_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    volume_attributes_class_name: value_volume_attributes_class_name,
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
                "volumeAttributesClassName",
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
            self.volume_attributes_class_name.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.volume_attributes_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributesClassName", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PersistentVolumeClaimSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "accessModes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("accessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "dataSource".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TypedLocalObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "dataSourceRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::TypedObjectReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef\n  allows any non-core object, as well as PersistentVolumeClaim objects.\n* While dataSource ignores disallowed values (dropping them), dataSourceRef\n  preserves all values, and generates an error if a disallowed value is\n  specified.\n* While dataSource only allows local objects, dataSourceRef allows objects\n  in any namespaces.\n(Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::VolumeResourceRequirements>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "selector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("selector is a label query over volumes to consider for binding.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("storageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeAttributesClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim. If specified, the CSI driver will create or update the volume with the attributes defined in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName, it can be changed after the claim is created. An empty string value means that no VolumeAttributesClass will be applied to the claim but it's not allowed to reset this field to empty string once it is set. If unspecified and the PersistentVolumeClaim is unbound, the default VolumeAttributesClass will be set by the persistentvolume controller if it exists. If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource exists. More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/ (Beta) Using this field requires the VolumeAttributesClass feature gate to be enabled (off by default).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeName is the binding reference to the PersistentVolume backing this claim.".into()),
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
