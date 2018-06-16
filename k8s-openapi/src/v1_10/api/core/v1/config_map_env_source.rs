// Generated from definition io.k8s.api.core.v1.ConfigMapEnvSource

/// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
///
/// The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapEnvSource {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Specify whether the ConfigMap must be defined
    pub optional: Option<bool>,
}

impl<'de> ::serde::Deserialize<'de> for ConfigMapEnvSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_optional,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "name" => Field::Key_name,
                            "optional" => Field::Key_optional,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigMapEnvSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ConfigMapEnvSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_optional: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapEnvSource {
                    name: value_name,
                    optional: value_optional,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapEnvSource",
            &[
                "name",
                "optional",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ConfigMapEnvSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapEnvSource",
            0 +
            self.name.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.optional {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
