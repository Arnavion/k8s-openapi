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

impl crate::DeepMerge for EnvVarSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.config_map_key_ref, other.config_map_key_ref);
        crate::DeepMerge::merge_from(&mut self.field_ref, other.field_ref);
        crate::DeepMerge::merge_from(&mut self.resource_field_ref, other.resource_field_ref);
        crate::DeepMerge::merge_from(&mut self.secret_key_ref, other.secret_key_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EnvVarSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map_key_ref,
            Key_field_ref,
            Key_resource_field_ref,
            Key_secret_key_ref,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EnvVarSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EnvVarSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config_map_key_ref: Option<crate::api::core::v1::ConfigMapKeySelector> = None;
                let mut value_field_ref: Option<crate::api::core::v1::ObjectFieldSelector> = None;
                let mut value_resource_field_ref: Option<crate::api::core::v1::ResourceFieldSelector> = None;
                let mut value_secret_key_ref: Option<crate::api::core::v1::SecretKeySelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map_key_ref => value_config_map_key_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_field_ref => value_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_field_ref => value_resource_field_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_key_ref => value_secret_key_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for EnvVarSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvVarSource",
            self.config_map_key_ref.as_ref().map_or(0, |_| 1) +
            self.field_ref.as_ref().map_or(0, |_| 1) +
            self.resource_field_ref.as_ref().map_or(0, |_| 1) +
            self.secret_key_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_map_key_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMapKeyRef", value)?;
        }
        if let Some(value) = &self.field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldRef", value)?;
        }
        if let Some(value) = &self.resource_field_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceFieldRef", value)?;
        }
        if let Some(value) = &self.secret_key_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretKeyRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EnvVarSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.EnvVarSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("EnvVarSource represents a source for the value of an EnvVar.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "configMapKeyRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapKeySelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects a key of a ConfigMap.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "fieldRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectFieldSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceFieldRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ResourceFieldSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "secretKeyRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretKeySelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects a key of a secret in the pod's namespace".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
