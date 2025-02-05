// Generated from definition io.k8s.api.core.v1.ContainerPort

/// ContainerPort represents a network port in a single container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 \< x \< 65536.
    pub container_port: i32,

    /// What host IP to bind the external port to.
    pub host_ip: Option<std::string::String>,

    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 \< x \< 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    pub host_port: Option<i32>,

    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    pub name: Option<std::string::String>,

    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    pub protocol: Option<std::string::String>,
}

impl crate::DeepMerge for ContainerPort {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container_port, other.container_port);
        crate::DeepMerge::merge_from(&mut self.host_ip, other.host_ip);
        crate::DeepMerge::merge_from(&mut self.host_port, other.host_port);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.protocol, other.protocol);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_port,
            Key_host_ip,
            Key_host_port,
            Key_name,
            Key_protocol,
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
                            "containerPort" => Field::Key_container_port,
                            "hostIP" => Field::Key_host_ip,
                            "hostPort" => Field::Key_host_port,
                            "name" => Field::Key_name,
                            "protocol" => Field::Key_protocol,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerPort;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_port: Option<i32> = None;
                let mut value_host_ip: Option<std::string::String> = None;
                let mut value_host_port: Option<i32> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_protocol: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_port => value_container_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ip => value_host_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_port => value_host_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerPort {
                    container_port: value_container_port.unwrap_or_default(),
                    host_ip: value_host_ip,
                    host_port: value_host_port,
                    name: value_name,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerPort",
            &[
                "containerPort",
                "hostIP",
                "hostPort",
                "name",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerPort",
            1 +
            self.host_ip.as_ref().map_or(0, |_| 1) +
            self.host_port.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerPort", &self.container_port)?;
        if let Some(value) = &self.host_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostIP", value)?;
        }
        if let Some(value) = &self.host_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPort", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerPort {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ContainerPort".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerPort represents a network port in a single container.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerPort".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostIP".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("What host IP to bind the external port to.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostPort".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protocol".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "containerPort".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
