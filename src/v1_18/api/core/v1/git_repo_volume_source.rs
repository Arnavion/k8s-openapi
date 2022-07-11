// Generated from definition io.k8s.api.core.v1.GitRepoVolumeSource

/// Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.
///
/// DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GitRepoVolumeSource {
    /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
    pub directory: Option<String>,

    /// Repository URL
    pub repository: String,

    /// Commit hash for the specified revision.
    pub revision: Option<String>,

}

#[cfg(feature = "dsl")]
impl GitRepoVolumeSource  {
    /// Set [`Self::directory`]
    pub  fn directory_set(&mut self, directory: impl Into<Option<String>>) -> &mut Self {
        self.directory = directory.into(); self
    }

    pub  fn directory(&mut self) -> &mut String {
        if self.directory.is_none() { self.directory = Some(Default::default()) }
        self.directory.as_mut().unwrap()
    }

    /// Modify [`Self::directory`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn directory_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.directory.is_none() { self.directory = Some(Default::default()) };
        func(self.directory.as_mut().unwrap()); self
    }


    /// Set [`Self::repository`]
    pub  fn repository_set(&mut self, repository: impl Into<String>) -> &mut Self {
        self.repository = repository.into(); self
    }

    pub  fn repository(&mut self) -> &mut String {
        &mut self.repository
    }

    /// Modify [`Self::repository`] with a `func`
    pub  fn repository_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.repository); self
    }


    /// Set [`Self::revision`]
    pub  fn revision_set(&mut self, revision: impl Into<Option<String>>) -> &mut Self {
        self.revision = revision.into(); self
    }

    pub  fn revision(&mut self) -> &mut String {
        if self.revision.is_none() { self.revision = Some(Default::default()) }
        self.revision.as_mut().unwrap()
    }

    /// Modify [`Self::revision`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn revision_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.revision.is_none() { self.revision = Some(Default::default()) };
        func(self.revision.as_mut().unwrap()); self
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GitRepoVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_directory: Option<String> = None;
                let mut value_repository: Option<String> = None;
                let mut value_revision: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.GitRepoVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.\n\nDEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "directory".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "repository".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Repository URL".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "revision".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Commit hash for the specified revision.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "repository".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
