// Generated from definition io.k8s.api.core.v1.GlusterfsVolumeSource

/// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlusterfsVolumeSource {
    /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    pub endpoints: String,

    /// Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    pub path: String,

    /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    pub read_only: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for GlusterfsVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_endpoints,
            Key_path,
            Key_read_only,
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
                            "endpoints" => Field::Key_endpoints,
                            "path" => Field::Key_path,
                            "readOnly" => Field::Key_read_only,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GlusterfsVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GlusterfsVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_endpoints: Option<String> = None;
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_endpoints => value_endpoints = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GlusterfsVolumeSource {
                    endpoints: value_endpoints.ok_or_else(|| serde::de::Error::missing_field("endpoints"))?,
                    path: value_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    read_only: value_read_only,
                })
            }
        }

        deserializer.deserialize_struct(
            "GlusterfsVolumeSource",
            &[
                "endpoints",
                "path",
                "readOnly",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for GlusterfsVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GlusterfsVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "endpoints", &self.endpoints)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
