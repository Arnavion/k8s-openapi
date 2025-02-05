// Generated from definition io.k8s.api.resource.v1alpha2.AllocationResult

/// AllocationResult contains attributes of an allocated resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllocationResult {
    /// This field will get set by the resource driver after it has allocated the resource to inform the scheduler where it can schedule Pods using the ResourceClaim.
    ///
    /// Setting this field is optional. If null, the resource is available everywhere.
    pub available_on_nodes: Option<crate::api::core::v1::NodeSelector>,

    /// ResourceHandles contain the state associated with an allocation that should be maintained throughout the lifetime of a claim. Each ResourceHandle contains data that should be passed to a specific kubelet plugin once it lands on a node. This data is returned by the driver after a successful allocation and is opaque to Kubernetes. Driver documentation may explain to users how to interpret this data if needed.
    ///
    /// Setting this field is optional. It has a maximum size of 32 entries. If null (or empty), it is assumed this allocation will be processed by a single kubelet plugin with no ResourceHandle data attached. The name of the kubelet plugin invoked will match the DriverName set in the ResourceClaimStatus this AllocationResult is embedded in.
    pub resource_handles: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceHandle>>,

    /// Shareable determines whether the resource supports more than one consumer at a time.
    pub shareable: Option<bool>,
}

impl crate::DeepMerge for AllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.available_on_nodes, other.available_on_nodes);
        crate::merge_strategies::list::atomic(&mut self.resource_handles, other.resource_handles);
        crate::DeepMerge::merge_from(&mut self.shareable, other.shareable);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_on_nodes,
            Key_resource_handles,
            Key_shareable,
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
                            "availableOnNodes" => Field::Key_available_on_nodes,
                            "resourceHandles" => Field::Key_resource_handles,
                            "shareable" => Field::Key_shareable,
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
                let mut value_available_on_nodes: Option<crate::api::core::v1::NodeSelector> = None;
                let mut value_resource_handles: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceHandle>> = None;
                let mut value_shareable: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_on_nodes => value_available_on_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_handles => value_resource_handles = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_shareable => value_shareable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllocationResult {
                    available_on_nodes: value_available_on_nodes,
                    resource_handles: value_resource_handles,
                    shareable: value_shareable,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllocationResult",
            &[
                "availableOnNodes",
                "resourceHandles",
                "shareable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AllocationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllocationResult",
            self.available_on_nodes.as_ref().map_or(0, |_| 1) +
            self.resource_handles.as_ref().map_or(0, |_| 1) +
            self.shareable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.available_on_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableOnNodes", value)?;
        }
        if let Some(value) = &self.resource_handles {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceHandles", value)?;
        }
        if let Some(value) = &self.shareable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareable", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AllocationResult {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.AllocationResult".into()
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
                        "availableOnNodes".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("This field will get set by the resource driver after it has allocated the resource to inform the scheduler where it can schedule Pods using the ResourceClaim.\n\nSetting this field is optional. If null, the resource is available everywhere.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceHandles".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceHandles contain the state associated with an allocation that should be maintained throughout the lifetime of a claim. Each ResourceHandle contains data that should be passed to a specific kubelet plugin once it lands on a node. This data is returned by the driver after a successful allocation and is opaque to Kubernetes. Driver documentation may explain to users how to interpret this data if needed.\n\nSetting this field is optional. It has a maximum size of 32 entries. If null (or empty), it is assumed this allocation will be processed by a single kubelet plugin with no ResourceHandle data attached. The name of the kubelet plugin invoked will match the DriverName set in the ResourceClaimStatus this AllocationResult is embedded in.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::ResourceHandle>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shareable".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Shareable determines whether the resource supports more than one consumer at a time.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
