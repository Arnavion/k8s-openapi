// Generated from definition io.k8s.api.resource.v1alpha3.NetworkDeviceData

/// NetworkDeviceData provides network-related details for the allocated device. This information may be filled by drivers or other components to configure or identify the device within a network context.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkDeviceData {
    /// HardwareAddress represents the hardware address (e.g. MAC Address) of the device's network interface.
    ///
    /// Must not be longer than 128 characters.
    pub hardware_address: Option<std::string::String>,

    /// InterfaceName specifies the name of the network interface associated with the allocated device. This might be the name of a physical or virtual network interface being configured in the pod.
    ///
    /// Must not be longer than 256 characters.
    pub interface_name: Option<std::string::String>,

    /// IPs lists the network addresses assigned to the device's network interface. This can include both IPv4 and IPv6 addresses. The IPs are in the CIDR notation, which includes both the address and the associated subnet mask. e.g.: "192.0.2.5/24" for IPv4 and "2001:db8::5/64" for IPv6.
    pub ips: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for NetworkDeviceData {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.hardware_address, other.hardware_address);
        crate::DeepMerge::merge_from(&mut self.interface_name, other.interface_name);
        crate::merge_strategies::list::atomic(&mut self.ips, other.ips);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NetworkDeviceData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hardware_address,
            Key_interface_name,
            Key_ips,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "hardwareAddress" => Field::Key_hardware_address,
                            "interfaceName" => Field::Key_interface_name,
                            "ips" => Field::Key_ips,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkDeviceData;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NetworkDeviceData")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hardware_address: Option<std::string::String> = None;
                let mut value_interface_name: Option<std::string::String> = None;
                let mut value_ips: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hardware_address => value_hardware_address = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_interface_name => value_interface_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ips => value_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NetworkDeviceData {
                    hardware_address: value_hardware_address,
                    interface_name: value_interface_name,
                    ips: value_ips,
                })
            }
        }

        deserializer.deserialize_struct(
            "NetworkDeviceData",
            &[
                "hardwareAddress",
                "interfaceName",
                "ips",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NetworkDeviceData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NetworkDeviceData",
            self.hardware_address.as_ref().map_or(0, |_| 1) +
            self.interface_name.as_ref().map_or(0, |_| 1) +
            self.ips.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hardware_address {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hardwareAddress", value)?;
        }
        if let Some(value) = &self.interface_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "interfaceName", value)?;
        }
        if let Some(value) = &self.ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ips", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NetworkDeviceData {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.NetworkDeviceData".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NetworkDeviceData provides network-related details for the allocated device. This information may be filled by drivers or other components to configure or identify the device within a network context.",
            "type": "object",
            "properties": {
                "hardwareAddress": {
                    "description": "HardwareAddress represents the hardware address (e.g. MAC Address) of the device's network interface.\n\nMust not be longer than 128 characters.",
                    "type": "string",
                },
                "interfaceName": {
                    "description": "InterfaceName specifies the name of the network interface associated with the allocated device. This might be the name of a physical or virtual network interface being configured in the pod.\n\nMust not be longer than 256 characters.",
                    "type": "string",
                },
                "ips": {
                    "description": "IPs lists the network addresses assigned to the device's network interface. This can include both IPv4 and IPv6 addresses. The IPs are in the CIDR notation, which includes both the address and the associated subnet mask. e.g.: \"192.0.2.5/24\" for IPv4 and \"2001:db8::5/64\" for IPv6.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
