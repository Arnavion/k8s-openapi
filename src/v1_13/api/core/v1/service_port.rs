// Generated from definition io.k8s.api.core.v1.ServicePort

/// ServicePort contains information on service's port.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServicePort {
    /// The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the 'Name' field in EndpointPort objects. Optional if only one ServicePort is defined on this service.
    pub name: Option<String>,

    /// The port on each node on which this service is exposed when type=NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
    pub node_port: Option<i32>,

    /// The port that will be exposed by this service.
    pub port: i32,

    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
    pub protocol: Option<String>,

    /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
    pub target_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,
}

impl<'de> serde::Deserialize<'de> for ServicePort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_node_port,
            Key_port,
            Key_protocol,
            Key_target_port,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServicePort;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServicePort")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_node_port: Option<i32> = None;
                let mut value_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;
                let mut value_target_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_port => value_node_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_protocol => value_protocol = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_port => value_target_port = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServicePort {
                    name: value_name,
                    node_port: value_node_port,
                    port: value_port.ok_or_else(|| serde::de::Error::missing_field("port"))?,
                    protocol: value_protocol,
                    target_port: value_target_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServicePort",
            &[
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

impl serde::Serialize for ServicePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServicePort",
            1 +
            self.name.as_ref().map_or(0, |_| 1) +
            self.node_port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1) +
            self.target_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.node_port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodePort", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        if let Some(value) = &self.protocol {
            serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        if let Some(value) = &self.target_port {
            serde::ser::SerializeStruct::serialize_field(&mut state, "targetPort", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
