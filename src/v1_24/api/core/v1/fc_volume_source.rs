// Generated from definition io.k8s.api.core.v1.FCVolumeSource

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FCVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<String>,

    /// lun is Optional: FC target lun number
    pub lun: Option<i32>,

    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// targetWWNs is Optional: FC target worldwide names (WWNs)
    pub target_wwns: Option<Vec<String>>,

    /// wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    pub wwids: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl FCVolumeSource  {
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


    /// Set [`Self::lun`]
    pub  fn lun_set(&mut self, lun: impl Into<Option<i32>>) -> &mut Self {
        self.lun = lun.into(); self
    }

    pub  fn lun(&mut self) -> &mut i32 {
        if self.lun.is_none() { self.lun = Some(Default::default()) }
        self.lun.as_mut().unwrap()
    }

    /// Modify [`Self::lun`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn lun_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.lun.is_none() { self.lun = Some(Default::default()) };
        func(self.lun.as_mut().unwrap()); self
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


    /// Set [`Self::target_wwns`]
    pub  fn target_wwns_set(&mut self, target_wwns: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.target_wwns = target_wwns.into(); self
    }

    pub  fn target_wwns(&mut self) -> &mut Vec<String> {
        if self.target_wwns.is_none() { self.target_wwns = Some(Default::default()) }
        self.target_wwns.as_mut().unwrap()
    }

    /// Modify [`Self::target_wwns`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn target_wwns_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.target_wwns.is_none() { self.target_wwns = Some(Default::default()) };
        func(self.target_wwns.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::target_wwns`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn target_wwns_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.target_wwns.is_none() {
            self.target_wwns = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.target_wwns.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::target_wwns`]
    pub  fn target_wwns_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.target_wwns.is_none() { self.target_wwns = Some(Vec::new()); }
         let target_wwns = &mut self.target_wwns.as_mut().unwrap();
         for item in other.borrow() {
             target_wwns.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::wwids`]
    pub  fn wwids_set(&mut self, wwids: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.wwids = wwids.into(); self
    }

    pub  fn wwids(&mut self) -> &mut Vec<String> {
        if self.wwids.is_none() { self.wwids = Some(Default::default()) }
        self.wwids.as_mut().unwrap()
    }

    /// Modify [`Self::wwids`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn wwids_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.wwids.is_none() { self.wwids = Some(Default::default()) };
        func(self.wwids.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::wwids`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn wwids_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.wwids.is_none() {
            self.wwids = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.wwids.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::wwids`]
    pub  fn wwids_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.wwids.is_none() { self.wwids = Some(Vec::new()); }
         let wwids = &mut self.wwids.as_mut().unwrap();
         for item in other.borrow() {
             wwids.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for FCVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_lun,
            Key_read_only,
            Key_target_wwns,
            Key_wwids,
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
                            "lun" => Field::Key_lun,
                            "readOnly" => Field::Key_read_only,
                            "targetWWNs" => Field::Key_target_wwns,
                            "wwids" => Field::Key_wwids,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FCVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FCVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_target_wwns: Option<Vec<String>> = None;
                let mut value_wwids: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_wwns => value_target_wwns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_wwids => value_wwids = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FCVolumeSource {
                    fs_type: value_fs_type,
                    lun: value_lun,
                    read_only: value_read_only,
                    target_wwns: value_target_wwns,
                    wwids: value_wwids,
                })
            }
        }

        deserializer.deserialize_struct(
            "FCVolumeSource",
            &[
                "fsType",
                "lun",
                "readOnly",
                "targetWWNs",
                "wwids",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FCVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FCVolumeSource",
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.lun.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.target_wwns.as_ref().map_or(0, |_| 1) +
            self.wwids.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.lun {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.target_wwns {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetWWNs", value)?;
        }
        if let Some(value) = &self.wwids {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "wwids", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FCVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.FCVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.".to_owned()),
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
                        "lun".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("lun is Optional: FC target lun number".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetWWNs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetWWNs is Optional: FC target worldwide names (WWNs)".to_owned()),
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
                        "wwids".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.".to_owned()),
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
