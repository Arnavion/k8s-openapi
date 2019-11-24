// Generated from definition io.k8s.api.core.v1.CephFSPersistentVolumeSource

/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CephFSPersistentVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub monitors: Vec<String>,

    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    pub path: Option<String>,

    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub read_only: Option<bool>,

    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_file: Option<String>,

    /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_ref: Option<crate::api::core::v1::SecretReference>,

    /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub user: Option<String>,
}

impl<'de> serde::Deserialize<'de> for CephFSPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CephFSPersistentVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CephFSPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_monitors: Option<Vec<String>> = None;
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_file: Option<String> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretReference> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_monitors => value_monitors = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_file => value_secret_file = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CephFSPersistentVolumeSource {
                    monitors: value_monitors.ok_or_else(|| serde::de::Error::missing_field("monitors"))?,
                    path: value_path,
                    read_only: value_read_only,
                    secret_file: value_secret_file,
                    secret_ref: value_secret_ref,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "CephFSPersistentVolumeSource",
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

impl serde::Serialize for CephFSPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CephFSPersistentVolumeSource",
            1 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_file.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "monitors", &self.monitors)?;
        if let Some(value) = &self.path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_file {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretFile", value)?;
        }
        if let Some(value) = &self.secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.user {
            serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
