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

#[cfg(feature = "dsl")]
impl GlusterfsVolumeSource  {
    /// Set [`Self::endpoints`]
    pub  fn endpoints_set(&mut self, endpoints: impl Into<String>) -> &mut Self {
        self.endpoints = endpoints.into(); self
    }

    pub  fn endpoints(&mut self) -> &mut String {
        &mut self.endpoints
    }

    /// Modify [`Self::endpoints`] with a `func`
    pub  fn endpoints_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.endpoints); self
    }


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


}


impl<'de> crate::serde::Deserialize<'de> for GlusterfsVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_endpoints,
            Key_path,
            Key_read_only,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = GlusterfsVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GlusterfsVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_endpoints: Option<String> = None;
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_endpoints => value_endpoints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GlusterfsVolumeSource {
                    endpoints: value_endpoints.unwrap_or_default(),
                    path: value_path.unwrap_or_default(),
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

impl crate::serde::Serialize for GlusterfsVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GlusterfsVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "endpoints", &self.endpoints)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GlusterfsVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.GlusterfsVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "endpoints".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "path".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod".to_owned()),
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
                                description: Some("ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "endpoints".to_owned(),
                    "path".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
