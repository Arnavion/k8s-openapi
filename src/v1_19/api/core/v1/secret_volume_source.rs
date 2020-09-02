// Generated from definition io.k8s.api.core.v1.SecretVolumeSource

/// Adapts a Secret into a volume.
///
/// The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretVolumeSource {
    /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    pub items: Option<Vec<crate::api::core::v1::KeyToPath>>,

    /// Specify whether the Secret or its keys must be defined
    pub optional: Option<bool>,

    /// Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    pub secret_name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for SecretVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_items,
            Key_optional,
            Key_secret_name,
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
                            "defaultMode" => Field::Key_default_mode,
                            "items" => Field::Key_items,
                            "optional" => Field::Key_optional,
                            "secretName" => Field::Key_secret_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecretVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecretVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_items: Option<Vec<crate::api::core::v1::KeyToPath>> = None;
                let mut value_optional: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretVolumeSource {
                    default_mode: value_default_mode,
                    items: value_items,
                    optional: value_optional,
                    secret_name: value_secret_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretVolumeSource",
            &[
                "defaultMode",
                "items",
                "optional",
                "secretName",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for SecretVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretVolumeSource",
            self.default_mode.as_ref().map_or(0, |_| 1) +
            self.items.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1) +
            self.secret_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_mode {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        if let Some(value) = &self.items {
            serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        if let Some(value) = &self.optional {
            serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        if let Some(value) = &self.secret_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
