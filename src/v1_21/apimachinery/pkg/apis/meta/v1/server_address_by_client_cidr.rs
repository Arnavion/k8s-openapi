// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR

/// ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServerAddressByClientCIDR {
    /// The CIDR with which clients can match their IP to figure out the server address that they should use.
    pub client_cidr: String,

    /// Address of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.
    pub server_address: String,
}

impl<'de> crate::serde::Deserialize<'de> for ServerAddressByClientCIDR {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_cidr,
            Key_server_address,
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
                            "clientCIDR" => Field::Key_client_cidr,
                            "serverAddress" => Field::Key_server_address,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServerAddressByClientCIDR;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServerAddressByClientCIDR")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_client_cidr: Option<String> = None;
                let mut value_server_address: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_cidr => value_client_cidr = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_server_address => value_server_address = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServerAddressByClientCIDR {
                    client_cidr: value_client_cidr.ok_or_else(|| crate::serde::de::Error::missing_field("clientCIDR"))?,
                    server_address: value_server_address.ok_or_else(|| crate::serde::de::Error::missing_field("serverAddress"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServerAddressByClientCIDR",
            &[
                "clientCIDR",
                "serverAddress",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServerAddressByClientCIDR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServerAddressByClientCIDR",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientCIDR", &self.client_cidr)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serverAddress", &self.server_address)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl ServerAddressByClientCIDR {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.",
          "properties": {
            "clientCIDR": {
              "description": "The CIDR with which clients can match their IP to figure out the server address that they should use.",
              "type": "string"
            },
            "serverAddress": {
              "description": "Address of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.",
              "type": "string"
            }
          },
          "required": [
            "clientCIDR",
            "serverAddress"
          ],
          "type": "object"
        })
    }
}
