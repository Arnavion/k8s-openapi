// Generated from definition io.k8s.api.storage.v1alpha1.VolumeAttachmentStatus

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentStatus {
    /// The last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attach_error: Option<crate::api::storage::v1alpha1::VolumeError>,

    /// Indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attached: bool,

    /// Upon successful attach, this field is populated with any information returned by the attach operation that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attachment_metadata: std::collections::BTreeMap<String, String>,

    /// The last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.
    pub detach_error: Option<crate::api::storage::v1alpha1::VolumeError>,
}

impl<'de> crate::serde::Deserialize<'de> for VolumeAttachmentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attach_error,
            Key_attached,
            Key_attachment_metadata,
            Key_detach_error,
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
                            "attachError" => Field::Key_attach_error,
                            "attached" => Field::Key_attached,
                            "attachmentMetadata" => Field::Key_attachment_metadata,
                            "detachError" => Field::Key_detach_error,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeAttachmentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_attach_error: Option<crate::api::storage::v1alpha1::VolumeError> = None;
                let mut value_attached: Option<bool> = None;
                let mut value_attachment_metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_detach_error: Option<crate::api::storage::v1alpha1::VolumeError> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attach_error => value_attach_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attached => value_attached = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_attachment_metadata => value_attachment_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_detach_error => value_detach_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentStatus {
                    attach_error: value_attach_error,
                    attached: value_attached.ok_or_else(|| crate::serde::de::Error::missing_field("attached"))?,
                    attachment_metadata: value_attachment_metadata.unwrap_or_default(),
                    detach_error: value_detach_error,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentStatus",
            &[
                "attachError",
                "attached",
                "attachmentMetadata",
                "detachError",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeAttachmentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentStatus",
            1 +
            self.attach_error.as_ref().map_or(0, |_| 1) +
            usize::from(!self.attachment_metadata.is_empty()) +
            self.detach_error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.attach_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attachError", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attached", &self.attached)?;
        if !self.attachment_metadata.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attachmentMetadata", &self.attachment_metadata)?;
        }
        if let Some(value) = &self.detach_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "detachError", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for VolumeAttachmentStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "VolumeAttachmentStatus is the status of a VolumeAttachment request.",
          "properties": {
            "attachError": crate::schema_ref_with_description(crate::api::storage::v1alpha1::VolumeError::schema(), "The last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher."),
            "attached": {
              "description": "Indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.",
              "type": "boolean"
            },
            "attachmentMetadata": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Upon successful attach, this field is populated with any information returned by the attach operation that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.",
              "type": "object"
            },
            "detachError": crate::schema_ref_with_description(crate::api::storage::v1alpha1::VolumeError::schema(), "The last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.")
          },
          "required": [
            "attached"
          ],
          "type": "object"
        })
    }
}
