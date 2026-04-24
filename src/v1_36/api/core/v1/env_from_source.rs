// Generated from definition io.k8s.api.core.v1.EnvFromSource

/// EnvFromSource represents the source of a set of ConfigMaps or Secrets
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvFromSource {
    /// The ConfigMap to select from
    pub config_map_ref: Option<crate::api::core::v1::ConfigMapEnvSource>,

    /// Optional text to prepend to the name of each environment variable. May consist of any printable ASCII characters except '='.
    pub prefix: Option<std::string::String>,

    /// The Secret to select from
    pub secret_ref: Option<crate::api::core::v1::SecretEnvSource>,
}

impl crate::DeepMerge for EnvFromSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.config_map_ref, other.config_map_ref);
        crate::DeepMerge::merge_from(&mut self.prefix, other.prefix);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EnvFromSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map_ref,
            Key_prefix,
            Key_secret_ref,
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
                            "configMapRef" => Field::Key_config_map_ref,
                            "prefix" => Field::Key_prefix,
                            "secretRef" => Field::Key_secret_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EnvFromSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EnvFromSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config_map_ref: Option<crate::api::core::v1::ConfigMapEnvSource> = None;
                let mut value_prefix: Option<std::string::String> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretEnvSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map_ref => value_config_map_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_prefix => value_prefix = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EnvFromSource {
                    config_map_ref: value_config_map_ref,
                    prefix: value_prefix,
                    secret_ref: value_secret_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "EnvFromSource",
            &[
                "configMapRef",
                "prefix",
                "secretRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EnvFromSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvFromSource",
            self.config_map_ref.as_ref().map_or(0, |_| 1) +
            self.prefix.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_map_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMapRef", value)?;
        }
        if let Some(value) = &self.prefix {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "prefix", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EnvFromSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.EnvFromSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EnvFromSource represents the source of a set of ConfigMaps or Secrets",
            "type": "object",
            "properties": {
                "configMapRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapEnvSource>();
                    schema_obj.ensure_object().insert("description".into(), "The ConfigMap to select from".into());
                    schema_obj
                }),
                "prefix": {
                    "description": "Optional text to prepend to the name of each environment variable. May consist of any printable ASCII characters except '='.",
                    "type": "string",
                },
                "secretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretEnvSource>();
                    schema_obj.ensure_object().insert("description".into(), "The Secret to select from".into());
                    schema_obj
                }),
            },
        })
    }
}
