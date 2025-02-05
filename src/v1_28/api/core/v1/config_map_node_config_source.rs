// Generated from definition io.k8s.api.core.v1.ConfigMapNodeConfigSource

/// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigMapNodeConfigSource {
    /// KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
    pub kubelet_config_key: std::string::String,

    /// Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
    pub name: std::string::String,

    /// Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
    pub namespace: std::string::String,

    /// ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    pub resource_version: Option<std::string::String>,

    /// UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    pub uid: Option<std::string::String>,
}

impl crate::DeepMerge for ConfigMapNodeConfigSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.kubelet_config_key, other.kubelet_config_key);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        crate::DeepMerge::merge_from(&mut self.resource_version, other.resource_version);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ConfigMapNodeConfigSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_kubelet_config_key: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespace: Option<std::string::String> = None;
                let mut value_resource_version: Option<std::string::String> = None;
                let mut value_uid: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_kubelet_config_key => value_kubelet_config_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ConfigMapNodeConfigSource {
                    kubelet_config_key: value_kubelet_config_key.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    namespace: value_namespace.unwrap_or_default(),
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ConfigMapNodeConfigSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ConfigMapNodeConfigSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "kubeletConfigKey".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "kubeletConfigKey".into(),
                    "name".into(),
                    "namespace".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
