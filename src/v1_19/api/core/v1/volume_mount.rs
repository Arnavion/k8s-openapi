// Generated from definition io.k8s.api.core.v1.VolumeMount

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mount_path: String,

    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    pub mount_propagation: Option<String>,

    /// This must match the Name of a Volume.
    pub name: String,

    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    pub read_only: Option<bool>,

    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    pub sub_path: Option<String>,

    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    pub sub_path_expr: Option<String>,

}

#[cfg(feature = "dsl")]
impl VolumeMount  {
    /// Set [`Self::mount_path`]
    pub  fn mount_path_set(&mut self, mount_path: impl Into<String>) -> &mut Self {
        self.mount_path = mount_path.into(); self
    }

    pub  fn mount_path(&mut self) -> &mut String {
        &mut self.mount_path
    }

    /// Modify [`Self::mount_path`] with a `func`
    pub  fn mount_path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.mount_path); self
    }


    /// Set [`Self::mount_propagation`]
    pub  fn mount_propagation_set(&mut self, mount_propagation: impl Into<Option<String>>) -> &mut Self {
        self.mount_propagation = mount_propagation.into(); self
    }

    pub  fn mount_propagation(&mut self) -> &mut String {
        if self.mount_propagation.is_none() { self.mount_propagation = Some(Default::default()) }
        self.mount_propagation.as_mut().unwrap()
    }

    /// Modify [`Self::mount_propagation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn mount_propagation_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.mount_propagation.is_none() { self.mount_propagation = Some(Default::default()) };
        func(self.mount_propagation.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
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


    /// Set [`Self::sub_path`]
    pub  fn sub_path_set(&mut self, sub_path: impl Into<Option<String>>) -> &mut Self {
        self.sub_path = sub_path.into(); self
    }

    pub  fn sub_path(&mut self) -> &mut String {
        if self.sub_path.is_none() { self.sub_path = Some(Default::default()) }
        self.sub_path.as_mut().unwrap()
    }

    /// Modify [`Self::sub_path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn sub_path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.sub_path.is_none() { self.sub_path = Some(Default::default()) };
        func(self.sub_path.as_mut().unwrap()); self
    }


    /// Set [`Self::sub_path_expr`]
    pub  fn sub_path_expr_set(&mut self, sub_path_expr: impl Into<Option<String>>) -> &mut Self {
        self.sub_path_expr = sub_path_expr.into(); self
    }

    pub  fn sub_path_expr(&mut self) -> &mut String {
        if self.sub_path_expr.is_none() { self.sub_path_expr = Some(Default::default()) }
        self.sub_path_expr.as_mut().unwrap()
    }

    /// Modify [`Self::sub_path_expr`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn sub_path_expr_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.sub_path_expr.is_none() { self.sub_path_expr = Some(Default::default()) };
        func(self.sub_path_expr.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for VolumeMount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mount_path,
            Key_mount_propagation,
            Key_name,
            Key_read_only,
            Key_sub_path,
            Key_sub_path_expr,
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
                            "mountPath" => Field::Key_mount_path,
                            "mountPropagation" => Field::Key_mount_propagation,
                            "name" => Field::Key_name,
                            "readOnly" => Field::Key_read_only,
                            "subPath" => Field::Key_sub_path,
                            "subPathExpr" => Field::Key_sub_path_expr,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeMount;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeMount")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_mount_path: Option<String> = None;
                let mut value_mount_propagation: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_sub_path: Option<String> = None;
                let mut value_sub_path_expr: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mount_path => value_mount_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_propagation => value_mount_propagation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sub_path => value_sub_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sub_path_expr => value_sub_path_expr = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeMount {
                    mount_path: value_mount_path.unwrap_or_default(),
                    mount_propagation: value_mount_propagation,
                    name: value_name.unwrap_or_default(),
                    read_only: value_read_only,
                    sub_path: value_sub_path,
                    sub_path_expr: value_sub_path_expr,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeMount",
            &[
                "mountPath",
                "mountPropagation",
                "name",
                "readOnly",
                "subPath",
                "subPathExpr",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeMount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeMount",
            2 +
            self.mount_propagation.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.sub_path.as_ref().map_or(0, |_| 1) +
            self.sub_path_expr.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mountPath", &self.mount_path)?;
        if let Some(value) = &self.mount_propagation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mountPropagation", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.sub_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subPath", value)?;
        }
        if let Some(value) = &self.sub_path_expr {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subPathExpr", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeMount {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.VolumeMount".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeMount describes a mounting of a Volume within a container.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "mountPath".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path within the container at which the volume should be mounted.  Must not contain ':'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "mountPropagation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This must match the Name of a Volume.".to_owned()),
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
                                description: Some("Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subPath".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subPathExpr".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to \"\" (volume's root). SubPathExpr and SubPath are mutually exclusive.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "mountPath".to_owned(),
                    "name".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
