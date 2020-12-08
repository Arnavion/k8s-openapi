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

impl<'de> serde::Deserialize<'de> for Info {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Info;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Info")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_build_date: Option<String> = None;
                let mut value_compiler: Option<String> = None;
                let mut value_git_commit: Option<String> = None;
                let mut value_git_tree_state: Option<String> = None;
                let mut value_git_version: Option<String> = None;
                let mut value_go_version: Option<String> = None;
                let mut value_major: Option<String> = None;
                let mut value_minor: Option<String> = None;
                let mut value_platform: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_build_date => value_build_date = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_compiler => value_compiler = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_git_commit => value_git_commit = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_git_tree_state => value_git_tree_state = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_git_version => value_git_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_go_version => value_go_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_major => value_major = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_minor => value_minor = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_platform => value_platform = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Info {
                    build_date: value_build_date.ok_or_else(|| serde::de::Error::missing_field("buildDate"))?,
                    compiler: value_compiler.ok_or_else(|| serde::de::Error::missing_field("compiler"))?,
                    git_commit: value_git_commit.ok_or_else(|| serde::de::Error::missing_field("gitCommit"))?,
                    git_tree_state: value_git_tree_state.ok_or_else(|| serde::de::Error::missing_field("gitTreeState"))?,
                    git_version: value_git_version.ok_or_else(|| serde::de::Error::missing_field("gitVersion"))?,
                    go_version: value_go_version.ok_or_else(|| serde::de::Error::missing_field("goVersion"))?,
                    major: value_major.ok_or_else(|| serde::de::Error::missing_field("major"))?,
                    minor: value_minor.ok_or_else(|| serde::de::Error::missing_field("minor"))?,
                    platform: value_platform.ok_or_else(|| serde::de::Error::missing_field("platform"))?,
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

impl serde::Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Info",
            9,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "buildDate", &self.build_date)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "compiler", &self.compiler)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "gitCommit", &self.git_commit)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "gitTreeState", &self.git_tree_state)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "gitVersion", &self.git_version)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "goVersion", &self.go_version)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "major", &self.major)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "minor", &self.minor)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "platform", &self.platform)?;
        serde::ser::SerializeStruct::end(state)
    }
}
