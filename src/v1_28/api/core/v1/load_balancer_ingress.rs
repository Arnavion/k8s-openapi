// Generated from definition io.k8s.api.core.v1.LoadBalancerIngress

/// LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerIngress {
    /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
    pub hostname: Option<String>,

    /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
    pub ip: Option<String>,

    /// Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
    pub ports: Option<Vec<crate::api::core::v1::PortStatus>>,
}

impl crate::DeepMerge for LoadBalancerIngress {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.hostname, other.hostname);
        crate::DeepMerge::merge_from(&mut self.ip, other.ip);
        crate::merge_strategies::list::atomic(&mut self.ports, other.ports);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LoadBalancerIngress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostname,
            Key_ip,
            Key_ports,
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
                            "hostname" => Field::Key_hostname,
                            "ip" => Field::Key_ip,
                            "ports" => Field::Key_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerIngress;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LoadBalancerIngress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hostname: Option<String> = None;
                let mut value_ip: Option<String> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::PortStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LoadBalancerIngress {
                    hostname: value_hostname,
                    ip: value_ip,
                    ports: value_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "LoadBalancerIngress",
            &[
                "hostname",
                "ip",
                "ports",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LoadBalancerIngress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LoadBalancerIngress",
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.ip.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LoadBalancerIngress {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.LoadBalancerIngress".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "hostname".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ip".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ports".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Ports is a list of records of service ports If used, every port defined in the service should have an entry in it".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PortStatus>()))),
                                ..Default::default()
                            })),
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
