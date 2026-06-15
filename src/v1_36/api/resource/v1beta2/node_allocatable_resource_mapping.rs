// Generated from definition io.k8s.api.resource.v1beta2.NodeAllocatableResourceMapping

/// NodeAllocatableResourceMapping defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAllocatableResourceMapping {
    /// AllocationMultiplier is used as a multiplier for the allocated device count or the allocated capacity in the claim. It defaults to 1 if not specified. How the field is used also depends on whether `capacityKey` is set. 1.  If `capacityKey` is NOT set: `allocationMultiplier` multiplies the device count allocated to the claim.
    ///        a. A DRA driver representing each CPU core as a device would have
    ///        {ResourceName: "cpu", allocationMultiplier: "2"} in its
    ///        `nodeAllocatableResourceMappings`. If 4 devices are allocated to the claim,
    ///           4 * 2 CPUs would be considered as allocated and subtracted from the node's capacity.
    ///     b. A GPU device that needs additional node memory per GPU allocation would
    ///        have {ResourceName: "memory", allocationMultiplier: "2Gi"}.  Each allocated
    ///           GPU device instance of this type will account for 2Gi of memory.
    ///
    /// 2.  If `capacityKey` IS set: `allocationMultiplier` is multiplied by the amount of that capacity consumed.
    ///        The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `allocationMultiplier`.
    ///     For example, if a Device's capacity "dra.example.com/cores" is consumed,
    ///     and each "core" provides 2 "cpu"s, the mapping would be:
    ///     {ResourceName: "cpu", capacityKey: "dra.example.com/cores", allocationMultiplier: "2"}.
    ///     If a claim consumes 8 "dra.example.com/cores", the CPU footprint is 8 * 2 = 16.
    pub allocation_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// CapacityKey references a capacity name defined as a key in the `spec.devices\[*\].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results\[*\].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. If `allocationMultiplier` is also set, it is multiplied with the base quantity. For example, if `spec.devices\[*\].capacity` has an entry "dra.example.com/memory": "128Gi", and this field is set to "dra.example.com/memory", then for a claim allocation that consumes { "dra.example.com/memory": "4Gi" } the base quantity for the node allocatable resource mapping will be "4Gi", and `allocationMultiplier` should be omitted or set to "1".
    pub capacity_key: Option<std::string::String>,
}

impl crate::DeepMerge for NodeAllocatableResourceMapping {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocation_multiplier, other.allocation_multiplier);
        crate::DeepMerge::merge_from(&mut self.capacity_key, other.capacity_key);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeAllocatableResourceMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocation_multiplier,
            Key_capacity_key,
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
                            "allocationMultiplier" => Field::Key_allocation_multiplier,
                            "capacityKey" => Field::Key_capacity_key,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAllocatableResourceMapping;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeAllocatableResourceMapping")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocation_multiplier: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_capacity_key: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation_multiplier => value_allocation_multiplier = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity_key => value_capacity_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAllocatableResourceMapping {
                    allocation_multiplier: value_allocation_multiplier,
                    capacity_key: value_capacity_key,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAllocatableResourceMapping",
            &[
                "allocationMultiplier",
                "capacityKey",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeAllocatableResourceMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAllocatableResourceMapping",
            self.allocation_multiplier.as_ref().map_or(0, |_| 1) +
            self.capacity_key.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation_multiplier {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocationMultiplier", value)?;
        }
        if let Some(value) = &self.capacity_key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacityKey", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeAllocatableResourceMapping {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.NodeAllocatableResourceMapping".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeAllocatableResourceMapping defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource.",
            "type": "object",
            "properties": {
                "allocationMultiplier": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "AllocationMultiplier is used as a multiplier for the allocated device count or the allocated capacity in the claim. It defaults to 1 if not specified. How the field is used also depends on whether `capacityKey` is set. 1.  If `capacityKey` is NOT set: `allocationMultiplier` multiplies the device count allocated to the claim.\n\t   a. A DRA driver representing each CPU core as a device would have\n       {ResourceName: \"cpu\", allocationMultiplier: \"2\"} in its\n       `nodeAllocatableResourceMappings`. If 4 devices are allocated to the claim,\n\t\t  4 * 2 CPUs would be considered as allocated and subtracted from the node's capacity.\n    b. A GPU device that needs additional node memory per GPU allocation would\n       have {ResourceName: \"memory\", allocationMultiplier: \"2Gi\"}.  Each allocated\n\t\t  GPU device instance of this type will account for 2Gi of memory.\n\n2.  If `capacityKey` IS set: `allocationMultiplier` is multiplied by the amount of that capacity consumed.\n\t   The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `allocationMultiplier`.\n    For example, if a Device's capacity \"dra.example.com/cores\" is consumed,\n    and each \"core\" provides 2 \"cpu\"s, the mapping would be:\n    {ResourceName: \"cpu\", capacityKey: \"dra.example.com/cores\", allocationMultiplier: \"2\"}.\n    If a claim consumes 8 \"dra.example.com/cores\", the CPU footprint is 8 * 2 = 16.".into());
                    schema_obj
                }),
                "capacityKey": {
                    "description": "CapacityKey references a capacity name defined as a key in the `spec.devices[*].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results[*].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. If `allocationMultiplier` is also set, it is multiplied with the base quantity. For example, if `spec.devices[*].capacity` has an entry \"dra.example.com/memory\": \"128Gi\", and this field is set to \"dra.example.com/memory\", then for a claim allocation that consumes { \"dra.example.com/memory\": \"4Gi\" } the base quantity for the node allocatable resource mapping will be \"4Gi\", and `allocationMultiplier` should be omitted or set to \"1\".",
                    "type": "string",
                },
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for NodeAllocatableResourceMapping {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.NodeAllocatableResourceMapping".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("NodeAllocatableResourceMapping defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "allocationMultiplier".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("AllocationMultiplier is used as a multiplier for the allocated device count or the allocated capacity in the claim. It defaults to 1 if not specified. How the field is used also depends on whether `capacityKey` is set. 1.  If `capacityKey` is NOT set: `allocationMultiplier` multiplies the device count allocated to the claim.\n\t   a. A DRA driver representing each CPU core as a device would have\n       {ResourceName: \"cpu\", allocationMultiplier: \"2\"} in its\n       `nodeAllocatableResourceMappings`. If 4 devices are allocated to the claim,\n\t\t  4 * 2 CPUs would be considered as allocated and subtracted from the node's capacity.\n    b. A GPU device that needs additional node memory per GPU allocation would\n       have {ResourceName: \"memory\", allocationMultiplier: \"2Gi\"}.  Each allocated\n\t\t  GPU device instance of this type will account for 2Gi of memory.\n\n2.  If `capacityKey` IS set: `allocationMultiplier` is multiplied by the amount of that capacity consumed.\n\t   The final node allocatable resource amount is `consumedCapacity[capacityKey]` * `allocationMultiplier`.\n    For example, if a Device's capacity \"dra.example.com/cores\" is consumed,\n    and each \"core\" provides 2 \"cpu\"s, the mapping would be:\n    {ResourceName: \"cpu\", capacityKey: \"dra.example.com/cores\", allocationMultiplier: \"2\"}.\n    If a claim consumes 8 \"dra.example.com/cores\", the CPU footprint is 8 * 2 = 16.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "capacityKey".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("CapacityKey references a capacity name defined as a key in the `spec.devices[*].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results[*].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. If `allocationMultiplier` is also set, it is multiplied with the base quantity. For example, if `spec.devices[*].capacity` has an entry \"dra.example.com/memory\": \"128Gi\", and this field is set to \"dra.example.com/memory\", then for a claim allocation that consumes { \"dra.example.com/memory\": \"4Gi\" } the base quantity for the node allocatable resource mapping will be \"4Gi\", and `allocationMultiplier` should be omitted or set to \"1\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
