// Generated from definition io.k8s.api.core.v1.VolumeMount

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mount_path: std::string::String,

    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).
    pub mount_propagation: Option<std::string::String>,

    /// This must match the Name of a Volume.
    pub name: std::string::String,

    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    pub read_only: Option<bool>,

    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.
    ///
    /// If ReadOnly is false, this field has no meaning and must be unspecified.
    ///
    /// If ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.
    ///
    /// If this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).
    ///
    /// If this field is not specified, it is treated as an equivalent of Disabled.
    pub recursive_read_only: Option<std::string::String>,

    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    pub sub_path: Option<std::string::String>,

    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    pub sub_path_expr: Option<std::string::String>,
}

impl crate::DeepMerge for VolumeMount {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.mount_path, other.mount_path);
        crate::DeepMerge::merge_from(&mut self.mount_propagation, other.mount_propagation);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.recursive_read_only, other.recursive_read_only);
        crate::DeepMerge::merge_from(&mut self.sub_path, other.sub_path);
        crate::DeepMerge::merge_from(&mut self.sub_path_expr, other.sub_path_expr);
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
            Key_recursive_read_only,
            Key_sub_path,
            Key_sub_path_expr,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "mountPath" => Field::Key_mount_path,
                            "mountPropagation" => Field::Key_mount_propagation,
                            "name" => Field::Key_name,
                            "readOnly" => Field::Key_read_only,
                            "recursiveReadOnly" => Field::Key_recursive_read_only,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeMount")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_mount_path: Option<std::string::String> = None;
                let mut value_mount_propagation: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_recursive_read_only: Option<std::string::String> = None;
                let mut value_sub_path: Option<std::string::String> = None;
                let mut value_sub_path_expr: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mount_path => value_mount_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_propagation => value_mount_propagation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_recursive_read_only => value_recursive_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    recursive_read_only: value_recursive_read_only,
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
                "recursiveReadOnly",
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
            self.recursive_read_only.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.recursive_read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "recursiveReadOnly", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.VolumeMount".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("VolumeMount describes a mounting of a Volume within a container.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "mountPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path within the container at which the volume should be mounted.  Must not contain ':'.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "mountPropagation".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("This must match the Name of a Volume.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "recursiveReadOnly".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("RecursiveReadOnly specifies whether read-only mounts should be handled recursively.\n\nIf ReadOnly is false, this field has no meaning and must be unspecified.\n\nIf ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.\n\nIf this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).\n\nIf this field is not specified, it is treated as an equivalent of Disabled.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subPathExpr".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to \"\" (volume's root). SubPathExpr and SubPath are mutually exclusive.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "mountPath".into(),
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
