// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus

/// API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct StorageVersionStatus {
    /// If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.
    pub common_encoding_version: Option<String>,

    /// The latest available observations of the storageVersion's state.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>::new"))]
    pub conditions: Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>,

    /// The reported versions per API server instance.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>::new"))]
    pub storage_versions: Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>,
}

impl<'de> crate::serde::Deserialize<'de> for StorageVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_common_encoding_version,
            Key_conditions,
            Key_storage_versions,
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
                            "commonEncodingVersion" => Field::Key_common_encoding_version,
                            "conditions" => Field::Key_conditions,
                            "storageVersions" => Field::Key_storage_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVersionStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StorageVersionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_common_encoding_version: Option<String> = None;
                let mut value_conditions: Option<Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>> = None;
                let mut value_storage_versions: Option<Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_common_encoding_version => value_common_encoding_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_versions => value_storage_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageVersionStatus {
                    common_encoding_version: value_common_encoding_version,
                    conditions: value_conditions.unwrap_or_default(),
                    storage_versions: value_storage_versions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageVersionStatus",
            &[
                "commonEncodingVersion",
                "conditions",
                "storageVersions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageVersionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageVersionStatus",
            self.common_encoding_version.as_ref().map_or(0, |_| 1) +
            usize::from(!self.conditions.is_empty()) +
            usize::from(!self.storage_versions.is_empty()),
        )?;
        if let Some(value) = &self.common_encoding_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "commonEncodingVersion", value)?;
        }
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        if !self.storage_versions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageVersions", &self.storage_versions)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
