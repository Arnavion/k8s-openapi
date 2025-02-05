// Generated from definition io.k8s.apimachinery.pkg.version.Info

/// Info contains versioning information. how we'll want to distribute that information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub build_date: std::string::String,

    pub compiler: std::string::String,

    pub git_commit: std::string::String,

    pub git_tree_state: std::string::String,

    pub git_version: std::string::String,

    pub go_version: std::string::String,

    pub major: std::string::String,

    pub minor: std::string::String,

    pub platform: std::string::String,
}

impl crate::DeepMerge for Info {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.build_date, other.build_date);
        crate::DeepMerge::merge_from(&mut self.compiler, other.compiler);
        crate::DeepMerge::merge_from(&mut self.git_commit, other.git_commit);
        crate::DeepMerge::merge_from(&mut self.git_tree_state, other.git_tree_state);
        crate::DeepMerge::merge_from(&mut self.git_version, other.git_version);
        crate::DeepMerge::merge_from(&mut self.go_version, other.go_version);
        crate::DeepMerge::merge_from(&mut self.major, other.major);
        crate::DeepMerge::merge_from(&mut self.minor, other.minor);
        crate::DeepMerge::merge_from(&mut self.platform, other.platform);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Info {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_build_date,
            Key_compiler,
            Key_git_commit,
            Key_git_tree_state,
            Key_git_version,
            Key_go_version,
            Key_major,
            Key_minor,
            Key_platform,
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
                            "buildDate" => Field::Key_build_date,
                            "compiler" => Field::Key_compiler,
                            "gitCommit" => Field::Key_git_commit,
                            "gitTreeState" => Field::Key_git_tree_state,
                            "gitVersion" => Field::Key_git_version,
                            "goVersion" => Field::Key_go_version,
                            "major" => Field::Key_major,
                            "minor" => Field::Key_minor,
                            "platform" => Field::Key_platform,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Info;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Info")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_build_date: Option<std::string::String> = None;
                let mut value_compiler: Option<std::string::String> = None;
                let mut value_git_commit: Option<std::string::String> = None;
                let mut value_git_tree_state: Option<std::string::String> = None;
                let mut value_git_version: Option<std::string::String> = None;
                let mut value_go_version: Option<std::string::String> = None;
                let mut value_major: Option<std::string::String> = None;
                let mut value_minor: Option<std::string::String> = None;
                let mut value_platform: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_build_date => value_build_date = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_compiler => value_compiler = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git_commit => value_git_commit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git_tree_state => value_git_tree_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git_version => value_git_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_go_version => value_go_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_major => value_major = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_minor => value_minor = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_platform => value_platform = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Info {
                    build_date: value_build_date.unwrap_or_default(),
                    compiler: value_compiler.unwrap_or_default(),
                    git_commit: value_git_commit.unwrap_or_default(),
                    git_tree_state: value_git_tree_state.unwrap_or_default(),
                    git_version: value_git_version.unwrap_or_default(),
                    go_version: value_go_version.unwrap_or_default(),
                    major: value_major.unwrap_or_default(),
                    minor: value_minor.unwrap_or_default(),
                    platform: value_platform.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Info",
            &[
                "buildDate",
                "compiler",
                "gitCommit",
                "gitTreeState",
                "gitVersion",
                "goVersion",
                "major",
                "minor",
                "platform",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Info",
            9,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "buildDate", &self.build_date)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "compiler", &self.compiler)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gitCommit", &self.git_commit)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gitTreeState", &self.git_tree_state)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gitVersion", &self.git_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "goVersion", &self.go_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "major", &self.major)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minor", &self.minor)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "platform", &self.platform)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Info {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.version.Info".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Info contains versioning information. how we'll want to distribute that information.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "buildDate".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "compiler".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitCommit".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitTreeState".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "goVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "major".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minor".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "platform".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "buildDate".into(),
                    "compiler".into(),
                    "gitCommit".into(),
                    "gitTreeState".into(),
                    "gitVersion".into(),
                    "goVersion".into(),
                    "major".into(),
                    "minor".into(),
                    "platform".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
