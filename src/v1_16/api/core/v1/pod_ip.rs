// Generated from definition io.k8s.api.core.v1.PodIP

/// IP address information for entries in the (plural) PodIPs field. Each entry includes:
///    IP: An IP address allocated to the pod. Routable at least within the cluster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodIP {
    /// ip is an IP address (IPv4 or IPv6) assigned to the pod
    pub ip: Option<String>,
}

impl<'de> serde::Deserialize<'de> for PodIP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip,
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
                            "ip" => Field::Key_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodIP;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodIP")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_ip: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip => value_ip = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodIP {
                    ip: value_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodIP",
            &[
                "ip",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodIP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodIP",
            self.ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
