// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.ServerStorageVersion

/// An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServerStorageVersion {
    /// The ID of the reporting API server.
    pub api_server_id: Option<String>,

    /// The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.
    pub decodable_versions: Option<Vec<String>>,

    /// The API server encodes the object to this version when persisting it in the backend (e.g., etcd).
    pub encoding_version: Option<String>,
}

impl crate::DeepMerge for ServerStorageVersion {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_server_id, other.api_server_id);
        crate::merge_strategies::list::set(&mut self.decodable_versions, other.decodable_versions);
        crate::DeepMerge::merge_from(&mut self.encoding_version, other.encoding_version);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServerStorageVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_server_id,
            Key_decodable_versions,
            Key_encoding_version,
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
                            "apiServerID" => Field::Key_api_server_id,
                            "decodableVersions" => Field::Key_decodable_versions,
                            "encodingVersion" => Field::Key_encoding_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServerStorageVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServerStorageVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_server_id: Option<String> = None;
                let mut value_decodable_versions: Option<Vec<String>> = None;
                let mut value_encoding_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_server_id => value_api_server_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_decodable_versions => value_decodable_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_encoding_version => value_encoding_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServerStorageVersion {
                    api_server_id: value_api_server_id,
                    decodable_versions: value_decodable_versions,
                    encoding_version: value_encoding_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServerStorageVersion",
            &[
                "apiServerID",
                "decodableVersions",
                "encodingVersion",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServerStorageVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServerStorageVersion",
            self.api_server_id.as_ref().map_or(0, |_| 1) +
            self.decodable_versions.as_ref().map_or(0, |_| 1) +
            self.encoding_version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_server_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiServerID", value)?;
        }
        if let Some(value) = &self.decodable_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "decodableVersions", value)?;
        }
        if let Some(value) = &self.encoding_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "encodingVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServerStorageVersion {
    fn schema_name() -> String {
        "io.k8s.api.apiserverinternal.v1alpha1.ServerStorageVersion".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiServerID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The ID of the reporting API server.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "decodableVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "encodingVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The API server encodes the object to this version when persisting it in the backend (e.g., etcd).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
