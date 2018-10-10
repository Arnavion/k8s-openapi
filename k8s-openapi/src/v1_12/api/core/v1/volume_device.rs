// Generated from definition io.k8s.api.core.v1.VolumeDevice

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeDevice {
    /// devicePath is the path inside of the container that the device will be mapped to.
    pub device_path: String,

    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}

impl<'de> ::serde::Deserialize<'de> for VolumeDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_device_path,
            Key_name,
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
                            "devicePath" => Field::Key_device_path,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeDevice;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct VolumeDevice")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_device_path: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_device_path => value_device_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeDevice {
                    device_path: value_device_path.ok_or_else(|| ::serde::de::Error::missing_field("devicePath"))?,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeDevice",
            &[
                "devicePath",
                "name",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for VolumeDevice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeDevice",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "devicePath", &self.device_path)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
