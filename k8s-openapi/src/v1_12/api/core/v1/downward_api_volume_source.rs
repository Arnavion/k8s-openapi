// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeSource

/// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIVolumeSource {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// Items is a list of downward API volume file
    pub items: Option<Vec<::v1_12::api::core::v1::DownwardAPIVolumeFile>>,
}

impl<'de> ::serde::Deserialize<'de> for DownwardAPIVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_items,
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
                            "defaultMode" => Field::Key_default_mode,
                            "items" => Field::Key_items,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DownwardAPIVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_items: Option<Vec<::v1_12::api::core::v1::DownwardAPIVolumeFile>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIVolumeSource {
                    default_mode: value_default_mode,
                    items: value_items,
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIVolumeSource",
            &[
                "defaultMode",
                "items",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for DownwardAPIVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIVolumeSource",
            0 +
            self.default_mode.as_ref().map_or(0, |_| 1) +
            self.items.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_mode {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        if let Some(value) = &self.items {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
