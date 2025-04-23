// Generated from definition io.k8s.api.core.v1.DownwardAPIVolumeFile

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DownwardAPIVolumeFile {
    /// Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
    pub field_ref: Option<crate::api::core::v1::ObjectFieldSelector>,

    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub mode: Option<i32>,

    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: std::string::String,

    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    pub resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector>,
}

impl crate::DeepMerge for DownwardAPIVolumeFile {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.field_ref, other.field_ref);
        crate::DeepMerge::merge_from(&mut self.mode, other.mode);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.resource_field_ref, other.resource_field_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DownwardAPIVolumeFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field_ref,
            Key_mode,
            Key_path,
            Key_resource_field_ref,
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
                            "fieldRef" => Field::Key_field_ref,
                            "mode" => Field::Key_mode,
                            "path" => Field::Key_path,
                            "resourceFieldRef" => Field::Key_resource_field_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DownwardAPIVolumeFile;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DownwardAPIVolumeFile")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_field_ref: Option<crate::api::core::v1::ObjectFieldSelector> = None;
                let mut value_mode: Option<i32> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field_ref => value_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mode => value_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_field_ref => value_resource_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DownwardAPIVolumeFile {
                    field_ref: value_field_ref,
                    mode: value_mode,
                    path: value_path.unwrap_or_default(),
                    resource_field_ref: value_resource_field_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "DownwardAPIVolumeFile",
            &[
                "fieldRef",
                "mode",
                "path",
                "resourceFieldRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DownwardAPIVolumeFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DownwardAPIVolumeFile",
            1 +
            self.field_ref.as_ref().map_or(0, |_| 1) +
            self.mode.as_ref().map_or(0, |_| 1) +
            self.resource_field_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldRef", value)?;
        }
        if let Some(value) = &self.mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mode", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        if let Some(value) = &self.resource_field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceFieldRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DownwardAPIVolumeFile {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.DownwardAPIVolumeFile".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DownwardAPIVolumeFile represents information to create the file containing the pod field".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fieldRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectFieldSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "mode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "path".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceFieldRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ResourceFieldSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "path".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
