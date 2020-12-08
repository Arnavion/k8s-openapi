// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimSpec

/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<Vec<String>>,

    /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) * An existing custom resource that implements data population (Alpha) In order to use custom resource types that implement data population, the AnyVolumeDataSource feature gate must be enabled. If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source.
    pub data_source: Option<crate::api::core::v1::TypedLocalObjectReference>,

    /// Resources represents the minimum resources the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
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

impl<'de> serde::Deserialize<'de> for PersistentVolumeClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_data_source: Option<crate::api::core::v1::TypedLocalObjectReference> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_storage_class_name: Option<String> = None;
                let mut value_volume_mode: Option<String> = None;
                let mut value_volume_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data_source => value_data_source = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mode => value_volume_mode = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimSpec {
                    access_modes: value_access_modes,
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

impl serde::Serialize for PersistentVolumeClaimSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimSpec",
            self.access_modes.as_ref().map_or(0, |_| 1) +
            self.data_source.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.storage_class_name.as_ref().map_or(0, |_| 1) +
            self.volume_mode.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_modes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.data_source {
            serde::ser::SerializeStruct::serialize_field(&mut state, "dataSource", value)?;
        }
        if let Some(value) = &self.resources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.storage_class_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "storageClassName", value)?;
        }
        if let Some(value) = &self.volume_mode {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMode", value)?;
        }
        if let Some(value) = &self.volume_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
