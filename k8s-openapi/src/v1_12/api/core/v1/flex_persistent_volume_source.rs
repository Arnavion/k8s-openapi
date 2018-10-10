// Generated from definition io.k8s.api.core.v1.FlexPersistentVolumeSource

/// FlexPersistentVolumeSource represents a generic persistent volume resource that is provisioned/attached using an exec based plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,

    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    pub fs_type: Option<String>,

    /// Optional: Extra command options if any.
    pub options: Option<::std::collections::BTreeMap<String, String>>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    pub secret_ref: Option<::v1_12::api::core::v1::SecretReference>,
}

impl<'de> ::serde::Deserialize<'de> for FlexPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_options,
            Key_read_only,
            Key_secret_ref,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlexPersistentVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct FlexPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_fs_type: Option<String> = None;
                let mut value_options: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<::v1_12::api::core::v1::SecretReference> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlexPersistentVolumeSource {
                    driver: value_driver.ok_or_else(|| ::serde::de::Error::missing_field("driver"))?,
                    fs_type: value_fs_type,
                    options: value_options,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlexPersistentVolumeSource",
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

impl ::serde::Serialize for FlexPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlexPersistentVolumeSource",
            0 +
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
