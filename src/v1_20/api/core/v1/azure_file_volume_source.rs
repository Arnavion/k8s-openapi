// Generated from definition io.k8s.api.core.v1.AzureFileVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureFileVolumeSource {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// the name of secret that contains Azure Storage Account Name and Key
    pub secret_name: String,

    /// Share Name
    pub share_name: String,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AzureFileVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;
                let mut value_share_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_share_name => value_share_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureFileVolumeSource {
                    read_only: value_read_only,
                    secret_name: value_secret_name.ok_or_else(|| crate::serde::de::Error::missing_field("secretName"))?,
                    share_name: value_share_name.ok_or_else(|| crate::serde::de::Error::missing_field("shareName"))?,
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

#[cfg(feature = "schema")]
impl crate::Schema for AzureFileVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "AzureFile represents an Azure File Service mount on the host and bind mount to the pod.",
          "properties": {
            "readOnly": {
              "description": "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
              "type": "boolean"
            },
            "secretName": {
              "description": "the name of secret that contains Azure Storage Account Name and Key",
              "type": "string"
            },
            "shareName": {
              "description": "Share Name",
              "type": "string"
            }
          },
          "required": [
            "secretName",
            "shareName"
          ],
          "type": "object"
        })
    }
}
