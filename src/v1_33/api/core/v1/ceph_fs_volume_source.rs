// Generated from definition io.k8s.api.core.v1.CephFSVolumeSource

/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CephFSVolumeSource {
    /// monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub monitors: std::vec::Vec<std::string::String>,

    /// path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    pub path: Option<std::string::String>,

    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub read_only: Option<bool>,

    /// secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_file: Option<std::string::String>,

    /// secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub user: Option<std::string::String>,
}

impl crate::DeepMerge for CephFSVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.monitors, other.monitors);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_file, other.secret_file);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CephFSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_monitors,
            Key_path,
            Key_read_only,
            Key_secret_file,
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
                            "monitors" => Field::Key_monitors,
                            "path" => Field::Key_path,
                            "readOnly" => Field::Key_read_only,
                            "secretFile" => Field::Key_secret_file,
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
            type Value = CephFSVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CephFSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_monitors: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_file: Option<std::string::String> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_user: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_monitors => value_monitors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_file => value_secret_file = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CephFSVolumeSource {
                    monitors: value_monitors.unwrap_or_default(),
                    path: value_path,
                    read_only: value_read_only,
                    secret_file: value_secret_file,
                    secret_ref: value_secret_ref,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "CephFSVolumeSource",
            &[
                "monitors",
                "path",
                "readOnly",
                "secretFile",
                "secretRef",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CephFSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CephFSVolumeSource",
            1 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_file.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "monitors", &self.monitors)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_file {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretFile", value)?;
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
impl crate::schemars::JsonSchema for CephFSVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.CephFSVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.",
            "type": "object",
            "properties": {
                "monitors": {
                    "description": "monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "path": {
                    "description": "path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /",
                    "type": "string",
                },
                "readOnly": {
                    "description": "readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it",
                    "type": "boolean",
                },
                "secretFile": {
                    "description": "secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it",
                    "type": "string",
                },
                "secretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".into());
                    schema_obj
                }),
                "user": {
                    "description": "user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it",
                    "type": "string",
                },
            },
            "required": [
                "monitors",
            ],
        })
    }
}
