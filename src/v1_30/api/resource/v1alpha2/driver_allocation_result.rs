// Generated from definition io.k8s.api.resource.v1alpha2.DriverAllocationResult

/// DriverAllocationResult contains vendor parameters and the allocation result for one request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DriverAllocationResult {
    /// NamedResources describes the allocation result when using the named resources model.
    pub named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesAllocationResult>,

    /// VendorRequestParameters are the per-request configuration parameters from the time that the claim was allocated.
    pub vendor_request_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for DriverAllocationResult {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.named_resources, other.named_resources);
        crate::DeepMerge::merge_from(&mut self.vendor_request_parameters, other.vendor_request_parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DriverAllocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_named_resources,
            Key_vendor_request_parameters,
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
                            "namedResources" => Field::Key_named_resources,
                            "vendorRequestParameters" => Field::Key_vendor_request_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DriverAllocationResult;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DriverAllocationResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesAllocationResult> = None;
                let mut value_vendor_request_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_named_resources => value_named_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_request_parameters => value_vendor_request_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DriverAllocationResult {
                    named_resources: value_named_resources,
                    vendor_request_parameters: value_vendor_request_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "DriverAllocationResult",
            &[
                "namedResources",
                "vendorRequestParameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DriverAllocationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DriverAllocationResult",
            self.named_resources.as_ref().map_or(0, |_| 1) +
            self.vendor_request_parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.named_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namedResources", value)?;
        }
        if let Some(value) = &self.vendor_request_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorRequestParameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DriverAllocationResult {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.DriverAllocationResult".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DriverAllocationResult contains vendor parameters and the allocation result for one request.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "namedResources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesAllocationResult>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NamedResources describes the allocation result when using the named resources model.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vendorRequestParameters".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorRequestParameters are the per-request configuration parameters from the time that the claim was allocated.".into()),
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
