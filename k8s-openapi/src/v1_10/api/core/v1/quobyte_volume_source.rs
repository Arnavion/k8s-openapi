// Generated from definition io.k8s.api.core.v1.QuobyteVolumeSource

/// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuobyteVolumeSource {
    /// Group to map volume access to Default is no group
    pub group: Option<String>,

    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
    pub read_only: Option<bool>,

    /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    pub registry: String,

    /// User to map volume access to Defaults to serivceaccount user
    pub user: Option<String>,

    /// Volume is a string that references an already created Quobyte volume by name.
    pub volume: String,
}

impl<'de> ::serde::Deserialize<'de> for QuobyteVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_read_only,
            Key_registry,
            Key_user,
            Key_volume,
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
                            "group" => Field::Key_group,
                            "readOnly" => Field::Key_read_only,
                            "registry" => Field::Key_registry,
                            "user" => Field::Key_user,
                            "volume" => Field::Key_volume,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QuobyteVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct QuobyteVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_group: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_registry: Option<String> = None;
                let mut value_user: Option<String> = None;
                let mut value_volume: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_registry => value_registry = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_user => value_user = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume => value_volume = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(QuobyteVolumeSource {
                    group: value_group,
                    read_only: value_read_only,
                    registry: value_registry.ok_or_else(|| ::serde::de::Error::missing_field("registry"))?,
                    user: value_user,
                    volume: value_volume.ok_or_else(|| ::serde::de::Error::missing_field("volume"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "QuobyteVolumeSource",
            &[
                "group",
                "readOnly",
                "registry",
                "user",
                "volume",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for QuobyteVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "QuobyteVolumeSource",
            0 +
            self.group.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            1 +
            self.user.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.group {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "registry", &self.registry)?;
        if let Some(value) = &self.user {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "volume", &self.volume)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
