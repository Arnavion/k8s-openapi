// Generated from definition io.k8s.api.resource.v1alpha3.DeviceRequest

/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequest {
    /// AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    ///
    /// This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
    pub admin_access: Option<bool>,

    /// AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:
    ///
    /// - ExactCount: This request is for a specific number of devices.
    ///   This is the default. The exact number is provided in the
    ///   count field.
    ///
    /// - All: This request is for all of the matching devices in a pool.
    ///   At least one device must exist on the node for the allocation to succeed.
    ///   Allocation will fail if some devices are already allocated,
    ///   unless adminAccess is requested.
    ///
    /// If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    pub allocation_mode: Option<std::string::String>,

    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    pub count: Option<i64>,

    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.
    ///
    /// A class is required if no subrequests are specified in the firstAvailable list and no class can be set if subrequests are specified in the firstAvailable list. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    pub device_class_name: Option<std::string::String>,

    /// FirstAvailable contains subrequests, of which exactly one will be satisfied by the scheduler to satisfy this request. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one cannot be used.
    ///
    /// This field may only be set in the entries of DeviceClaim.Requests.
    ///
    /// DRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.
    pub first_available: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSubRequest>>,

    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// Must be a DNS label.
    pub name: std::string::String,

    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    pub selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>>,

    /// If specified, the request's tolerations.
    ///
    /// Tolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.
    ///
    /// In addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.
    ///
    /// The maximum number of tolerations is 16.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    pub tolerations: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceToleration>>,
}

impl crate::DeepMerge for DeviceRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.admin_access, other.admin_access);
        crate::DeepMerge::merge_from(&mut self.allocation_mode, other.allocation_mode);
        crate::DeepMerge::merge_from(&mut self.count, other.count);
        crate::DeepMerge::merge_from(&mut self.device_class_name, other.device_class_name);
        crate::merge_strategies::list::atomic(&mut self.first_available, other.first_available);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
        crate::merge_strategies::list::atomic(&mut self.tolerations, other.tolerations);
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
            Key_first_available,
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
                            "adminAccess" => Field::Key_admin_access,
                            "allocationMode" => Field::Key_allocation_mode,
                            "count" => Field::Key_count,
                            "deviceClassName" => Field::Key_device_class_name,
                            "firstAvailable" => Field::Key_first_available,
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
            type Value = DeviceRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_admin_access: Option<bool> = None;
                let mut value_allocation_mode: Option<std::string::String> = None;
                let mut value_count: Option<i64> = None;
                let mut value_device_class_name: Option<std::string::String> = None;
                let mut value_first_available: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSubRequest>> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_selectors: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceSelector>> = None;
                let mut value_tolerations: Option<std::vec::Vec<crate::api::resource::v1alpha3::DeviceToleration>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admin_access => value_admin_access = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocation_mode => value_allocation_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_count => value_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device_class_name => value_device_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_first_available => value_first_available = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequest {
                    admin_access: value_admin_access,
                    allocation_mode: value_allocation_mode,
                    count: value_count,
                    device_class_name: value_device_class_name,
                    first_available: value_first_available,
                    name: value_name.unwrap_or_default(),
                    selectors: value_selectors,
                    tolerations: value_tolerations,
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
                "firstAvailable",
                "name",
                "selectors",
                "tolerations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceRequest",
            1 +
            self.admin_access.as_ref().map_or(0, |_| 1) +
            self.allocation_mode.as_ref().map_or(0, |_| 1) +
            self.count.as_ref().map_or(0, |_| 1) +
            self.device_class_name.as_ref().map_or(0, |_| 1) +
            self.first_available.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.device_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deviceClassName", value)?;
        }
        if let Some(value) = &self.first_available {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "firstAvailable", value)?;
        }
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
impl crate::schemars::JsonSchema for DeviceRequest {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "adminAccess".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.\n\nThis field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.\n\nThis is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.".into()),
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
                                description: Some("AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:\n\n- ExactCount: This request is for a specific number of devices.\n  This is the default. The exact number is provided in the\n  count field.\n\n- All: This request is for all of the matching devices in a pool.\n  At least one device must exist on the node for the allocation to succeed.\n  Allocation will fail if some devices are already allocated,\n  unless adminAccess is requested.\n\nIf AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.\n\nThis field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.\n\nMore modes may get added in the future. Clients must refuse to handle requests with unknown modes.".into()),
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
                                description: Some("Count is used only when the count mode is \"ExactCount\". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.\n\nThis field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.".into()),
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
                                description: Some("DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.\n\nA class is required if no subrequests are specified in the firstAvailable list and no class can be set if subrequests are specified in the firstAvailable list. Which classes are available depends on the cluster.\n\nAdministrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "firstAvailable".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("FirstAvailable contains subrequests, of which exactly one will be satisfied by the scheduler to satisfy this request. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one cannot be used.\n\nThis field may only be set in the entries of DeviceClaim.Requests.\n\nDRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha3::DeviceSubRequest>()))),
                                ..Default::default()
                            })),
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
                                description: Some("Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.\n\nThis field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.".into()),
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
                                description: Some("If specified, the request's tolerations.\n\nTolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.\n\nIn addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.\n\nThe maximum number of tolerations is 16.\n\nThis field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.\n\nThis is an alpha field and requires enabling the DRADeviceTaints feature gate.".into()),
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
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
