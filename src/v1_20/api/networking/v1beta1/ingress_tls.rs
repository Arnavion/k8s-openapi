// Generated from definition io.k8s.api.networking.v1beta1.IngressTLS

/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressTLS {
    /// Hosts are a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
    pub hosts: Option<Vec<String>>,

    /// SecretName is the name of the secret used to terminate TLS traffic on port 443. Field is left optional to allow TLS routing based on SNI hostname alone. If the SNI host in a listener conflicts with the "Host" header field used by an IngressRule, the SNI host is used for termination and value of the Host header is used for routing.
    pub secret_name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for IngressTLS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hosts,
            Key_secret_name,
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
                            "hosts" => Field::Key_hosts,
                            "secretName" => Field::Key_secret_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngressTLS;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressTLS")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_hosts: Option<Vec<String>> = None;
                let mut value_secret_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hosts => value_hosts = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressTLS {
                    hosts: value_hosts,
                    secret_name: value_secret_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressTLS",
            &[
                "hosts",
                "secretName",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IngressTLS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressTLS",
            self.hosts.as_ref().map_or(0, |_| 1) +
            self.secret_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hosts {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hosts", value)?;
        }
        if let Some(value) = &self.secret_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
