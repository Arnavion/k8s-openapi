// Generated from definition io.k8s.api.core.v1.CSIPersistentVolumeSource

/// Represents storage that is managed by an external CSI volume driver
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume. Required.
    pub driver: String,

    /// Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    pub volume_handle: String,
}

impl<'de> ::serde::Deserialize<'de> for CSIPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_read_only,
            Key_volume_handle,
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
                            "readOnly" => Field::Key_read_only,
                            "volumeHandle" => Field::Key_volume_handle,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CSIPersistentVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CSIPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_handle: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_handle => value_volume_handle = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIPersistentVolumeSource {
                    driver: value_driver.ok_or_else(|| ::serde::de::Error::missing_field("driver"))?,
                    read_only: value_read_only,
                    volume_handle: value_volume_handle.ok_or_else(|| ::serde::de::Error::missing_field("volumeHandle"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIPersistentVolumeSource",
            &[
                "driver",
                "readOnly",
                "volumeHandle",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CSIPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIPersistentVolumeSource",
            0 +
            1 +
            (if self.read_only.is_some() { 1 } else { 0 }) +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeHandle", &self.volume_handle)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
