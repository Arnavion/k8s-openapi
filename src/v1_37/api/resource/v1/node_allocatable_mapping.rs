// Generated from definition io.k8s.api.resource.v1.NodeAllocatableMapping

/// NodeAllocatableMapping defines how a DRA allocation directly translates into a node allocatable resource quantity. The mapping can be derived from either the count of allocated devices (via deviceMultiplier) or the specific capacity consumed (via capacityKey and capacityMultiplier). These options are mutually exclusive. Kubelet adds this mapped resource quantity from claim to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableMapping {
    /// CapacityKey references a capacity name defined as a key in the `spec.devices\[*\].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results\[*\].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. `capacityMultiplier` must also be set and is multiplied with the base quantity. For example, if `spec.devices\[*\].capacity` has an entry "dra.example.com/memory": "128Gi", and this field is set to "dra.example.com/memory", then for a claim allocation that consumes { "dra.example.com/memory": "4Gi" } the base quantity for the node allocatable resource mapping will be "4Gi". The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `capacityMultiplier`.
    pub capacity_key: Option<std::string::String>,

    /// CapacityMultiplier is used as a multiplier for the allocated capacity consumed. It is only valid if `capacityKey` is set. The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `capacityMultiplier`. For example, if a Device's capacity "dra.example.com/cores" is consumed, and each "core" provides 2 "cpu"s, the mapping would be: {ResourceName: "cpu", capacityKey: "dra.example.com/cores", capacityMultiplier: "2"}. If a claim consumes 8 "dra.example.com/cores", the CPU footprint is 8 * 2 = 16.
    pub capacity_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// DeviceMultiplier is used as a multiplier for the allocated device count in the claim. The final node allocatable resource amount is `deviceCount` * `deviceMultiplier`. For example, a DRA driver representing each cache complex (CCX) as a device would have {ResourceName: "cpu", deviceMultiplier: "8"} in its `nodeAllocatableResources`. If 2 devices (CCX) are allocated to the claim, 2 * 8 = 16 CPUs would be considered as allocated. It is only valid when `capacityKey` and `capacityMultiplier` are not set.
    pub device_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity>,
}

impl crate::DeepMerge for NodeAllocatableMapping {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.capacity_key, other.capacity_key);
        crate::DeepMerge::merge_from(&mut self.capacity_multiplier, other.capacity_multiplier);
        crate::DeepMerge::merge_from(&mut self.device_multiplier, other.device_multiplier);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_capacity_key,
            Key_capacity_multiplier,
            Key_device_multiplier,
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
                            "capacityKey" => Field::Key_capacity_key,
                            "capacityMultiplier" => Field::Key_capacity_multiplier,
                            "deviceMultiplier" => Field::Key_device_multiplier,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableMapping;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableMapping")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_capacity_key: Option<std::string::String> = None;
                let mut value_capacity_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_device_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_capacity_key => value_capacity_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity_multiplier => value_capacity_multiplier = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_device_multiplier => value_device_multiplier = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableMapping {
                    capacity_key: value_capacity_key,
                    capacity_multiplier: value_capacity_multiplier,
                    device_multiplier: value_device_multiplier,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableMapping",
            &[
                "capacityKey",
                "capacityMultiplier",
                "deviceMultiplier",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableMapping",
            self.capacity_key.as_ref().map_or(0, |_| 1) +
            self.capacity_multiplier.as_ref().map_or(0, |_| 1) +
            self.device_multiplier.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.capacity_key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacityKey", value)?;
        }
        if let Some(value) = &self.capacity_multiplier {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacityMultiplier", value)?;
        }
        if let Some(value) = &self.device_multiplier {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deviceMultiplier", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableMapping {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1.NodeAllocatableMapping".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableMapping defines how a DRA allocation directly translates into a node allocatable resource quantity. The mapping can be derived from either the count of allocated devices (via deviceMultiplier) or the specific capacity consumed (via capacityKey and capacityMultiplier). These options are mutually exclusive. Kubelet adds this mapped resource quantity from claim to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.",
            "type": "object",
            "properties": {
                "capacityKey": {
                    "description": "CapacityKey references a capacity name defined as a key in the `spec.devices[*].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results[*].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. `capacityMultiplier` must also be set and is multiplied with the base quantity. For example, if `spec.devices[*].capacity` has an entry \"dra.example.com/memory\": \"128Gi\", and this field is set to \"dra.example.com/memory\", then for a claim allocation that consumes { \"dra.example.com/memory\": \"4Gi\" } the base quantity for the node allocatable resource mapping will be \"4Gi\". The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `capacityMultiplier`.",
                    "type": "string",
                },
                "capacityMultiplier": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "CapacityMultiplier is used as a multiplier for the allocated capacity consumed. It is only valid if `capacityKey` is set. The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `capacityMultiplier`. For example, if a Device's capacity \"dra.example.com/cores\" is consumed, and each \"core\" provides 2 \"cpu\"s, the mapping would be: {ResourceName: \"cpu\", capacityKey: \"dra.example.com/cores\", capacityMultiplier: \"2\"}. If a claim consumes 8 \"dra.example.com/cores\", the CPU footprint is 8 * 2 = 16.".into());
                    schema_obj
                }),
                "deviceMultiplier": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "DeviceMultiplier is used as a multiplier for the allocated device count in the claim. The final node allocatable resource amount is `deviceCount` * `deviceMultiplier`. For example, a DRA driver representing each cache complex (CCX) as a device would have {ResourceName: \"cpu\", deviceMultiplier: \"8\"} in its `nodeAllocatableResources`. If 2 devices (CCX) are allocated to the claim, 2 * 8 = 16 CPUs would be considered as allocated. It is only valid when `capacityKey` and `capacityMultiplier` are not set.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableMapping {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1.NodeAllocatableMapping".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableMapping defines how a DRA allocation directly translates into a node allocatable resource quantity. The mapping can be derived from either the count of allocated devices (via deviceMultiplier) or the specific capacity consumed (via capacityKey and capacityMultiplier). These options are mutually exclusive. Kubelet adds this mapped resource quantity from claim to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "capacityKey".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("CapacityKey references a capacity name defined as a key in the `spec.devices[*].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results[*].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. `capacityMultiplier` must also be set and is multiplied with the base quantity. For example, if `spec.devices[*].capacity` has an entry \"dra.example.com/memory\": \"128Gi\", and this field is set to \"dra.example.com/memory\", then for a claim allocation that consumes { \"dra.example.com/memory\": \"4Gi\" } the base quantity for the node allocatable resource mapping will be \"4Gi\". The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `capacityMultiplier`.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacityMultiplier".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("CapacityMultiplier is used as a multiplier for the allocated capacity consumed. It is only valid if `capacityKey` is set. The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `capacityMultiplier`. For example, if a Device's capacity \"dra.example.com/cores\" is consumed, and each \"core\" provides 2 \"cpu\"s, the mapping would be: {ResourceName: \"cpu\", capacityKey: \"dra.example.com/cores\", capacityMultiplier: \"2\"}. If a claim consumes 8 \"dra.example.com/cores\", the CPU footprint is 8 * 2 = 16.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deviceMultiplier".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("DeviceMultiplier is used as a multiplier for the allocated device count in the claim. The final node allocatable resource amount is `deviceCount` * `deviceMultiplier`. For example, a DRA driver representing each cache complex (CCX) as a device would have {ResourceName: \"cpu\", deviceMultiplier: \"8\"} in its `nodeAllocatableResources`. If 2 devices (CCX) are allocated to the claim, 2 * 8 = 16 CPUs would be considered as allocated. It is only valid when `capacityKey` and `capacityMultiplier` are not set.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
