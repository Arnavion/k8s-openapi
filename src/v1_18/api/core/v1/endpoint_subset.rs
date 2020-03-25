// Generated from definition io.k8s.api.core.v1.EndpointSubset

/// EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:
///   {
///     Addresses: \[{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}\],
///     Ports:     \[{"name": "a", "port": 8675}, {"name": "b", "port": 309}\]
///   }
/// The resulting set of endpoints can be viewed as:
///     a: \[ 10.10.1.1:8675, 10.10.2.2:8675 \],
///     b: \[ 10.10.1.1:309, 10.10.2.2:309 \]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointSubset {
    /// IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
    pub addresses: Option<Vec<crate::api::core::v1::EndpointAddress>>,

    /// IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.
    pub not_ready_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>>,

    /// Port numbers available on the related IP addresses.
    pub ports: Option<Vec<crate::api::core::v1::EndpointPort>>,
}

impl<'de> serde::Deserialize<'de> for EndpointSubset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_addresses,
            Key_not_ready_addresses,
            Key_ports,
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
                            "addresses" => Field::Key_addresses,
                            "notReadyAddresses" => Field::Key_not_ready_addresses,
                            "ports" => Field::Key_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndpointSubset;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointSubset")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>> = None;
                let mut value_not_ready_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::EndpointPort>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not_ready_addresses => value_not_ready_addresses = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointSubset {
                    addresses: value_addresses,
                    not_ready_addresses: value_not_ready_addresses,
                    ports: value_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointSubset",
            &[
                "addresses",
                "notReadyAddresses",
                "ports",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EndpointSubset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointSubset",
            self.addresses.as_ref().map_or(0, |_| 1) +
            self.not_ready_addresses.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.addresses {
            serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", value)?;
        }
        if let Some(value) = &self.not_ready_addresses {
            serde::ser::SerializeStruct::serialize_field(&mut state, "notReadyAddresses", value)?;
        }
        if let Some(value) = &self.ports {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
