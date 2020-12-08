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

impl<'de> serde::Deserialize<'de> for ServerStorageVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_server_id,
            Key_decodable_versions,
            Key_encoding_version,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServerStorageVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServerStorageVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_server_id: Option<String> = None;
                let mut value_decodable_versions: Option<Vec<String>> = None;
                let mut value_encoding_version: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_server_id => value_api_server_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_decodable_versions => value_decodable_versions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_encoding_version => value_encoding_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for ServerStorageVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServerStorageVersion",
            self.api_server_id.as_ref().map_or(0, |_| 1) +
            self.decodable_versions.as_ref().map_or(0, |_| 1) +
            self.encoding_version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_server_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiServerID", value)?;
        }
        if let Some(value) = &self.decodable_versions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "decodableVersions", value)?;
        }
        if let Some(value) = &self.encoding_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "encodingVersion", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
