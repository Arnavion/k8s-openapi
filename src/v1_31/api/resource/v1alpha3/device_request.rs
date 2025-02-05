// Generated from definition io.k8s.api.resource.v1alpha3.DeviceRequest

/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.
///
/// A DeviceClassName is currently required. Clients must check that it is indeed set. It's absence indicates that something changed in a way that is not supported by the client yet, in which case it must refuse to handle the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequest {
    /// AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.
    pub admin_access: Option<bool>,

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
    /// If AlloctionMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    pub allocation_mode: Option<std::string::String>,

    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    pub count: Option<i64>,

    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.
    ///
    /// A class is required. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    pub device_class_name: std::string::String,

    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// Must be a DNS label.
    pub name: std::string::String,

    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    pub selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>>,
}

impl crate::DeepMerge for DeviceRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.admin_access, other.admin_access);
        crate::DeepMerge::merge_from(&mut self.allocation_mode, other.allocation_mode);
        crate::DeepMerge::merge_from(&mut self.count, other.count);
        crate::DeepMerge::merge_from(&mut self.device_class_name, other.device_class_name);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admin_access,
            Key_allocation_mode,
            Key_count,
            Key_device_class_name,
            Key_name,
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
                            "adminAccess" => Field::Key_admin_access,
                            "allocationMode" => Field::Key_allocation_mode,
                            "count" => Field::Key_count,
                            "deviceClassName" => Field::Key_device_class_name,
                            "name" => Field::Key_name,
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
            type Value = DeviceRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_admin_access: Option<bool> = None;
                let mut value_allocation_mode: Option<std::string::String> = None;
                let mut value_count: Option<i64> = None;
                let mut value_device_class_name: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admin_access => value_admin_access = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocation_mode => value_allocation_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_count => value_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device_class_name => value_device_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequest {
                    admin_access: value_admin_access,
                    allocation_mode: value_allocation_mode,
                    count: value_count,
                    device_class_name: value_device_class_name.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    selectors: value_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceRequest",
            &[
                "adminAccess",
                "allocationMode",
                "count",
                "deviceClassName",
                "name",
                "selectors",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceRequest",
            2 +
            self.admin_access.as_ref().map_or(0, |_| 1) +
            self.allocation_mode.as_ref().map_or(0, |_| 1) +
            self.count.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.admin_access {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "adminAccess", value)?;
        }
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
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceRequest {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.\n\nA DeviceClassName is currently required. Clients must check that it is indeed set. It's absence indicates that something changed in a way that is not supported by the client yet, in which case it must refuse to handle the request.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "adminAccess".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocationMode".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:\n\n- ExactCount: This request is for a specific number of devices.\n  This is the default. The exact number is provided in the\n  count field.\n\n- All: This request is for all of the matching devices in a pool.\n  Allocation will fail if some devices are already allocated,\n  unless adminAccess is requested.\n\nIf AlloctionMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.\n\nMore modes may get added in the future. Clients must refuse to handle requests with unknown modes.".into()),
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
                                description: Some("DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.\n\nA class is required. Which classes are available depends on the cluster.\n\nAdministrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.".into()),
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
                                description: Some("Name can be used to reference this request in a pod.spec.containers[].resources.claims entry and in a constraint of the claim.\n\nMust be a DNS label.".into()),
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
