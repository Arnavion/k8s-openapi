// Generated from definition io.k8s.api.core.v1.AzureFilePersistentVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureFilePersistentVolumeSource {
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretName is the name of secret that contains Azure Storage Account Name and Key
    pub secret_name: String,

    /// secretNamespace is the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod
    pub secret_namespace: Option<String>,

    /// shareName is the azure Share Name
    pub share_name: String,

}

#[cfg(feature = "dsl")]
impl AzureFilePersistentVolumeSource  {
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


    /// Set [`Self::secret_name`]
    pub  fn secret_name_set(&mut self, secret_name: impl Into<String>) -> &mut Self {
        self.secret_name = secret_name.into(); self
    }

    pub  fn secret_name(&mut self) -> &mut String {
        &mut self.secret_name
    }

    /// Modify [`Self::secret_name`] with a `func`
    pub  fn secret_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.secret_name); self
    }


    /// Set [`Self::secret_namespace`]
    pub  fn secret_namespace_set(&mut self, secret_namespace: impl Into<Option<String>>) -> &mut Self {
        self.secret_namespace = secret_namespace.into(); self
    }

    pub  fn secret_namespace(&mut self) -> &mut String {
        if self.secret_namespace.is_none() { self.secret_namespace = Some(Default::default()) }
        self.secret_namespace.as_mut().unwrap()
    }

    /// Modify [`Self::secret_namespace`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_namespace_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.secret_namespace.is_none() { self.secret_namespace = Some(Default::default()) };
        func(self.secret_namespace.as_mut().unwrap()); self
    }


    /// Set [`Self::share_name`]
    pub  fn share_name_set(&mut self, share_name: impl Into<String>) -> &mut Self {
        self.share_name = share_name.into(); self
    }

    pub  fn share_name(&mut self) -> &mut String {
        &mut self.share_name
    }

    /// Modify [`Self::share_name`] with a `func`
    pub  fn share_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.share_name); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for AzureFilePersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_read_only,
            Key_secret_name,
            Key_secret_namespace,
            Key_share_name,
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
                            "readOnly" => Field::Key_read_only,
                            "secretName" => Field::Key_secret_name,
                            "secretNamespace" => Field::Key_secret_namespace,
                            "shareName" => Field::Key_share_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AzureFilePersistentVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AzureFilePersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;
                let mut value_secret_namespace: Option<String> = None;
                let mut value_share_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_namespace => value_secret_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_name => value_share_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureFilePersistentVolumeSource {
                    read_only: value_read_only,
                    secret_name: value_secret_name.unwrap_or_default(),
                    secret_namespace: value_secret_namespace,
                    share_name: value_share_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "AzureFilePersistentVolumeSource",
            &[
                "readOnly",
                "secretName",
                "secretNamespace",
                "shareName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AzureFilePersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureFilePersistentVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", &self.secret_name)?;
        if let Some(value) = &self.secret_namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretNamespace", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareName", &self.share_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AzureFilePersistentVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.AzureFilePersistentVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AzureFile represents an Azure File Service mount on the host and bind mount to the pod.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretName is the name of secret that contains Azure Storage Account Name and Key".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretNamespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretNamespace is the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shareName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("shareName is the azure Share Name".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "secretName".to_owned(),
                    "shareName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
