// Generated from definition io.k8s.api.resource.v1alpha3.DeviceRequestAllocationResult

/// DeviceRequestAllocationResult contains the allocation result for one request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequestAllocationResult {
    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    pub device: String,

    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    pub driver: String,

    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: String,

    /// Request is the name of the request in the claim which caused this device to be allocated. Multiple devices may have been allocated per request.
    pub request: String,
}

impl crate::DeepMerge for DeviceRequestAllocationResult {
    fn merge_from(&mut self, other: Self) {
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

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DeviceRequestAllocationResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_device: Option<String> = None;
                let mut value_driver: Option<String> = None;
                let mut value_pool: Option<String> = None;
                let mut value_request: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_device => value_device = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequestAllocationResult {
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
            4,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "device", &self.device)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", &self.pool)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceRequestAllocationResult {
    fn schema_name() -> String {
        "io.k8s.api.resource.v1alpha3.DeviceRequestAllocationResult".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceRequestAllocationResult contains the allocation result for one request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "device".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Device references one device instance via its name in the driver's resource pool. It must be a DNS label.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "driver".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pool".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).\n\nMust not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "request".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Request is the name of the request in the claim which caused this device to be allocated. Multiple devices may have been allocated per request.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "device".to_owned(),
                    "driver".to_owned(),
                    "pool".to_owned(),
                    "request".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
