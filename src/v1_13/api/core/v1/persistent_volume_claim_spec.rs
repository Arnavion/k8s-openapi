// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimSpec

/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Vec<String>,

    /// This field requires the VolumeSnapshotDataSource alpha feature gate to be enabled and currently VolumeSnapshot is the only supported data source. If the provisioner can support VolumeSnapshot data source, it will create a new volume and data will be restored to the volume at the same time. If the provisioner does not support VolumeSnapshot data source, volume will not be created and the failure will be reported as an event. In the future, we plan to support more data source types and the behavior of the provisioner may change.
    pub data_source: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Resources represents the minimum resources the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// A label query over volumes to consider for binding.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    pub storage_class_name: Option<String>,

    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is a beta feature.
    pub volume_mode: Option<String>,

    /// VolumeName is the binding reference to the PersistentVolume backing this claim.
    pub volume_name: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_data_source,
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
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_storage_class_name: Option<String> = None;
                let mut value_volume_mode: Option<String> = None;
                let mut value_volume_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source => value_data_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mode => value_volume_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimSpec {
                    access_modes: value_access_modes.unwrap_or_default(),
                    data_source: value_data_source,
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
            usize::from(!self.access_modes.is_empty()) +
            self.data_source.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.storage_class_name.as_ref().map_or(0, |_| 1) +
            self.volume_mode.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if !self.access_modes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", &self.access_modes)?;
        }
        if let Some(value) = &self.data_source {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dataSource", value)?;
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

#[cfg(feature = "schema")]
impl crate::Schema for PersistentVolumeClaimSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes",
          "properties": {
            "accessModes": {
              "description": "AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "dataSource": crate::schema_ref_with_description(crate::api::core::v1::TypedLocalObjectReference::schema(), "This field requires the VolumeSnapshotDataSource alpha feature gate to be enabled and currently VolumeSnapshot is the only supported data source. If the provisioner can support VolumeSnapshot data source, it will create a new volume and data will be restored to the volume at the same time. If the provisioner does not support VolumeSnapshot data source, volume will not be created and the failure will be reported as an event. In the future, we plan to support more data source types and the behavior of the provisioner may change."),
            "resources": crate::schema_ref_with_description(crate::api::core::v1::ResourceRequirements::schema(), "Resources represents the minimum resources the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources"),
            "selector": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::LabelSelector::schema(), "A label query over volumes to consider for binding."),
            "storageClassName": {
              "description": "Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1",
              "type": "string"
            },
            "volumeMode": {
              "description": "volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is a beta feature.",
              "type": "string"
            },
            "volumeName": {
              "description": "VolumeName is the binding reference to the PersistentVolume backing this claim.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
