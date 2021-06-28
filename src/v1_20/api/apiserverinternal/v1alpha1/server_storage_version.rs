// Generated from definition io.k8s.api.apiserverinternal.v1alpha1.ServerStorageVersion

/// An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServerStorageVersion {
    /// The ID of the reporting API server.
    pub api_server_id: Option<String>,

    /// The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.
    pub decodable_versions: Vec<String>,

    /// The API server encodes the object to this version when persisting it in the backend (e.g., etcd).
    pub encoding_version: Option<String>,
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
                    decodable_versions: value_decodable_versions.unwrap_or_default(),
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
            usize::from(!self.decodable_versions.is_empty()) +
            self.encoding_version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_server_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiServerID", value)?;
        }
        if !self.decodable_versions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "decodableVersions", &self.decodable_versions)?;
        }
        if let Some(value) = &self.encoding_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "encodingVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ServerStorageVersion {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.",
          "properties": {
            "apiServerID": {
              "description": "The ID of the reporting API server.",
              "type": "string"
            },
            "decodableVersions": {
              "description": "The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "encodingVersion": {
              "description": "The API server encodes the object to this version when persisting it in the backend (e.g., etcd).",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
