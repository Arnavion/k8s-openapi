// Generated from definition io.k8s.api.resource.v1beta2.DeviceRequestAllocationResult

/// DeviceRequestAllocationResult contains the allocation result for one request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequestAllocationResult {
    /// AdminAccess indicates that this device was allocated for administrative access. See the corresponding request field for a definition of mode.
    ///
    /// This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
    pub admin_access: Option<bool>,

    /// BindingConditions contains a copy of the BindingConditions from the corresponding ResourceSlice at the time of allocation.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    pub binding_conditions: Option<std::vec::Vec<std::string::String>>,

    /// BindingFailureConditions contains a copy of the BindingFailureConditions from the corresponding ResourceSlice at the time of allocation.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    pub binding_failure_conditions: Option<std::vec::Vec<std::string::String>>,

    /// ConsumedCapacity tracks the amount of capacity consumed per device as part of the claim request. The consumed amount may differ from the requested amount: it is rounded up to the nearest valid value based on the device’s requestPolicy if applicable (i.e., may not be less than the requested amount).
    ///
    /// The total consumed capacity for each device must not exceed the DeviceCapacity's Value.
    ///
    /// This field is populated only for devices that allow multiple allocations. All capacity entries are included, even if the consumed amount is zero.
    pub consumed_capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    pub device: std::string::String,

    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters.
    pub driver: std::string::String,

    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: std::string::String,

    /// Request is the name of the request in the claim which caused this device to be allocated. If it references a subrequest in the firstAvailable list on a DeviceRequest, this field must include both the name of the main request and the subrequest using the format \<main request\>/\<subrequest\>.
    ///
    /// Multiple devices may have been allocated per request.
    pub request: std::string::String,

    /// ShareID uniquely identifies an individual allocation share of the device, used when the device supports multiple simultaneous allocations. It serves as an additional map key to differentiate concurrent shares of the same device.
    pub share_id: Option<std::string::String>,

    /// A copy of all tolerations specified in the request at the time when the device got allocated.
    ///
    /// The maximum number of tolerations is 16.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    pub tolerations: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceToleration>>,
}

impl crate::DeepMerge for DeviceRequestAllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.admin_access, other.admin_access);
        crate::merge_strategies::list::atomic(&mut self.binding_conditions, other.binding_conditions);
        crate::merge_strategies::list::atomic(&mut self.binding_failure_conditions, other.binding_failure_conditions);
        crate::merge_strategies::map::granular(&mut self.consumed_capacity, other.consumed_capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.device, other.device);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::DeepMerge::merge_from(&mut self.request, other.request);
        crate::DeepMerge::merge_from(&mut self.share_id, other.share_id);
        crate::merge_strategies::list::atomic(&mut self.tolerations, other.tolerations);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceRequestAllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admin_access,
            Key_binding_conditions,
            Key_binding_failure_conditions,
            Key_consumed_capacity,
            Key_device,
            Key_driver,
            Key_pool,
            Key_request,
            Key_share_id,
            Key_tolerations,
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
                            "bindingConditions" => Field::Key_binding_conditions,
                            "bindingFailureConditions" => Field::Key_binding_failure_conditions,
                            "consumedCapacity" => Field::Key_consumed_capacity,
                            "device" => Field::Key_device,
                            "driver" => Field::Key_driver,
                            "pool" => Field::Key_pool,
                            "request" => Field::Key_request,
                            "shareID" => Field::Key_share_id,
                            "tolerations" => Field::Key_tolerations,
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
                let mut value_binding_conditions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_binding_failure_conditions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_consumed_capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_device: Option<std::string::String> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_pool: Option<std::string::String> = None;
                let mut value_request: Option<std::string::String> = None;
                let mut value_share_id: Option<std::string::String> = None;
                let mut value_tolerations: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceToleration>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admin_access => value_admin_access = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_binding_conditions => value_binding_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_binding_failure_conditions => value_binding_failure_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_consumed_capacity => value_consumed_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device => value_device = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_id => value_share_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequestAllocationResult {
                    admin_access: value_admin_access,
                    binding_conditions: value_binding_conditions,
                    binding_failure_conditions: value_binding_failure_conditions,
                    consumed_capacity: value_consumed_capacity,
                    device: value_device.unwrap_or_default(),
                    driver: value_driver.unwrap_or_default(),
                    pool: value_pool.unwrap_or_default(),
                    request: value_request.unwrap_or_default(),
                    share_id: value_share_id,
                    tolerations: value_tolerations,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceRequestAllocationResult",
            &[
                "adminAccess",
                "bindingConditions",
                "bindingFailureConditions",
                "consumedCapacity",
                "device",
                "driver",
                "pool",
                "request",
                "shareID",
                "tolerations",
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
            self.admin_access.as_ref().map_or(0, |_| 1) +
            self.binding_conditions.as_ref().map_or(0, |_| 1) +
            self.binding_failure_conditions.as_ref().map_or(0, |_| 1) +
            self.consumed_capacity.as_ref().map_or(0, |_| 1) +
            self.share_id.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.admin_access {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "adminAccess", value)?;
        }
        if let Some(value) = &self.binding_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bindingConditions", value)?;
        }
        if let Some(value) = &self.binding_failure_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bindingFailureConditions", value)?;
        }
        if let Some(value) = &self.consumed_capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "consumedCapacity", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "device", &self.device)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        if let Some(value) = &self.share_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareID", value)?;
        }
        if let Some(value) = &self.tolerations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceRequestAllocationResult {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceRequestAllocationResult".into()
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
                "bindingConditions": {
                    "description": "BindingConditions contains a copy of the BindingConditions from the corresponding ResourceSlice at the time of allocation.\n\nThis is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "bindingFailureConditions": {
                    "description": "BindingFailureConditions contains a copy of the BindingFailureConditions from the corresponding ResourceSlice at the time of allocation.\n\nThis is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "consumedCapacity": {
                    "description": "ConsumedCapacity tracks the amount of capacity consumed per device as part of the claim request. The consumed amount may differ from the requested amount: it is rounded up to the nearest valid value based on the device’s requestPolicy if applicable (i.e., may not be less than the requested amount).\n\nThe total consumed capacity for each device must not exceed the DeviceCapacity's Value.\n\nThis field is populated only for devices that allow multiple allocations. All capacity entries are included, even if the consumed amount is zero.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
                "device": {
                    "description": "Device references one device instance via its name in the driver's resource pool. It must be a DNS label.",
                    "type": "string",
                },
                "driver": {
                    "description": "Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters.",
                    "type": "string",
                },
                "pool": {
                    "description": "This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).\n\nMust not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.",
                    "type": "string",
                },
                "request": {
                    "description": "Request is the name of the request in the claim which caused this device to be allocated. If it references a subrequest in the firstAvailable list on a DeviceRequest, this field must include both the name of the main request and the subrequest using the format <main request>/<subrequest>.\n\nMultiple devices may have been allocated per request.",
                    "type": "string",
                },
                "shareID": {
                    "description": "ShareID uniquely identifies an individual allocation share of the device, used when the device supports multiple simultaneous allocations. It serves as an additional map key to differentiate concurrent shares of the same device.",
                    "type": "string",
                },
                "tolerations": {
                    "description": "A copy of all tolerations specified in the request at the time when the device got allocated.\n\nThe maximum number of tolerations is 16.\n\nThis is an alpha field and requires enabling the DRADeviceTaints feature gate.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceToleration>()),
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
