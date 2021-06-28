// Generated from definition io.k8s.api.core.v1.ConfigMapNodeConfigSource

/// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapNodeConfigSource {
    /// KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
    pub kubelet_config_key: String,

    /// Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
    pub name: String,

    /// Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
    pub namespace: String,

    /// ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    pub resource_version: Option<String>,

    /// UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    pub uid: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ConfigMapNodeConfigSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_kubelet_config_key,
            Key_name,
            Key_namespace,
            Key_resource_version,
            Key_uid,
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
                            "kubeletConfigKey" => Field::Key_kubelet_config_key,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "resourceVersion" => Field::Key_resource_version,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigMapNodeConfigSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ConfigMapNodeConfigSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_kubelet_config_key: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_kubelet_config_key => value_kubelet_config_key = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_resource_version => value_resource_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapNodeConfigSource {
                    kubelet_config_key: value_kubelet_config_key.ok_or_else(|| crate::serde::de::Error::missing_field("kubeletConfigKey"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace.ok_or_else(|| crate::serde::de::Error::missing_field("namespace"))?,
                    resource_version: value_resource_version,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "ConfigMapNodeConfigSource",
            &[
                "kubeletConfigKey",
                "name",
                "namespace",
                "resourceVersion",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ConfigMapNodeConfigSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ConfigMapNodeConfigSource",
            3 +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kubeletConfigKey", &self.kubelet_config_key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        if let Some(value) = &self.resource_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ConfigMapNodeConfigSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node.",
          "properties": {
            "kubeletConfigKey": {
              "description": "KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.",
              "type": "string"
            },
            "name": {
              "description": "Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.",
              "type": "string"
            },
            "namespace": {
              "description": "Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.",
              "type": "string"
            },
            "resourceVersion": {
              "description": "ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.",
              "type": "string"
            },
            "uid": {
              "description": "UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.",
              "type": "string"
            }
          },
          "required": [
            "kubeletConfigKey",
            "name",
            "namespace"
          ],
          "type": "object"
        })
    }
}
