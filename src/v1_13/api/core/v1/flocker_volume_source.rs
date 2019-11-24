// Generated from definition io.k8s.api.core.v1.FlockerVolumeSource

/// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlockerVolumeSource {
    /// Name of the dataset stored as metadata -\> name on the dataset for Flocker should be considered as deprecated
    pub dataset_name: Option<String>,

    /// UUID of the dataset. This is unique identifier of a Flocker dataset
    pub dataset_uuid: Option<String>,
}

impl<'de> serde::Deserialize<'de> for FlockerVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_dataset_name,
            Key_dataset_uuid,
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
                            "datasetName" => Field::Key_dataset_name,
                            "datasetUUID" => Field::Key_dataset_uuid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FlockerVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlockerVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_dataset_name: Option<String> = None;
                let mut value_dataset_uuid: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_dataset_name => value_dataset_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dataset_uuid => value_dataset_uuid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlockerVolumeSource {
                    dataset_name: value_dataset_name,
                    dataset_uuid: value_dataset_uuid,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlockerVolumeSource",
            &[
                "datasetName",
                "datasetUUID",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for FlockerVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlockerVolumeSource",
            self.dataset_name.as_ref().map_or(0, |_| 1) +
            self.dataset_uuid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.dataset_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "datasetName", value)?;
        }
        if let Some(value) = &self.dataset_uuid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "datasetUUID", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
