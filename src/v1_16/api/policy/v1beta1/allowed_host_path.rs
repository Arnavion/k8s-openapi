// Generated from definition io.k8s.api.policy.v1beta1.AllowedHostPath

/// AllowedHostPath defines the host volume conditions that will be enabled by a policy for pods to use. It requires the path prefix to be defined.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllowedHostPath {
    /// pathPrefix is the path prefix that the host volume must match. It does not support `*`. Trailing slashes are trimmed when validating the path prefix with a host path.
    ///
    /// Examples: `/foo` would allow `/foo`, `/foo/` and `/foo/bar` `/foo` would not allow `/food` or `/etc/foo`
    pub path_prefix: Option<String>,

    /// when set to true, will allow host volumes matching the pathPrefix only if all volume mounts are readOnly.
    pub read_only: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for AllowedHostPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path_prefix,
            Key_read_only,
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
                            "pathPrefix" => Field::Key_path_prefix,
                            "readOnly" => Field::Key_read_only,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AllowedHostPath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AllowedHostPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_path_prefix: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path_prefix => value_path_prefix = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllowedHostPath {
                    path_prefix: value_path_prefix,
                    read_only: value_read_only,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllowedHostPath",
            &[
                "pathPrefix",
                "readOnly",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AllowedHostPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllowedHostPath",
            self.path_prefix.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.path_prefix {
            serde::ser::SerializeStruct::serialize_field(&mut state, "pathPrefix", value)?;
        }
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
