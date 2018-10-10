// Generated from definition io.k8s.api.core.v1.LoadBalancerIngress

/// LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerIngress {
    /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
    pub hostname: Option<String>,

    /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
    pub ip: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerIngress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostname,
            Key_ip,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "hostname" => Field::Key_hostname,
                            "ip" => Field::Key_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerIngress;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct LoadBalancerIngress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_hostname: Option<String> = None;
                let mut value_ip: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostname => value_hostname = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LoadBalancerIngress {
                    hostname: value_hostname,
                    ip: value_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "LoadBalancerIngress",
            &[
                "hostname",
                "ip",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for LoadBalancerIngress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LoadBalancerIngress",
            0 +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostname {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.ip {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
