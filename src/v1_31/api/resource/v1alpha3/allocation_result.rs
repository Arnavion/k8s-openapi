// Generated from definition io.k8s.api.resource.v1alpha3.AllocationResult

/// AllocationResult contains attributes of an allocated resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllocationResult {
    /// Controller is the name of the DRA driver which handled the allocation. That driver is also responsible for deallocating the claim. It is empty when the claim can be deallocated without involving a driver.
    ///
    /// A driver may allocate devices provided by other drivers, so this driver name here can be different from the driver names listed for the results.
    ///
    /// This is an alpha field and requires enabling the DRAControlPlaneController feature gate.
    pub controller: Option<std::string::String>,

    /// Devices is the result of allocating devices.
    pub devices: Option<crate::api::resource::v1alpha3::DeviceAllocationResult>,

    /// NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
    pub node_selector: Option<crate::api::core::v1::NodeSelector>,
}

impl crate::DeepMerge for AllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.controller, other.controller);
        crate::DeepMerge::merge_from(&mut self.devices, other.devices);
        crate::DeepMerge::merge_from(&mut self.node_selector, other.node_selector);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller,
            Key_devices,
            Key_node_selector,
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
                            "controller" => Field::Key_controller,
                            "devices" => Field::Key_devices,
                            "nodeSelector" => Field::Key_node_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AllocationResult;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("AllocationResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_controller: Option<std::string::String> = None;
                let mut value_devices: Option<crate::api::resource::v1alpha3::DeviceAllocationResult> = None;
                let mut value_node_selector: Option<crate::api::core::v1::NodeSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller => value_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllocationResult {
                    controller: value_controller,
                    devices: value_devices,
                    node_selector: value_node_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllocationResult",
            &[
                "controller",
                "devices",
                "nodeSelector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AllocationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllocationResult",
            self.controller.as_ref().map_or(0, |_| 1) +
            self.devices.as_ref().map_or(0, |_| 1) +
            self.node_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        if let Some(value) = &self.devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devices", value)?;
        }
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AllocationResult {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.AllocationResult".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("AllocationResult contains attributes of an allocated resource.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "controller".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Controller is the name of the DRA driver which handled the allocation. That driver is also responsible for deallocating the claim. It is empty when the claim can be deallocated without involving a driver.\n\nA driver may allocate devices provided by other drivers, so this driver name here can be different from the driver names listed for the results.\n\nThis is an alpha field and requires enabling the DRAControlPlaneController feature gate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "devices".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::DeviceAllocationResult>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Devices is the result of allocating devices.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nodeSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
