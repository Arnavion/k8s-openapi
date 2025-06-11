// Generated from definition io.k8s.api.core.v1.RBDVolumeSource

/// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RBDVolumeSource {
    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
    pub fs_type: Option<std::string::String>,

    /// image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub image: std::string::String,

    /// keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub keyring: Option<std::string::String>,

    /// monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub monitors: std::vec::Vec<std::string::String>,

    /// pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub pool: Option<std::string::String>,

    /// readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub read_only: Option<bool>,

    /// secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub user: Option<std::string::String>,
}

impl crate::DeepMerge for RBDVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.image, other.image);
        crate::DeepMerge::merge_from(&mut self.keyring, other.keyring);
        crate::merge_strategies::list::atomic(&mut self.monitors, other.monitors);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
    }
}

impl<'de> crate::serde::Deserialize<'de> for RBDVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_image,
            Key_keyring,
            Key_monitors,
            Key_pool,
            Key_read_only,
            Key_secret_ref,
            Key_user,
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
                            "fsType" => Field::Key_fs_type,
                            "image" => Field::Key_image,
                            "keyring" => Field::Key_keyring,
                            "monitors" => Field::Key_monitors,
                            "pool" => Field::Key_pool,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RBDVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("RBDVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_image: Option<std::string::String> = None;
                let mut value_keyring: Option<std::string::String> = None;
                let mut value_monitors: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_pool: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_user: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_keyring => value_keyring = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_monitors => value_monitors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RBDVolumeSource {
                    fs_type: value_fs_type,
                    image: value_image.unwrap_or_default(),
                    keyring: value_keyring,
                    monitors: value_monitors.unwrap_or_default(),
                    pool: value_pool,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "RBDVolumeSource",
            &[
                "fsType",
                "image",
                "keyring",
                "monitors",
                "pool",
                "readOnly",
                "secretRef",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RBDVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RBDVolumeSource",
            2 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.keyring.as_ref().map_or(0, |_| 1) +
            self.pool.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        if let Some(value) = &self.keyring {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "keyring", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "monitors", &self.monitors)?;
        if let Some(value) = &self.pool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RBDVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.RBDVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.",
            "type": "object",
            "properties": {
                "fsType": {
                    "description": "fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd",
                    "type": "string",
                },
                "image": {
                    "description": "image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "string",
                },
                "keyring": {
                    "description": "keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "string",
                },
                "monitors": {
                    "description": "monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "pool": {
                    "description": "pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "string",
                },
                "readOnly": {
                    "description": "readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "boolean",
                },
                "secretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it".into());
                    schema_obj
                }),
                "user": {
                    "description": "user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it",
                    "type": "string",
                },
            },
            "required": [
                "image",
                "monitors",
            ],
        })
    }
}
