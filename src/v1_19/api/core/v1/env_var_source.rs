// Generated from definition io.k8s.api.core.v1.EnvVarSource

/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvVarSource {
    /// Selects a key of a ConfigMap.
    pub config_map_key_ref: Option<crate::api::core::v1::ConfigMapKeySelector>,

    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels\['\<KEY\>'\]`, `metadata.annotations\['\<KEY\>'\]`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    pub field_ref: Option<crate::api::core::v1::ObjectFieldSelector>,

    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    pub resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector>,

    /// Selects a key of a secret in the pod's namespace
    pub secret_key_ref: Option<crate::api::core::v1::SecretKeySelector>,
}

impl<'de> serde::Deserialize<'de> for EnvVarSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map_key_ref,
            Key_field_ref,
            Key_resource_field_ref,
            Key_secret_key_ref,
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
                            "configMapKeyRef" => Field::Key_config_map_key_ref,
                            "fieldRef" => Field::Key_field_ref,
                            "resourceFieldRef" => Field::Key_resource_field_ref,
                            "secretKeyRef" => Field::Key_secret_key_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnvVarSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EnvVarSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_config_map_key_ref: Option<crate::api::core::v1::ConfigMapKeySelector> = None;
                let mut value_field_ref: Option<crate::api::core::v1::ObjectFieldSelector> = None;
                let mut value_resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector> = None;
                let mut value_secret_key_ref: Option<crate::api::core::v1::SecretKeySelector> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map_key_ref => value_config_map_key_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_field_ref => value_field_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_field_ref => value_resource_field_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_key_ref => value_secret_key_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EnvVarSource {
                    config_map_key_ref: value_config_map_key_ref,
                    field_ref: value_field_ref,
                    resource_field_ref: value_resource_field_ref,
                    secret_key_ref: value_secret_key_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "EnvVarSource",
            &[
                "configMapKeyRef",
                "fieldRef",
                "resourceFieldRef",
                "secretKeyRef",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EnvVarSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvVarSource",
            self.config_map_key_ref.as_ref().map_or(0, |_| 1) +
            self.field_ref.as_ref().map_or(0, |_| 1) +
            self.resource_field_ref.as_ref().map_or(0, |_| 1) +
            self.secret_key_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_map_key_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "configMapKeyRef", value)?;
        }
        if let Some(value) = &self.field_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fieldRef", value)?;
        }
        if let Some(value) = &self.resource_field_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resourceFieldRef", value)?;
        }
        if let Some(value) = &self.secret_key_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretKeyRef", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
