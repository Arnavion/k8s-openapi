// Generated from definition io.k8s.api.core.v1.QuobyteVolumeSource

/// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuobyteVolumeSource {
    /// group to map volume access to Default is no group
    pub group: Option<std::string::String>,

    /// readOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
    pub read_only: Option<bool>,

    /// registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    pub registry: std::string::String,

    /// tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
    pub tenant: Option<std::string::String>,

    /// user to map volume access to Defaults to serivceaccount user
    pub user: Option<std::string::String>,

    /// volume is a string that references an already created Quobyte volume by name.
    pub volume: std::string::String,
}

impl crate::DeepMerge for QuobyteVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.registry, other.registry);
        crate::DeepMerge::merge_from(&mut self.tenant, other.tenant);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
        crate::DeepMerge::merge_from(&mut self.volume, other.volume);
    }
}

impl<'de> crate::serde::Deserialize<'de> for QuobyteVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_read_only,
            Key_registry,
            Key_tenant,
            Key_user,
            Key_volume,
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
                            "group" => Field::Key_group,
                            "readOnly" => Field::Key_read_only,
                            "registry" => Field::Key_registry,
                            "tenant" => Field::Key_tenant,
                            "user" => Field::Key_user,
                            "volume" => Field::Key_volume,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = QuobyteVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("QuobyteVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<std::string::String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_registry: Option<std::string::String> = None;
                let mut value_tenant: Option<std::string::String> = None;
                let mut value_user: Option<std::string::String> = None;
                let mut value_volume: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_registry => value_registry = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tenant => value_tenant = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume => value_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(QuobyteVolumeSource {
                    group: value_group,
                    read_only: value_read_only,
                    registry: value_registry.unwrap_or_default(),
                    tenant: value_tenant,
                    user: value_user,
                    volume: value_volume.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "QuobyteVolumeSource",
            &[
                "group",
                "readOnly",
                "registry",
                "tenant",
                "user",
                "volume",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for QuobyteVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "QuobyteVolumeSource",
            2 +
            self.group.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.tenant.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "registry", &self.registry)?;
        if let Some(value) = &self.tenant {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tenant", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volume", &self.volume)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for QuobyteVolumeSource {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.QuobyteVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "group".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("group to map volume access to Default is no group".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "readOnly".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "registry".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tenant".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "user".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("user to map volume access to Defaults to serivceaccount user".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volume".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("volume is a string that references an already created Quobyte volume by name.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "registry".into(),
                    "volume".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
