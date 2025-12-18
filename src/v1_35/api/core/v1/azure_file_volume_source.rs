// Generated from definition io.k8s.api.core.v1.AzureFileVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureFileVolumeSource {
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretName is the  name of secret that contains Azure Storage Account Name and Key
    pub secret_name: std::string::String,

    /// shareName is the azure share Name
    pub share_name: std::string::String,
}

impl crate::DeepMerge for AzureFileVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_name, other.secret_name);
        crate::DeepMerge::merge_from(&mut self.share_name, other.share_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AzureFileVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_read_only,
            Key_secret_name,
            Key_share_name,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "readOnly" => Field::Key_read_only,
                            "secretName" => Field::Key_secret_name,
                            "shareName" => Field::Key_share_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AzureFileVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("AzureFileVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_name: Option<std::string::String> = None;
                let mut value_share_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_name => value_share_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureFileVolumeSource {
                    read_only: value_read_only,
                    secret_name: value_secret_name.unwrap_or_default(),
                    share_name: value_share_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "AzureFileVolumeSource",
            &[
                "readOnly",
                "secretName",
                "shareName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AzureFileVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureFileVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", &self.secret_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareName", &self.share_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AzureFileVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.AzureFileVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "AzureFile represents an Azure File Service mount on the host and bind mount to the pod.",
            "type": "object",
            "properties": {
                "readOnly": {
                    "description": "readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
                    "type": "boolean",
                },
                "secretName": {
                    "description": "secretName is the  name of secret that contains Azure Storage Account Name and Key",
                    "type": "string",
                },
                "shareName": {
                    "description": "shareName is the azure share Name",
                    "type": "string",
                },
            },
            "required": [
                "secretName",
                "shareName",
            ],
        })
    }
}
