// Generated from definition io.k8s.api.resource.v1alpha3.DeviceTaintSelector

/// DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceTaintSelector {
    /// If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices\[\].name.
    ///
    /// Setting also driver and pool may be required to avoid ambiguity, but is not required.
    pub device: Option<std::string::String>,

    /// If DeviceClassName is set, the selectors defined there must be satisfied by a device to be selected. This field corresponds to class.metadata.name.
    pub device_class_name: Option<std::string::String>,

    /// If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.
    pub driver: Option<std::string::String>,

    /// If pool is set, only devices in that pool are selected.
    ///
    /// Also setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.
    pub pool: Option<std::string::String>,

    /// Selectors contains the same selection criteria as a ResourceClaim. Currently, CEL expressions are supported. All of these selectors must be satisfied.
    pub selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>>,
}

impl crate::DeepMerge for DeviceTaintSelector {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.device, other.device);
        crate::DeepMerge::merge_from(&mut self.device_class_name, other.device_class_name);
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.pool, other.pool);
        crate::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceTaintSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_device,
            Key_device_class_name,
            Key_driver,
            Key_pool,
            Key_selectors,
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
                            "device" => Field::Key_device,
                            "deviceClassName" => Field::Key_device_class_name,
                            "driver" => Field::Key_driver,
                            "pool" => Field::Key_pool,
                            "selectors" => Field::Key_selectors,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceTaintSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceTaintSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_device: Option<std::string::String> = None;
                let mut value_device_class_name: Option<std::string::String> = None;
                let mut value_driver: Option<std::string::String> = None;
                let mut value_pool: Option<std::string::String> = None;
                let mut value_selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_device => value_device = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device_class_name => value_device_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pool => value_pool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceTaintSelector {
                    device: value_device,
                    device_class_name: value_device_class_name,
                    driver: value_driver,
                    pool: value_pool,
                    selectors: value_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceTaintSelector",
            &[
                "device",
                "deviceClassName",
                "driver",
                "pool",
                "selectors",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceTaintSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceTaintSelector",
            self.device.as_ref().map_or(0, |_| 1) +
            self.device_class_name.as_ref().map_or(0, |_| 1) +
            self.driver.as_ref().map_or(0, |_| 1) +
            self.pool.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.device {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "device", value)?;
        }
        if let Some(value) = &self.device_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deviceClassName", value)?;
        }
        if let Some(value) = &self.driver {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", value)?;
        }
        if let Some(value) = &self.pool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pool", value)?;
        }
        if let Some(value) = &self.selectors {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectors", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceTaintSelector {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.DeviceTaintSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.",
            "type": "object",
            "properties": {
                "device": {
                    "description": "If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices[].name.\n\nSetting also driver and pool may be required to avoid ambiguity, but is not required.",
                    "type": "string",
                },
                "deviceClassName": {
                    "description": "If DeviceClassName is set, the selectors defined there must be satisfied by a device to be selected. This field corresponds to class.metadata.name.",
                    "type": "string",
                },
                "driver": {
                    "description": "If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.",
                    "type": "string",
                },
                "pool": {
                    "description": "If pool is set, only devices in that pool are selected.\n\nAlso setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.",
                    "type": "string",
                },
                "selectors": {
                    "description": "Selectors contains the same selection criteria as a ResourceClaim. Currently, CEL expressions are supported. All of these selectors must be satisfied.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceSelector>()),
                },
            },
        })
    }
}
