// Generated from definition io.k8s.api.autoscaling.v1.CrossVersionObjectReference

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CrossVersionObjectReference {
    /// API version of the referent
    pub api_version: Option<String>,

    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    pub kind: String,

    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}

impl<'de> serde::Deserialize<'de> for CrossVersionObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossVersionObjectReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CrossVersionObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CrossVersionObjectReference {
                    api_version: value_api_version,
                    kind: value_kind.ok_or_else(|| serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CrossVersionObjectReference",
            &[
                "apiVersion",
                "kind",
                "name",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CrossVersionObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CrossVersionObjectReference",
            2 +
            self.api_version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::end(state)
    }
}
