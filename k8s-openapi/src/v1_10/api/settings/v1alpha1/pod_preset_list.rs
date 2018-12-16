// Generated from definition io.k8s.api.settings.v1alpha1.PodPresetList

/// PodPresetList is a list of PodPreset objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodPresetList {
    /// Items is a list of schema objects.
    pub items: Vec<crate::v1_10::api::settings::v1alpha1::PodPreset>,

    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ListMeta>,
}

impl crate::Resource for PodPresetList {
    fn api_version() -> &'static str {
        "settings.k8s.io/v1alpha1"
    }

    fn group() -> &'static str {
        "settings.k8s.io"
    }

    fn kind() -> &'static str {
        "PodPresetList"
    }

    fn version() -> &'static str {
        "v1alpha1"
    }
}

impl<'de> serde::Deserialize<'de> for PodPresetList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_items,
            Key_metadata,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "items" => Field::Key_items,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodPresetList;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct PodPresetList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<crate::v1_10::api::settings::v1alpha1::PodPreset>> = None;
                let mut value_metadata: Option<crate::v1_10::apimachinery::pkg::apis::meta::v1::ListMeta> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_items => value_items = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodPresetList {
                    items: value_items.ok_or_else(|| serde::de::Error::missing_field("items"))?,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodPresetList",
            &[
                "apiVersion",
                "kind",
                "items",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodPresetList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodPresetList",
            0 +
            2 +
            1 +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
