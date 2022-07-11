// Generated from definition io.k8s.api.core.v1.ServicePort

/// ServicePort contains information on service's port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServicePort {
    /// The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol. This is a beta field that is guarded by the ServiceAppProtocol feature gate and enabled by default.
    pub app_protocol: Option<String>,

    /// The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.
    pub name: Option<String>,

    /// The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
    pub node_port: Option<i32>,

    /// The port that will be exposed by this service.
    pub port: i32,

    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
    pub protocol: Option<String>,

    /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
    pub target_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

}

#[cfg(feature = "dsl")]
impl ServicePort  {
    /// Set [`Self::app_protocol`]
    pub  fn app_protocol_set(&mut self, app_protocol: impl Into<Option<String>>) -> &mut Self {
        self.app_protocol = app_protocol.into(); self
    }

    pub  fn app_protocol(&mut self) -> &mut String {
        if self.app_protocol.is_none() { self.app_protocol = Some(Default::default()) }
        self.app_protocol.as_mut().unwrap()
    }

    /// Modify [`Self::app_protocol`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn app_protocol_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.app_protocol.is_none() { self.app_protocol = Some(Default::default()) };
        func(self.app_protocol.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<Option<String>>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        if self.name.is_none() { self.name = Some(Default::default()) }
        self.name.as_mut().unwrap()
    }

    /// Modify [`Self::name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.name.is_none() { self.name = Some(Default::default()) };
        func(self.name.as_mut().unwrap()); self
    }


    /// Set [`Self::node_port`]
    pub  fn node_port_set(&mut self, node_port: impl Into<Option<i32>>) -> &mut Self {
        self.node_port = node_port.into(); self
    }

    pub  fn node_port(&mut self) -> &mut i32 {
        if self.node_port.is_none() { self.node_port = Some(Default::default()) }
        self.node_port.as_mut().unwrap()
    }

    /// Modify [`Self::node_port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_port_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.node_port.is_none() { self.node_port = Some(Default::default()) };
        func(self.node_port.as_mut().unwrap()); self
    }


    /// Set [`Self::port`]
    pub  fn port_set(&mut self, port: impl Into<i32>) -> &mut Self {
        self.port = port.into(); self
    }

    pub  fn port(&mut self) -> &mut i32 {
        &mut self.port
    }

    /// Modify [`Self::port`] with a `func`
    pub  fn port_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.port); self
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


    /// Set [`Self::target_port`]
    pub  fn target_port_set(&mut self, target_port: impl Into<Option<crate::apimachinery::pkg::util::intstr::IntOrString>>) -> &mut Self {
        self.target_port = target_port.into(); self
    }

    pub  fn target_port(&mut self) -> &mut crate::apimachinery::pkg::util::intstr::IntOrString {
        if self.target_port.is_none() { self.target_port = Some(Default::default()) }
        self.target_port.as_mut().unwrap()
    }

    /// Modify [`Self::target_port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn target_port_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::util::intstr::IntOrString)) -> &mut Self {
        if self.target_port.is_none() { self.target_port = Some(Default::default()) };
        func(self.target_port.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ServicePort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_app_protocol,
            Key_name,
            Key_node_port,
            Key_port,
            Key_protocol,
            Key_target_port,
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
                            "appProtocol" => Field::Key_app_protocol,
                            "name" => Field::Key_name,
                            "nodePort" => Field::Key_node_port,
                            "port" => Field::Key_port,
                            "protocol" => Field::Key_protocol,
                            "targetPort" => Field::Key_target_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServicePort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServicePort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_app_protocol: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_node_port: Option<i32> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;
                let mut value_target_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_app_protocol => value_app_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_port => value_node_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_port => value_target_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServicePort {
                    app_protocol: value_app_protocol,
                    name: value_name,
                    node_port: value_node_port,
                    port: value_port.unwrap_or_default(),
                    protocol: value_protocol,
                    target_port: value_target_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServicePort",
            &[
                "appProtocol",
                "name",
                "nodePort",
                "port",
                "protocol",
                "targetPort",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServicePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServicePort",
            1 +
            self.app_protocol.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.node_port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1) +
            self.target_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.app_protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "appProtocol", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.node_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodePort", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        if let Some(value) = &self.protocol {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        if let Some(value) = &self.target_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetPort", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServicePort {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ServicePort".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ServicePort contains information on service's port.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "appProtocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol. This is a beta field that is guarded by the ServiceAppProtocol feature gate and enabled by default.".to_owned()),
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
                                description: Some("The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodePort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The port that will be exposed by this service.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The IP protocol for this port. Supports \"TCP\", \"UDP\", and \"SCTP\". Default is TCP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetPort".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "port".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
