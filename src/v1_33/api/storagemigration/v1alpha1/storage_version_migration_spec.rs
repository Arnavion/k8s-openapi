// Generated from definition io.k8s.api.storagemigration.v1alpha1.StorageVersionMigrationSpec

/// Spec of the storage version migration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageVersionMigrationSpec {
    /// The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is "Running", users can use this token to check the progress of the migration.
    pub continue_token: Option<std::string::String>,

    /// The resource that is being migrated. The migrator sends requests to the endpoint serving the resource. Immutable.
    pub resource: crate::api::storagemigration::v1alpha1::GroupVersionResource,
}

impl crate::DeepMerge for StorageVersionMigrationSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.continue_token, other.continue_token);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StorageVersionMigrationSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_continue_token,
            Key_resource,
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
                            "continueToken" => Field::Key_continue_token,
                            "resource" => Field::Key_resource,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVersionMigrationSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StorageVersionMigrationSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_continue_token: Option<std::string::String> = None;
                let mut value_resource: Option<crate::api::storagemigration::v1alpha1::GroupVersionResource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_continue_token => value_continue_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageVersionMigrationSpec {
                    continue_token: value_continue_token,
                    resource: value_resource.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageVersionMigrationSpec",
            &[
                "continueToken",
                "resource",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageVersionMigrationSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageVersionMigrationSpec",
            1 +
            self.continue_token.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.continue_token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "continueToken", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageVersionMigrationSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storagemigration.v1alpha1.StorageVersionMigrationSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Spec of the storage version migration.",
            "type": "object",
            "properties": {
                "continueToken": {
                    "description": "The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is \"Running\", users can use this token to check the progress of the migration.",
                    "type": "string",
                },
                "resource": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::storagemigration::v1alpha1::GroupVersionResource>();
                    schema_obj.ensure_object().insert("description".into(), "The resource that is being migrated. The migrator sends requests to the endpoint serving the resource. Immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "resource",
            ],
        })
    }
}
