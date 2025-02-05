// Generated from definition io.k8s.api.resource.v1alpha2.StructuredResourceHandle

/// StructuredResourceHandle is the in-tree representation of the allocation result.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StructuredResourceHandle {
    /// NodeName is the name of the node providing the necessary resources if the resources are local to a node.
    pub node_name: Option<std::string::String>,

    /// Results lists all allocated driver resources.
    pub results: std::vec::Vec<crate::api::resource::v1alpha2::DriverAllocationResult>,

    /// VendorClaimParameters are the per-claim configuration parameters from the resource claim parameters at the time that the claim was allocated.
    pub vendor_claim_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,

    /// VendorClassParameters are the per-claim configuration parameters from the resource class at the time that the claim was allocated.
    pub vendor_class_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for StructuredResourceHandle {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::merge_strategies::list::atomic(&mut self.results, other.results);
        crate::DeepMerge::merge_from(&mut self.vendor_claim_parameters, other.vendor_claim_parameters);
        crate::DeepMerge::merge_from(&mut self.vendor_class_parameters, other.vendor_class_parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StructuredResourceHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_name,
            Key_results,
            Key_vendor_claim_parameters,
            Key_vendor_class_parameters,
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
                            "nodeName" => Field::Key_node_name,
                            "results" => Field::Key_results,
                            "vendorClaimParameters" => Field::Key_vendor_claim_parameters,
                            "vendorClassParameters" => Field::Key_vendor_class_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StructuredResourceHandle;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StructuredResourceHandle")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_results: Option<std::vec::Vec<crate::api::resource::v1alpha2::DriverAllocationResult>> = None;
                let mut value_vendor_claim_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_vendor_class_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_results => value_results = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_claim_parameters => value_vendor_claim_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_class_parameters => value_vendor_class_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StructuredResourceHandle {
                    node_name: value_node_name,
                    results: value_results.unwrap_or_default(),
                    vendor_claim_parameters: value_vendor_claim_parameters,
                    vendor_class_parameters: value_vendor_class_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "StructuredResourceHandle",
            &[
                "nodeName",
                "results",
                "vendorClaimParameters",
                "vendorClassParameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StructuredResourceHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StructuredResourceHandle",
            1 +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.vendor_claim_parameters.as_ref().map_or(0, |_| 1) +
            self.vendor_class_parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "results", &self.results)?;
        if let Some(value) = &self.vendor_claim_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorClaimParameters", value)?;
        }
        if let Some(value) = &self.vendor_class_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorClassParameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StructuredResourceHandle {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.StructuredResourceHandle".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("StructuredResourceHandle is the in-tree representation of the allocation result.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nodeName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NodeName is the name of the node providing the necessary resources if the resources are local to a node.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "results".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Results lists all allocated driver resources.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::DriverAllocationResult>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vendorClaimParameters".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorClaimParameters are the per-claim configuration parameters from the resource claim parameters at the time that the claim was allocated.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vendorClassParameters".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorClassParameters are the per-claim configuration parameters from the resource class at the time that the claim was allocated.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "results".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
