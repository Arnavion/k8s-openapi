// Generated from definition io.k8s.api.core.v1.PodIP

/// IP address information for entries in the (plural) PodIPs field. Each entry includes:
///    IP: An IP address allocated to the pod. Routable at least within the cluster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodIP {
    /// ip is an IP address (IPv4 or IPv6) assigned to the pod
    pub ip: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for PodIP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip,
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
                            "ip" => Field::Key_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodIP;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodIP")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for PodIP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodIP",
            self.ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for PodIP {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "IP address information for entries in the (plural) PodIPs field. Each entry includes:\n   IP: An IP address allocated to the pod. Routable at least within the cluster.",
          "properties": {
            "ip": {
              "description": "ip is an IP address (IPv4 or IPv6) assigned to the pod",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
