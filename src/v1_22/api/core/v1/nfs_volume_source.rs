// Generated from definition io.k8s.api.core.v1.NFSVolumeSource

/// Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NFSVolumeSource {
    /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub path: String,

    /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub read_only: Option<bool>,

    /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub server: String,

}

#[cfg(feature = "dsl")]
impl NFSVolumeSource  {
    /// Set [`Self::path`]
    pub  fn path_set(&mut self, path: impl Into<String>) -> &mut Self {
        self.path = path.into(); self
    }

    pub  fn path(&mut self) -> &mut String {
        &mut self.path
    }

    /// Modify [`Self::path`] with a `func`
    pub  fn path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.path); self
    }


    /// Set [`Self::read_only`]
    pub  fn read_only_set(&mut self, read_only: impl Into<Option<bool>>) -> &mut Self {
        self.read_only = read_only.into(); self
    }

    pub  fn read_only(&mut self) -> &mut bool {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) }
        self.read_only.as_mut().unwrap()
    }

    /// Modify [`Self::read_only`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn read_only_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) };
        func(self.read_only.as_mut().unwrap()); self
    }


    /// Set [`Self::server`]
    pub  fn server_set(&mut self, server: impl Into<String>) -> &mut Self {
        self.server = server.into(); self
    }

    pub  fn server(&mut self) -> &mut String {
        &mut self.server
    }

    /// Modify [`Self::server`] with a `func`
    pub  fn server_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.server); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NFSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path,
            Key_read_only,
            Key_server,
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
                            "path" => Field::Key_path,
                            "readOnly" => Field::Key_read_only,
                            "server" => Field::Key_server,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NFSVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NFSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_server: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server => value_server = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NFSVolumeSource {
                    path: value_path.unwrap_or_default(),
                    read_only: value_read_only,
                    server: value_server.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NFSVolumeSource",
            &[
                "path",
                "readOnly",
                "server",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NFSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NFSVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "server", &self.server)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NFSVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NFSVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "path".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "server".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "path".to_owned(),
                    "server".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
