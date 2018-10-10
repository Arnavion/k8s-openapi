// Generated from definition io.k8s.api.core.v1.RBDVolumeSource

/// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RBDVolumeSource {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
    pub fs_type: Option<String>,

    /// The rados image name. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub image: String,

    /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub keyring: Option<String>,

    /// A collection of Ceph monitors. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub monitors: Vec<String>,

    /// The rados pool name. Default is rbd. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub pool: Option<String>,

    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub read_only: Option<bool>,

    /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub secret_ref: Option<::v1_12::api::core::v1::LocalObjectReference>,

    /// The rados user name. Default is admin. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    pub user: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for RBDVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RBDVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct RBDVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_image: Option<String> = None;
                let mut value_keyring: Option<String> = None;
                let mut value_monitors: Option<Vec<String>> = None;
                let mut value_pool: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<::v1_12::api::core::v1::LocalObjectReference> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_keyring => value_keyring = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_monitors => value_monitors = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_pool => value_pool = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RBDVolumeSource {
                    fs_type: value_fs_type,
                    image: value_image.ok_or_else(|| ::serde::de::Error::missing_field("image"))?,
                    keyring: value_keyring,
                    monitors: value_monitors.ok_or_else(|| ::serde::de::Error::missing_field("monitors"))?,
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

impl ::serde::Serialize for RBDVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RBDVolumeSource",
            0 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            1 +
            self.keyring.as_ref().map_or(0, |_| 1) +
            1 +
            self.pool.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        if let Some(value) = &self.keyring {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "keyring", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "monitors", &self.monitors)?;
        if let Some(value) = &self.pool {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.user {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
