// Generated from definition io.k8s.api.resource.v1beta1.DeviceRequestAllocationResult

/// DeviceRequestAllocationResult contains the allocation result for one request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequestAllocationResult {
    /// AdminAccess indicates that this device was allocated for administrative access. See the corresponding request field for a definition of mode.
    ///
    /// This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
    pub admin_access: Option<bool>,

    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    pub device: std::string::String,

    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    pub driver: std::string::String,

    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: std::string::String,

    /// Request is the name of the request in the claim which caused this device to be allocated. Multiple devices may have been allocated per request.
    pub request: std::string::String,
}

impl crate::DeepMerge for DeviceRequestAllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.admin_access, other.admin_access);
        crate::DeepMerge::merge_from(&mut self.device, other.device);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::DeepMerge::merge_from(&mut self.request, other.request);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceRequestAllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admin_access,
            Key_device,
            Key_driver,
            Key_pool,
            Key_request,
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
                            "adminAccess" => Field::Key_admin_access,
                            "device" => Field::Key_device,
                            "driver" => Field::Key_driver,
                            "pool" => Field::Key_pool,
                            "request" => Field::Key_request,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceRequestAllocationResult;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceRequestAllocationResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_admin_access: Option<bool> = None;
                let mut value_device: Option<std::string::String> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_pool: Option<std::string::String> = None;
                let mut value_request: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admin_access => value_admin_access = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device => value_device = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequestAllocationResult {
                    admin_access: value_admin_access,
                    device: value_device.unwrap_or_default(),
                    driver: value_driver.unwrap_or_default(),
                    pool: value_pool.unwrap_or_default(),
                    request: value_request.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceRequestAllocationResult",
            &[
                "adminAccess",
                "device",
                "driver",
                "pool",
                "request",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceRequestAllocationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceRequestAllocationResult",
            4 +
            self.admin_access.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.admin_access {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "adminAccess", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "device", &self.device)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceRequestAllocationResult {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.DeviceRequestAllocationResult".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceRequestAllocationResult contains the allocation result for one request.",
            "type": "object",
            "properties": {
                "adminAccess": {
                    "description": "AdminAccess indicates that this device was allocated for administrative access. See the corresponding request field for a definition of mode.\n\nThis is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.",
                    "type": "boolean",
                },
                "device": {
                    "description": "Device references one device instance via its name in the driver's resource pool. It must be a DNS label.",
                    "type": "string",
                },
                "driver": {
                    "description": "Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.",
                    "type": "string",
                },
                "pool": {
                    "description": "This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).\n\nMust not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.",
                    "type": "string",
                },
                "request": {
                    "description": "Request is the name of the request in the claim which caused this device to be allocated. Multiple devices may have been allocated per request.",
                    "type": "string",
                },
            },
            "required": [
                "device",
                "driver",
                "pool",
                "request",
            ],
        })
    }
}
