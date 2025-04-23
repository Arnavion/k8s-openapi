// Generated from definition io.k8s.api.resource.v1alpha3.DeviceSubRequest

/// DeviceSubRequest describes a request for device provided in the claim.spec.devices.requests\[\].firstAvailable array. Each is typically a request for a single resource like a device, but can also ask for several identical devices.
///
/// DeviceSubRequest is similar to Request, but doesn't expose the AdminAccess or FirstAvailable fields, as those can only be set on the top-level request. AdminAccess is not supported for requests with a prioritized list, and recursive FirstAvailable fields are not supported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceSubRequest {
    /// AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:
    ///
    /// - ExactCount: This request is for a specific number of devices.
    ///   This is the default. The exact number is provided in the
    ///   count field.
    ///
    /// - All: This request is for all of the matching devices in a pool.
    ///   Allocation will fail if some devices are already allocated,
    ///   unless adminAccess is requested.
    ///
    /// If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    pub allocation_mode: Option<std::string::String>,

    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    pub count: Option<i64>,

    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this subrequest.
    ///
    /// A class is required. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    pub device_class_name: std::string::String,

    /// Name can be used to reference this subrequest in the list of constraints or the list of configurations for the claim. References must use the format \<main request\>/\<subrequest\>.
    ///
    /// Must be a DNS label.
    pub name: std::string::String,

    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    pub selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>>,

    /// If specified, the request's tolerations.
    ///
    /// Tolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.
    ///
    /// In addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.
    ///
    /// The maximum number of tolerations is 16.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    pub tolerations: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceToleration>>,
}

impl crate::DeepMerge for DeviceSubRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocation_mode, other.allocation_mode);
        crate::DeepMerge::merge_from(&mut self.count, other.count);
        crate::DeepMerge::merge_from(&mut self.device_class_name, other.device_class_name);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
        crate::merge_strategies::list::atomic(&mut self.tolerations, other.tolerations);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceSubRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocation_mode,
            Key_count,
            Key_device_class_name,
            Key_name,
            Key_selectors,
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
                            "allocationMode" => Field::Key_allocation_mode,
                            "count" => Field::Key_count,
                            "deviceClassName" => Field::Key_device_class_name,
                            "name" => Field::Key_name,
                            "selectors" => Field::Key_selectors,
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
            type Value = DeviceSubRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceSubRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocation_mode: Option<std::string::String> = None;
                let mut value_count: Option<i64> = None;
                let mut value_device_class_name: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>> = None;
                let mut value_tolerations: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceToleration>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation_mode => value_allocation_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_count => value_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device_class_name => value_device_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceSubRequest {
                    allocation_mode: value_allocation_mode,
                    count: value_count,
                    device_class_name: value_device_class_name.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    selectors: value_selectors,
                    tolerations: value_tolerations,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceSubRequest",
            &[
                "allocationMode",
                "count",
                "deviceClassName",
                "name",
                "selectors",
                "tolerations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceSubRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceSubRequest",
            2 +
            self.allocation_mode.as_ref().map_or(0, |_| 1) +
            self.count.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocationMode", value)?;
        }
        if let Some(value) = &self.count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deviceClassName", &self.device_class_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.selectors {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectors", value)?;
        }
        if let Some(value) = &self.tolerations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceSubRequest {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceSubRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceSubRequest describes a request for device provided in the claim.spec.devices.requests[].firstAvailable array. Each is typically a request for a single resource like a device, but can also ask for several identical devices.\n\nDeviceSubRequest is similar to Request, but doesn't expose the AdminAccess or FirstAvailable fields, as those can only be set on the top-level request. AdminAccess is not supported for requests with a prioritized list, and recursive FirstAvailable fields are not supported.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocationMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:\n\n- ExactCount: This request is for a specific number of devices.\n  This is the default. The exact number is provided in the\n  count field.\n\n- All: This request is for all of the matching devices in a pool.\n  Allocation will fail if some devices are already allocated,\n  unless adminAccess is requested.\n\nIf AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.\n\nMore modes may get added in the future. Clients must refuse to handle requests with unknown modes.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "count".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Count is used only when the count mode is \"ExactCount\". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deviceClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this subrequest.\n\nA class is required. Which classes are available depends on the cluster.\n\nAdministrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name can be used to reference this subrequest in the list of constraints or the list of configurations for the claim. References must use the format <main request>/<subrequest>.\n\nMust be a DNS label.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selectors".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceSelector>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tolerations".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the request's tolerations.\n\nTolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.\n\nIn addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.\n\nThe maximum number of tolerations is 16.\n\nThis is an alpha field and requires enabling the DRADeviceTaints feature gate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceToleration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "deviceClassName".into(),
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
