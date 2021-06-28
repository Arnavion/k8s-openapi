// Generated from definition io.k8s.api.core.v1.EmptyDirVolumeSource

/// Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EmptyDirVolumeSource {
    /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    pub medium: Option<String>,

    /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
    pub size_limit: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl<'de> crate::serde::Deserialize<'de> for EmptyDirVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_medium,
            Key_size_limit,
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
                            "medium" => Field::Key_medium,
                            "sizeLimit" => Field::Key_size_limit,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EmptyDirVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EmptyDirVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_medium: Option<String> = None;
                let mut value_size_limit: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_medium => value_medium = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_size_limit => value_size_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EmptyDirVolumeSource {
                    medium: value_medium,
                    size_limit: value_size_limit,
                })
            }
        }

        deserializer.deserialize_struct(
            "EmptyDirVolumeSource",
            &[
                "medium",
                "sizeLimit",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EmptyDirVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EmptyDirVolumeSource",
            self.medium.as_ref().map_or(0, |_| 1) +
            self.size_limit.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.medium {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "medium", value)?;
        }
        if let Some(value) = &self.size_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sizeLimit", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for EmptyDirVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.",
          "properties": {
            "medium": {
              "description": "What type of storage medium should back this directory. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir",
              "type": "string"
            },
            "sizeLimit": crate::schema_ref_with_description(crate::apimachinery::pkg::api::resource::Quantity::schema(), "Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir")
          },
          "type": "object"
        })
    }
}
