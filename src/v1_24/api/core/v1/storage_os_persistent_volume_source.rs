// Generated from definition io.k8s.api.core.v1.StorageOSPersistentVolumeSource

/// Represents a StorageOS persistent volume resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageOSPersistentVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    pub secret_ref: Option<crate::api::core::v1::ObjectReference>,

    /// volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    pub volume_name: Option<String>,

    /// volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    pub volume_namespace: Option<String>,

}

#[cfg(feature = "dsl")]
impl StorageOSPersistentVolumeSource  {
    /// Set [`Self::fs_type`]
    pub  fn fs_type_set(&mut self, fs_type: impl Into<Option<String>>) -> &mut Self {
        self.fs_type = fs_type.into(); self
    }

    pub  fn fs_type(&mut self) -> &mut String {
        if self.fs_type.is_none() { self.fs_type = Some(Default::default()) }
        self.fs_type.as_mut().unwrap()
    }

    /// Modify [`Self::fs_type`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fs_type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.fs_type.is_none() { self.fs_type = Some(Default::default()) };
        func(self.fs_type.as_mut().unwrap()); self
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


    /// Set [`Self::secret_ref`]
    pub  fn secret_ref_set(&mut self, secret_ref: impl Into<Option<crate::api::core::v1::ObjectReference>>) -> &mut Self {
        self.secret_ref = secret_ref.into(); self
    }

    pub  fn secret_ref(&mut self) -> &mut crate::api::core::v1::ObjectReference {
        if self.secret_ref.is_none() { self.secret_ref = Some(Default::default()) }
        self.secret_ref.as_mut().unwrap()
    }

    /// Modify [`Self::secret_ref`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_ref_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ObjectReference)) -> &mut Self {
        if self.secret_ref.is_none() { self.secret_ref = Some(Default::default()) };
        func(self.secret_ref.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_name`]
    pub  fn volume_name_set(&mut self, volume_name: impl Into<Option<String>>) -> &mut Self {
        self.volume_name = volume_name.into(); self
    }

    pub  fn volume_name(&mut self) -> &mut String {
        if self.volume_name.is_none() { self.volume_name = Some(Default::default()) }
        self.volume_name.as_mut().unwrap()
    }

    /// Modify [`Self::volume_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volume_name.is_none() { self.volume_name = Some(Default::default()) };
        func(self.volume_name.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_namespace`]
    pub  fn volume_namespace_set(&mut self, volume_namespace: impl Into<Option<String>>) -> &mut Self {
        self.volume_namespace = volume_namespace.into(); self
    }

    pub  fn volume_namespace(&mut self) -> &mut String {
        if self.volume_namespace.is_none() { self.volume_namespace = Some(Default::default()) }
        self.volume_namespace.as_mut().unwrap()
    }

    /// Modify [`Self::volume_namespace`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_namespace_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volume_namespace.is_none() { self.volume_namespace = Some(Default::default()) };
        func(self.volume_namespace.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StorageOSPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_read_only,
            Key_secret_ref,
            Key_volume_name,
            Key_volume_namespace,
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
                            "fsType" => Field::Key_fs_type,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            "volumeName" => Field::Key_volume_name,
                            "volumeNamespace" => Field::Key_volume_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StorageOSPersistentVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StorageOSPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_volume_name: Option<String> = None;
                let mut value_volume_namespace: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_name => value_volume_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_namespace => value_volume_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageOSPersistentVolumeSource {
                    fs_type: value_fs_type,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                    volume_name: value_volume_name,
                    volume_namespace: value_volume_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageOSPersistentVolumeSource",
            &[
                "fsType",
                "readOnly",
                "secretRef",
                "volumeName",
                "volumeNamespace",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StorageOSPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageOSPersistentVolumeSource",
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.volume_name.as_ref().map_or(0, |_| 1) +
            self.volume_namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.volume_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeName", value)?;
        }
        if let Some(value) = &self.volume_namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeNamespace", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StorageOSPersistentVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.StorageOSPersistentVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a StorageOS persistent volume resource.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.".to_owned()),
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
                                description: Some("readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumeNamespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to \"default\" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.".to_owned()),
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
