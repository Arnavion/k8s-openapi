// Generated from definition io.k8s.api.core.v1.ConfigMapNodeConfigSource

/// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration
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

#[cfg(feature = "dsl")]
impl ConfigMapNodeConfigSource  {
    /// Set [`Self::kubelet_config_key`]
    pub  fn kubelet_config_key_set(&mut self, kubelet_config_key: impl Into<String>) -> &mut Self {
        self.kubelet_config_key = kubelet_config_key.into(); self
    }

    pub  fn kubelet_config_key(&mut self) -> &mut String {
        &mut self.kubelet_config_key
    }

    /// Modify [`Self::kubelet_config_key`] with a `func`
    pub  fn kubelet_config_key_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.kubelet_config_key); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::namespace`]
    pub  fn namespace_set(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into(); self
    }

    pub  fn namespace(&mut self) -> &mut String {
        &mut self.namespace
    }

    /// Modify [`Self::namespace`] with a `func`
    pub  fn namespace_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.namespace); self
    }


    /// Set [`Self::resource_version`]
    pub  fn resource_version_set(&mut self, resource_version: impl Into<Option<String>>) -> &mut Self {
        self.resource_version = resource_version.into(); self
    }

    pub  fn resource_version(&mut self) -> &mut String {
        if self.resource_version.is_none() { self.resource_version = Some(Default::default()) }
        self.resource_version.as_mut().unwrap()
    }

    /// Modify [`Self::resource_version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resource_version.is_none() { self.resource_version = Some(Default::default()) };
        func(self.resource_version.as_mut().unwrap()); self
    }


    /// Set [`Self::uid`]
    pub  fn uid_set(&mut self, uid: impl Into<Option<String>>) -> &mut Self {
        self.uid = uid.into(); self
    }

    pub  fn uid(&mut self) -> &mut String {
        if self.uid.is_none() { self.uid = Some(Default::default()) }
        self.uid.as_mut().unwrap()
    }

    /// Modify [`Self::uid`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn uid_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.uid.is_none() { self.uid = Some(Default::default()) };
        func(self.uid.as_mut().unwrap()); self
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
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ConfigMapNodeConfigSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "kubeletConfigKey".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "kubeletConfigKey".to_owned(),
                    "name".to_owned(),
                    "namespace".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
