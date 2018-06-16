// Generated from definition io.k8s.api.core.v1.NFSVolumeSource

/// Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NFSVolumeSource {
    /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub path: String,

    /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub read_only: Option<bool>,

    /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub server: String,
}

impl<'de> ::serde::Deserialize<'de> for NFSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path,
            Key_read_only,
            Key_server,
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
                            "path" => Field::Key_path,
                            "readOnly" => Field::Key_read_only,
                            "server" => Field::Key_server,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NFSVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NFSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_server: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path => value_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server => value_server = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NFSVolumeSource {
                    path: value_path.ok_or_else(|| ::serde::de::Error::missing_field("path"))?,
                    read_only: value_read_only,
                    server: value_server.ok_or_else(|| ::serde::de::Error::missing_field("server"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NFSVolumeSource",
            &[
                "path",
                "readOnly",
                "server",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NFSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NFSVolumeSource",
            0 +
            1 +
            self.read_only.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "server", &self.server)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
