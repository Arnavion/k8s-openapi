// Generated from definition io.k8s.api.core.v1.ProjectedVolumeSource

/// Represents a projected volume source
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectedVolumeSource {
    /// Mode bits to use on created files by default. Must be a value between 0 and 0777. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// list of volume projections
    pub sources: Vec<::v1_12::api::core::v1::VolumeProjection>,
}

impl<'de> ::serde::Deserialize<'de> for ProjectedVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_sources,
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
                            "sources" => Field::Key_sources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectedVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ProjectedVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_sources: Option<Vec<::v1_12::api::core::v1::VolumeProjection>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sources => value_sources = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ProjectedVolumeSource {
                    default_mode: value_default_mode,
                    sources: value_sources.ok_or_else(|| ::serde::de::Error::missing_field("sources"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ProjectedVolumeSource",
            &[
                "defaultMode",
                "sources",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ProjectedVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ProjectedVolumeSource",
            0 +
            self.default_mode.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.default_mode {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "sources", &self.sources)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
