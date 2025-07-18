// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus

/// API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionStatus {
    /// If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.
    pub common_encoding_version: Option<std::string::String>,

    /// The latest available observations of the storageVersion's state.
    pub conditions: Option<std::vec::Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>>,

    /// The reported versions per API server instance.
    pub storage_versions: Option<std::vec::Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>>,
}

impl crate::DeepMerge for StorageVersionStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.common_encoding_version, other.common_encoding_version);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.storage_versions,
            other.storage_versions,
            &[|lhs, rhs| lhs.api_server_id == rhs.api_server_id],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageVersionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_common_encoding_version: Option<std::string::String> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>> = None;
                let mut value_storage_versions: Option<std::vec::Vec<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>> = None;

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
                    conditions: value_conditions,
                    storage_versions: value_storage_versions,
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
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.storage_versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.common_encoding_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "commonEncodingVersion", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.storage_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageVersions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageVersionStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.",
            "type": "object",
            "properties": {
                "commonEncodingVersion": {
                    "description": "If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.",
                    "type": "string",
                },
                "conditions": {
                    "description": "The latest available observations of the storageVersion's state.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::apiserverinternal::v1alpha1::StorageVersionCondition>()),
                },
                "storageVersions": {
                    "description": "The reported versions per API server instance.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::apiserverinternal::v1alpha1::ServerStorageVersion>()),
                },
            },
        })
    }
}
