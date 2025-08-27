// Generated from definition io.k8s.api.autoscaling.v2.CrossVersionObjectReference

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CrossVersionObjectReference {
    /// apiVersion is the API version of the referent
    pub api_version: Option<std::string::String>,

    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: std::string::String,

    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: std::string::String,
}

impl crate::DeepMerge for CrossVersionObjectReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CrossVersionObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CrossVersionObjectReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CrossVersionObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<std::string::String> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CrossVersionObjectReference {
                    api_version: value_api_version,
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
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

impl crate::serde::Serialize for CrossVersionObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CrossVersionObjectReference",
            2 +
            self.api_version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CrossVersionObjectReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.autoscaling.v2.CrossVersionObjectReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CrossVersionObjectReference contains enough information to let you identify the referred resource.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "apiVersion is the API version of the referent",
                    "type": "string",
                },
                "kind": {
                    "description": "kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "name": {
                    "description": "name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names",
                    "type": "string",
                },
            },
            "required": [
                "kind",
                "name",
            ],
        })
    }
}
