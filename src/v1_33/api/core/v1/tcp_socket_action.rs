// Generated from definition io.k8s.api.core.v1.TCPSocketAction

/// TCPSocketAction describes an action based on opening a socket
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TCPSocketAction {
    /// Optional: Host name to connect to, defaults to the pod IP.
    pub host: Option<std::string::String>,

    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: crate::apimachinery::pkg::util::intstr::IntOrString,
}

impl crate::DeepMerge for TCPSocketAction {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.host, other.host);
        crate::DeepMerge::merge_from(&mut self.port, other.port);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TCPSocketAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_host,
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
                            "host" => Field::Key_host,
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
            type Value = TCPSocketAction;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TCPSocketAction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_host: Option<std::string::String> = None;
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_host => value_host = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TCPSocketAction {
                    host: value_host,
                    port: value_port.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "TCPSocketAction",
            &[
                "host",
                "port",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TCPSocketAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TCPSocketAction",
            1 +
            self.host.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.host {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TCPSocketAction {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.TCPSocketAction".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("TCPSocketAction describes an action based on opening a socket".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "host".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Optional: Host name to connect to, defaults to the pod IP.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "port".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
