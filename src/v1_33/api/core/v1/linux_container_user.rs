// Generated from definition io.k8s.api.core.v1.LinuxContainerUser

/// LinuxContainerUser represents user identity information in Linux containers
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinuxContainerUser {
    /// GID is the primary gid initially attached to the first process in the container
    pub gid: i64,

    /// SupplementalGroups are the supplemental groups initially attached to the first process in the container
    pub supplemental_groups: Option<std::vec::Vec<i64>>,

    /// UID is the primary uid initially attached to the first process in the container
    pub uid: i64,
}

impl crate::DeepMerge for LinuxContainerUser {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.gid, other.gid);
        crate::merge_strategies::list::atomic(&mut self.supplemental_groups, other.supplemental_groups);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LinuxContainerUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_gid,
            Key_supplemental_groups,
            Key_uid,
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
                            "gid" => Field::Key_gid,
                            "supplementalGroups" => Field::Key_supplemental_groups,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LinuxContainerUser;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LinuxContainerUser")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_gid: Option<i64> = None;
                let mut value_supplemental_groups: Option<std::vec::Vec<i64>> = None;
                let mut value_uid: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_gid => value_gid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_supplemental_groups => value_supplemental_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LinuxContainerUser {
                    gid: value_gid.unwrap_or_default(),
                    supplemental_groups: value_supplemental_groups,
                    uid: value_uid.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "LinuxContainerUser",
            &[
                "gid",
                "supplementalGroups",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LinuxContainerUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LinuxContainerUser",
            2 +
            self.supplemental_groups.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gid", &self.gid)?;
        if let Some(value) = &self.supplemental_groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LinuxContainerUser {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.LinuxContainerUser".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("LinuxContainerUser represents user identity information in Linux containers".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "gid".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("GID is the primary gid initially attached to the first process in the container".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "supplementalGroups".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("SupplementalGroups are the supplemental groups initially attached to the first process in the container".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".into()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID is the primary uid initially attached to the first process in the container".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "gid".into(),
                    "uid".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
