// Generated from definition io.k8s.api.core.v1.GitRepoVolumeSource

/// Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.
///
/// DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GitRepoVolumeSource {
    /// directory is the target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
    pub directory: Option<std::string::String>,

    /// repository is the URL
    pub repository: std::string::String,

    /// revision is the commit hash for the specified revision.
    pub revision: Option<std::string::String>,
}

impl crate::DeepMerge for GitRepoVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.directory, other.directory);
        crate::DeepMerge::merge_from(&mut self.repository, other.repository);
        crate::DeepMerge::merge_from(&mut self.revision, other.revision);
    }
}

impl<'de> crate::serde::Deserialize<'de> for GitRepoVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_directory,
            Key_repository,
            Key_revision,
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
                            "directory" => Field::Key_directory,
                            "repository" => Field::Key_repository,
                            "revision" => Field::Key_revision,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = GitRepoVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("GitRepoVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_directory: Option<std::string::String> = None;
                let mut value_repository: Option<std::string::String> = None;
                let mut value_revision: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_directory => value_directory = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_repository => value_repository = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision => value_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GitRepoVolumeSource {
                    directory: value_directory,
                    repository: value_repository.unwrap_or_default(),
                    revision: value_revision,
                })
            }
        }

        deserializer.deserialize_struct(
            "GitRepoVolumeSource",
            &[
                "directory",
                "repository",
                "revision",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for GitRepoVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GitRepoVolumeSource",
            1 +
            self.directory.as_ref().map_or(0, |_| 1) +
            self.revision.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.directory {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "directory", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "repository", &self.repository)?;
        if let Some(value) = &self.revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revision", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GitRepoVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.GitRepoVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.\n\nDEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "directory".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("directory is the target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "repository".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("repository is the URL".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "revision".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("revision is the commit hash for the specified revision.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "repository".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
