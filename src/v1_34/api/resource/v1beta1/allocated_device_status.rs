// Generated from definition io.k8s.api.resource.v1beta1.AllocatedDeviceStatus

/// AllocatedDeviceStatus contains the status of an allocated device, if the driver chooses to report it. This may include driver-specific information.
///
/// The combination of Driver, Pool, Device, and ShareID must match the corresponding key in Status.Allocation.Devices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllocatedDeviceStatus {
    /// Conditions contains the latest observation of the device's state. If the device has been configured according to the class and claim config references, the `Ready` condition should be True.
    ///
    /// Must not contain more than 8 entries.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// Data contains arbitrary driver-specific data.
    ///
    /// The length of the raw data must be smaller or equal to 10 Ki.
    pub data: Option<crate::apimachinery::pkg::runtime::RawExtension>,

    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    pub device: std::string::String,

    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    pub driver: std::string::String,

    /// NetworkData contains network-related information specific to the device.
    pub network_data: Option<crate::api::resource::v1beta1::NetworkDeviceData>,

    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: std::string::String,

    /// ShareID uniquely identifies an individual allocation share of the device.
    pub share_id: Option<std::string::String>,
}

impl crate::DeepMerge for AllocatedDeviceStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.data, other.data);
        crate::DeepMerge::merge_from(&mut self.device, other.device);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.network_data, other.network_data);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::DeepMerge::merge_from(&mut self.share_id, other.share_id);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AllocatedDeviceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_data,
            Key_device,
            Key_driver,
            Key_network_data,
            Key_pool,
            Key_share_id,
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
                            "conditions" => Field::Key_conditions,
                            "data" => Field::Key_data,
                            "device" => Field::Key_device,
                            "driver" => Field::Key_driver,
                            "networkData" => Field::Key_network_data,
                            "pool" => Field::Key_pool,
                            "shareID" => Field::Key_share_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AllocatedDeviceStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("AllocatedDeviceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_data: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_device: Option<std::string::String> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_network_data: Option<crate::api::resource::v1beta1::NetworkDeviceData> = None;
                let mut value_pool: Option<std::string::String> = None;
                let mut value_share_id: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data => value_data = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device => value_device = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_network_data => value_network_data = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_id => value_share_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllocatedDeviceStatus {
                    conditions: value_conditions,
                    data: value_data,
                    device: value_device.unwrap_or_default(),
                    driver: value_driver.unwrap_or_default(),
                    network_data: value_network_data,
                    pool: value_pool.unwrap_or_default(),
                    share_id: value_share_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllocatedDeviceStatus",
            &[
                "conditions",
                "data",
                "device",
                "driver",
                "networkData",
                "pool",
                "shareID",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AllocatedDeviceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllocatedDeviceStatus",
            3 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.data.as_ref().map_or(0, |_| 1) +
            self.network_data.as_ref().map_or(0, |_| 1) +
            self.share_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.data {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "data", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "device", &self.device)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.network_data {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "networkData", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        if let Some(value) = &self.share_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareID", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AllocatedDeviceStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.AllocatedDeviceStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "AllocatedDeviceStatus contains the status of an allocated device, if the driver chooses to report it. This may include driver-specific information.\n\nThe combination of Driver, Pool, Device, and ShareID must match the corresponding key in Status.Allocation.Devices.",
            "type": "object",
            "properties": {
                "conditions": {
                    "description": "Conditions contains the latest observation of the device's state. If the device has been configured according to the class and claim config references, the `Ready` condition should be True.\n\nMust not contain more than 8 entries.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "data": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>();
                    schema_obj.ensure_object().insert("description".into(), "Data contains arbitrary driver-specific data.\n\nThe length of the raw data must be smaller or equal to 10 Ki.".into());
                    schema_obj
                }),
                "device": {
                    "description": "Device references one device instance via its name in the driver's resource pool. It must be a DNS label.",
                    "type": "string",
                },
                "driver": {
                    "description": "Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.",
                    "type": "string",
                },
                "networkData": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta1::NetworkDeviceData>();
                    schema_obj.ensure_object().insert("description".into(), "NetworkData contains network-related information specific to the device.".into());
                    schema_obj
                }),
                "pool": {
                    "description": "This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).\n\nMust not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.",
                    "type": "string",
                },
                "shareID": {
                    "description": "ShareID uniquely identifies an individual allocation share of the device.",
                    "type": "string",
                },
            },
            "required": [
                "device",
                "driver",
                "pool",
            ],
        })
    }
}
