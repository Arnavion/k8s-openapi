// Generated from definition io.k8s.api.core.v1.SeccompProfile

/// SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
    pub localhost_profile: Option<String>,

    /// type indicates which kind of seccomp profile will be applied. Valid options are:
    ///
    /// Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for SeccompProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_localhost_profile,
            Key_type_,
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
                            "localhostProfile" => Field::Key_localhost_profile,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SeccompProfile;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SeccompProfile")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_localhost_profile: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_localhost_profile => value_localhost_profile = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SeccompProfile {
                    localhost_profile: value_localhost_profile,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "SeccompProfile",
            &[
                "localhostProfile",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SeccompProfile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SeccompProfile",
            1 +
            self.localhost_profile.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.localhost_profile {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "localhostProfile", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for SeccompProfile {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.",
          "properties": {
            "localhostProfile": {
              "description": "localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is \"Localhost\".",
              "type": "string"
            },
            "type": {
              "description": "type indicates which kind of seccomp profile will be applied. Valid options are:\n\nLocalhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.",
              "type": "string"
            }
          },
          "required": [
            "type"
          ],
          "type": "object"
        })
    }
}
