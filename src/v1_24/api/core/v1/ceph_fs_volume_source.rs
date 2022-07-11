// Generated from definition io.k8s.api.core.v1.CephFSVolumeSource

/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CephFSVolumeSource {
    /// monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub monitors: Vec<String>,

    /// path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    pub path: Option<String>,

    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub read_only: Option<bool>,

    /// secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_file: Option<String>,

    /// secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub user: Option<String>,

}

#[cfg(feature = "dsl")]
impl CephFSVolumeSource  {
    /// Set [`Self::monitors`]
    pub  fn monitors_set(&mut self, monitors: impl Into<Vec<String>>) -> &mut Self {
        self.monitors = monitors.into(); self
    }

    pub  fn monitors(&mut self) -> &mut Vec<String> {
        &mut self.monitors
    }

    /// Modify [`Self::monitors`] with a `func`
    pub  fn monitors_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.monitors); self
    }

    /// Push new element to [`Self::monitors`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn monitors_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.monitors.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::monitors`]
    pub  fn monitors_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.monitors.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::path`]
    pub  fn path_set(&mut self, path: impl Into<Option<String>>) -> &mut Self {
        self.path = path.into(); self
    }

    pub  fn path(&mut self) -> &mut String {
        if self.path.is_none() { self.path = Some(Default::default()) }
        self.path.as_mut().unwrap()
    }

    /// Modify [`Self::path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.path.is_none() { self.path = Some(Default::default()) };
        func(self.path.as_mut().unwrap()); self
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


    /// Set [`Self::secret_file`]
    pub  fn secret_file_set(&mut self, secret_file: impl Into<Option<String>>) -> &mut Self {
        self.secret_file = secret_file.into(); self
    }

    pub  fn secret_file(&mut self) -> &mut String {
        if self.secret_file.is_none() { self.secret_file = Some(Default::default()) }
        self.secret_file.as_mut().unwrap()
    }

    /// Modify [`Self::secret_file`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_file_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.secret_file.is_none() { self.secret_file = Some(Default::default()) };
        func(self.secret_file.as_mut().unwrap()); self
    }


    /// Set [`Self::secret_ref`]
    pub  fn secret_ref_set(&mut self, secret_ref: impl Into<Option<crate::api::core::v1::LocalObjectReference>>) -> &mut Self {
        self.secret_ref = secret_ref.into(); self
    }

    pub  fn secret_ref(&mut self) -> &mut crate::api::core::v1::LocalObjectReference {
        if self.secret_ref.is_none() { self.secret_ref = Some(Default::default()) }
        self.secret_ref.as_mut().unwrap()
    }

    /// Modify [`Self::secret_ref`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_ref_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LocalObjectReference)) -> &mut Self {
        if self.secret_ref.is_none() { self.secret_ref = Some(Default::default()) };
        func(self.secret_ref.as_mut().unwrap()); self
    }


    /// Set [`Self::user`]
    pub  fn user_set(&mut self, user: impl Into<Option<String>>) -> &mut Self {
        self.user = user.into(); self
    }

    pub  fn user(&mut self) -> &mut String {
        if self.user.is_none() { self.user = Some(Default::default()) }
        self.user.as_mut().unwrap()
    }

    /// Modify [`Self::user`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn user_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.user.is_none() { self.user = Some(Default::default()) };
        func(self.user.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for CephFSVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_monitors,
            Key_path,
            Key_read_only,
            Key_secret_file,
            Key_secret_ref,
            Key_user,
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
                            "monitors" => Field::Key_monitors,
                            "path" => Field::Key_path,
                            "readOnly" => Field::Key_read_only,
                            "secretFile" => Field::Key_secret_file,
                            "secretRef" => Field::Key_secret_ref,
                            "user" => Field::Key_user,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CephFSVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CephFSVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_monitors: Option<Vec<String>> = None;
                let mut value_path: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_file: Option<String> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_user: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_monitors => value_monitors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_file => value_secret_file = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CephFSVolumeSource {
                    monitors: value_monitors.unwrap_or_default(),
                    path: value_path,
                    read_only: value_read_only,
                    secret_file: value_secret_file,
                    secret_ref: value_secret_ref,
                    user: value_user,
                })
            }
        }

        deserializer.deserialize_struct(
            "CephFSVolumeSource",
            &[
                "monitors",
                "path",
                "readOnly",
                "secretFile",
                "secretRef",
                "user",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CephFSVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CephFSVolumeSource",
            1 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_file.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "monitors", &self.monitors)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_file {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretFile", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CephFSVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.CephFSVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "monitors".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".to_owned()),
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
                        "path".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /".to_owned()),
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
                                description: Some("readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretFile".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "user".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "monitors".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
