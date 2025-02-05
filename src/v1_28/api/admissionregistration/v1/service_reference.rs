// Generated from definition io.k8s.api.admissionregistration.v1.ServiceReference

/// ServiceReference holds a reference to Service.legacy.k8s.io
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceReference {
    /// `name` is the name of the service. Required
    pub name: std::string::String,

    /// `namespace` is the namespace of the service. Required
    pub namespace: std::string::String,

    /// `path` is an optional URL path which will be sent in any request to this service.
    pub path: Option<std::string::String>,

    /// If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
    pub port: Option<i32>,
}

impl crate::DeepMerge for ServiceReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.port, other.port);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServiceReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_namespace,
            Key_path,
            Key_port,
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
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "path" => Field::Key_path,
                            "port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ServiceReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespace: Option<std::string::String> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_port: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceReference {
                    name: value_name.unwrap_or_default(),
                    namespace: value_namespace.unwrap_or_default(),
                    path: value_path,
                    port: value_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceReference",
            &[
                "name",
                "namespace",
                "path",
                "port",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceReference",
            2 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceReference {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1.ServiceReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ServiceReference holds a reference to Service.legacy.k8s.io".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`name` is the name of the service. Required".into()),
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
                                description: Some("`namespace` is the namespace of the service. Required".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "path".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`path` is an optional URL path which will be sent in any request to this service.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "namespace".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
