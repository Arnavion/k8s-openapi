// Generated from definition io.k8s.api.networking.v1.NetworkPolicyPort

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkPolicyPort {
    /// If set, indicates that the range of ports from port to endPort, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port. This feature is in Beta state and is enabled by default. It can be disabled using the Feature Gate "NetworkPolicyEndPort".
    pub end_port: Option<i32>,

    /// The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    pub port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// The protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    pub protocol: Option<String>,

}

#[cfg(feature = "dsl")]
impl NetworkPolicyPort  {
    /// Set [`Self::end_port`]
    pub  fn end_port_set(&mut self, end_port: impl Into<Option<i32>>) -> &mut Self {
        self.end_port = end_port.into(); self
    }

    pub  fn end_port(&mut self) -> &mut i32 {
        if self.end_port.is_none() { self.end_port = Some(Default::default()) }
        self.end_port.as_mut().unwrap()
    }

    /// Modify [`Self::end_port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn end_port_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.end_port.is_none() { self.end_port = Some(Default::default()) };
        func(self.end_port.as_mut().unwrap()); self
    }


    /// Set [`Self::port`]
    pub  fn port_set(&mut self, port: impl Into<Option<crate::apimachinery::pkg::util::intstr::IntOrString>>) -> &mut Self {
        self.port = port.into(); self
    }

    pub  fn port(&mut self) -> &mut crate::apimachinery::pkg::util::intstr::IntOrString {
        if self.port.is_none() { self.port = Some(Default::default()) }
        self.port.as_mut().unwrap()
    }

    /// Modify [`Self::port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn port_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::util::intstr::IntOrString)) -> &mut Self {
        if self.port.is_none() { self.port = Some(Default::default()) };
        func(self.port.as_mut().unwrap()); self
    }


    /// Set [`Self::protocol`]
    pub  fn protocol_set(&mut self, protocol: impl Into<Option<String>>) -> &mut Self {
        self.protocol = protocol.into(); self
    }

    pub  fn protocol(&mut self) -> &mut String {
        if self.protocol.is_none() { self.protocol = Some(Default::default()) }
        self.protocol.as_mut().unwrap()
    }

    /// Modify [`Self::protocol`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn protocol_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.protocol.is_none() { self.protocol = Some(Default::default()) };
        func(self.protocol.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for NetworkPolicyPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_end_port,
            Key_port,
            Key_protocol,
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
                            "endPort" => Field::Key_end_port,
                            "port" => Field::Key_port,
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
            type Value = NetworkPolicyPort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NetworkPolicyPort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_end_port: Option<i32> = None;
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_end_port => value_end_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkPolicyPort {
                    end_port: value_end_port,
                    port: value_port,
                    protocol: value_protocol,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkPolicyPort",
            &[
                "endPort",
                "port",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NetworkPolicyPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkPolicyPort",
            self.end_port.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.end_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "endPort", value)?;
        }
        if let Some(value) = &self.port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NetworkPolicyPort {
    fn schema_name() -> String {
        "io.k8s.api.networking.v1.NetworkPolicyPort".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NetworkPolicyPort describes a port to allow traffic on".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "endPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If set, indicates that the range of ports from port to endPort, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port. This feature is in Beta state and is enabled by default. It can be disabled using the Feature Gate \"NetworkPolicyEndPort\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "protocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
