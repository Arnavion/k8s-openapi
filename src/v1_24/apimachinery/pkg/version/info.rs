// Generated from definition io.k8s.apimachinery.pkg.version.Info

/// Info contains versioning information. how we'll want to distribute that information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub build_date: String,

    pub compiler: String,

    pub git_commit: String,

    pub git_tree_state: String,

    pub git_version: String,

    pub go_version: String,

    pub major: String,

    pub minor: String,

    pub platform: String,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Info")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_build_date: Option<String> = None;
                let mut value_compiler: Option<String> = None;
                let mut value_git_commit: Option<String> = None;
                let mut value_git_tree_state: Option<String> = None;
                let mut value_git_version: Option<String> = None;
                let mut value_go_version: Option<String> = None;
                let mut value_major: Option<String> = None;
                let mut value_minor: Option<String> = None;
                let mut value_platform: Option<String> = None;

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
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.version.Info".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Info contains versioning information. how we'll want to distribute that information.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "buildDate".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "compiler".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitCommit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitTreeState".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gitVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "goVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "major".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minor".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "platform".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "buildDate".to_owned(),
                    "compiler".to_owned(),
                    "gitCommit".to_owned(),
                    "gitTreeState".to_owned(),
                    "gitVersion".to_owned(),
                    "goVersion".to_owned(),
                    "major".to_owned(),
                    "minor".to_owned(),
                    "platform".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
