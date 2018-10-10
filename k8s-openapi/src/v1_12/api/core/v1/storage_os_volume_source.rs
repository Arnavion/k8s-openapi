// Generated from definition io.k8s.api.core.v1.StorageOSVolumeSource

/// Represents a StorageOS persistent volume resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageOSVolumeSource {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    pub secret_ref: Option<::v1_12::api::core::v1::LocalObjectReference>,

    /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    pub volume_name: Option<String>,

    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    pub volume_namespace: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for StorageOSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_read_only,
            Key_secret_ref,
            Key_volume_name,
            Key_volume_namespace,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "fsType" => Field::Key_fs_type,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "volumeName" => Field::Key_volume_name,
                            "volumeNamespace" => Field::Key_volume_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageOSVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct StorageOSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<::v1_12::api::core::v1::LocalObjectReference> = None;
                let mut value_volume_name: Option<String> = None;
                let mut value_volume_namespace: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_namespace => value_volume_namespace = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageOSVolumeSource {
                    fs_type: value_fs_type,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    volume_name: value_volume_name,
                    volume_namespace: value_volume_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageOSVolumeSource",
            &[
                "fsType",
                "readOnly",
                "secretRef",
                "volumeName",
                "volumeNamespace",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for StorageOSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageOSVolumeSource",
            0 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1) +
            self.volume_namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.volume_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        if let Some(value) = &self.volume_namespace {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeNamespace", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
