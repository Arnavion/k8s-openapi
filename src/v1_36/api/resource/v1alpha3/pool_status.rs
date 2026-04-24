// Generated from definition io.k8s.api.resource.v1alpha3.PoolStatus

/// PoolStatus contains status information for a single resource pool.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PoolStatus {
    /// AllocatedDevices is the number of devices currently allocated to claims. A value of 0 means no devices are allocated. May be unset when validationError is set.
    pub allocated_devices: Option<i32>,

    /// AvailableDevices is the number of devices available for allocation. This equals TotalDevices - AllocatedDevices - UnavailableDevices. A value of 0 means no devices are currently available. May be unset when validationError is set.
    pub available_devices: Option<i32>,

    /// Driver is the DRA driver name for this pool. Must be a DNS subdomain (e.g., "gpu.example.com").
    pub driver: std::string::String,

    /// Generation is the pool generation observed across all ResourceSlices in this pool. Only the latest generation is reported. During a generation rollout, if not all slices at the latest generation have been published, the pool is included with a validationError and device counts unset.
    pub generation: i64,

    /// NodeName is the node this pool is associated with. When omitted, the pool is not associated with a specific node. Must be a valid DNS subdomain name (RFC1123).
    pub node_name: Option<std::string::String>,

    /// PoolName is the name of the pool. Must be a valid resource pool name (DNS subdomains separated by "/").
    pub pool_name: std::string::String,

    /// ResourceSliceCount is the number of ResourceSlices that make up this pool. May be unset when validationError is set.
    pub resource_slice_count: Option<i32>,

    /// TotalDevices is the total number of devices in the pool across all slices. A value of 0 means the pool has no devices. May be unset when validationError is set.
    pub total_devices: Option<i32>,

    /// UnavailableDevices is the number of devices that are not available due to taints or other conditions, but are not allocated. A value of 0 means all unallocated devices are available. May be unset when validationError is set.
    pub unavailable_devices: Option<i32>,

    /// ValidationError is set when the pool's data could not be fully validated (e.g., incomplete slice publication). When set, device count fields and ResourceSliceCount may be unset.
    pub validation_error: Option<std::string::String>,
}

impl crate::DeepMerge for PoolStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocated_devices, other.allocated_devices);
        crate::DeepMerge::merge_from(&mut self.available_devices, other.available_devices);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.generation, other.generation);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.pool_name, other.pool_name);
        crate::DeepMerge::merge_from(&mut self.resource_slice_count, other.resource_slice_count);
        crate::DeepMerge::merge_from(&mut self.total_devices, other.total_devices);
        crate::DeepMerge::merge_from(&mut self.unavailable_devices, other.unavailable_devices);
        crate::DeepMerge::merge_from(&mut self.validation_error, other.validation_error);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PoolStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocated_devices,
            Key_available_devices,
            Key_driver,
            Key_generation,
            Key_node_name,
            Key_pool_name,
            Key_resource_slice_count,
            Key_total_devices,
            Key_unavailable_devices,
            Key_validation_error,
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
                            "allocatedDevices" => Field::Key_allocated_devices,
                            "availableDevices" => Field::Key_available_devices,
                            "driver" => Field::Key_driver,
                            "generation" => Field::Key_generation,
                            "nodeName" => Field::Key_node_name,
                            "poolName" => Field::Key_pool_name,
                            "resourceSliceCount" => Field::Key_resource_slice_count,
                            "totalDevices" => Field::Key_total_devices,
                            "unavailableDevices" => Field::Key_unavailable_devices,
                            "validationError" => Field::Key_validation_error,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PoolStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PoolStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocated_devices: Option<i32> = None;
                let mut value_available_devices: Option<i32> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_generation: Option<i64> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_pool_name: Option<std::string::String> = None;
                let mut value_resource_slice_count: Option<i32> = None;
                let mut value_total_devices: Option<i32> = None;
                let mut value_unavailable_devices: Option<i32> = None;
                let mut value_validation_error: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocated_devices => value_allocated_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_available_devices => value_available_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generation => value_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool_name => value_pool_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_slice_count => value_resource_slice_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_total_devices => value_total_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unavailable_devices => value_unavailable_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validation_error => value_validation_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PoolStatus {
                    allocated_devices: value_allocated_devices,
                    available_devices: value_available_devices,
                    driver: value_driver.unwrap_or_default(),
                    generation: value_generation.unwrap_or_default(),
                    node_name: value_node_name,
                    pool_name: value_pool_name.unwrap_or_default(),
                    resource_slice_count: value_resource_slice_count,
                    total_devices: value_total_devices,
                    unavailable_devices: value_unavailable_devices,
                    validation_error: value_validation_error,
                })
            }
        }

        deserializer.deserialize_struct(
            "PoolStatus",
            &[
                "allocatedDevices",
                "availableDevices",
                "driver",
                "generation",
                "nodeName",
                "poolName",
                "resourceSliceCount",
                "totalDevices",
                "unavailableDevices",
                "validationError",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PoolStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PoolStatus",
            3 +
            self.allocated_devices.as_ref().map_or(0, |_| 1) +
            self.available_devices.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.resource_slice_count.as_ref().map_or(0, |_| 1) +
            self.total_devices.as_ref().map_or(0, |_| 1) +
            self.unavailable_devices.as_ref().map_or(0, |_| 1) +
            self.validation_error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocated_devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatedDevices", value)?;
        }
        if let Some(value) = &self.available_devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableDevices", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generation", &self.generation)?;
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "poolName", &self.pool_name)?;
        if let Some(value) = &self.resource_slice_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceSliceCount", value)?;
        }
        if let Some(value) = &self.total_devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "totalDevices", value)?;
        }
        if let Some(value) = &self.unavailable_devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unavailableDevices", value)?;
        }
        if let Some(value) = &self.validation_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validationError", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PoolStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.PoolStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PoolStatus contains status information for a single resource pool.",
            "type": "object",
            "properties": {
                "allocatedDevices": {
                    "description": "AllocatedDevices is the number of devices currently allocated to claims. A value of 0 means no devices are allocated. May be unset when validationError is set.",
                    "type": "integer",
                    "format": "int32",
                },
                "availableDevices": {
                    "description": "AvailableDevices is the number of devices available for allocation. This equals TotalDevices - AllocatedDevices - UnavailableDevices. A value of 0 means no devices are currently available. May be unset when validationError is set.",
                    "type": "integer",
                    "format": "int32",
                },
                "driver": {
                    "description": "Driver is the DRA driver name for this pool. Must be a DNS subdomain (e.g., \"gpu.example.com\").",
                    "type": "string",
                },
                "generation": {
                    "description": "Generation is the pool generation observed across all ResourceSlices in this pool. Only the latest generation is reported. During a generation rollout, if not all slices at the latest generation have been published, the pool is included with a validationError and device counts unset.",
                    "type": "integer",
                    "format": "int64",
                },
                "nodeName": {
                    "description": "NodeName is the node this pool is associated with. When omitted, the pool is not associated with a specific node. Must be a valid DNS subdomain name (RFC1123).",
                    "type": "string",
                },
                "poolName": {
                    "description": "PoolName is the name of the pool. Must be a valid resource pool name (DNS subdomains separated by \"/\").",
                    "type": "string",
                },
                "resourceSliceCount": {
                    "description": "ResourceSliceCount is the number of ResourceSlices that make up this pool. May be unset when validationError is set.",
                    "type": "integer",
                    "format": "int32",
                },
                "totalDevices": {
                    "description": "TotalDevices is the total number of devices in the pool across all slices. A value of 0 means the pool has no devices. May be unset when validationError is set.",
                    "type": "integer",
                    "format": "int32",
                },
                "unavailableDevices": {
                    "description": "UnavailableDevices is the number of devices that are not available due to taints or other conditions, but are not allocated. A value of 0 means all unallocated devices are available. May be unset when validationError is set.",
                    "type": "integer",
                    "format": "int32",
                },
                "validationError": {
                    "description": "ValidationError is set when the pool's data could not be fully validated (e.g., incomplete slice publication). When set, device count fields and ResourceSliceCount may be unset.",
                    "type": "string",
                },
            },
            "required": [
                "driver",
                "generation",
                "poolName",
            ],
        })
    }
}
