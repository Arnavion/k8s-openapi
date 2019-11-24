// Generated from definition io.k8s.api.core.v1.EnvFromSource

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvFromSource {
    /// The ConfigMap to select from
    pub config_map_ref: Option<crate::api::core::v1::ConfigMapEnvSource>,

    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    pub prefix: Option<String>,

    /// The Secret to select from
    pub secret_ref: Option<crate::api::core::v1::SecretEnvSource>,
}

impl<'de> serde::Deserialize<'de> for EnvFromSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map_ref,
            Key_prefix,
            Key_secret_ref,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnvFromSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EnvFromSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_config_map_ref: Option<crate::api::core::v1::ConfigMapEnvSource> = None;
                let mut value_prefix: Option<String> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretEnvSource> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map_ref => value_config_map_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_prefix => value_prefix = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for EnvFromSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvFromSource",
            self.config_map_ref.as_ref().map_or(0, |_| 1) +
            self.prefix.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_map_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "configMapRef", value)?;
        }
        if let Some(value) = &self.prefix {
            serde::ser::SerializeStruct::serialize_field(&mut state, "prefix", value)?;
        }
        if let Some(value) = &self.secret_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
