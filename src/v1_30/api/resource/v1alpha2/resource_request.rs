// Generated from definition io.k8s.api.resource.v1alpha2.ResourceRequest

/// ResourceRequest is a request for resources from one particular driver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRequest {
    /// NamedResources describes a request for resources with the named resources model.
    pub named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesRequest>,

    /// VendorParameters are arbitrary setup parameters for the requested resource. They are ignored while allocating a claim.
    pub vendor_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for ResourceRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.named_resources, other.named_resources);
        crate::DeepMerge::merge_from(&mut self.vendor_parameters, other.vendor_parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_named_resources,
            Key_vendor_parameters,
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
                            "vendorParameters" => Field::Key_vendor_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_named_resources: Option<crate::api::resource::v1alpha2::NamedResourcesRequest> = None;
                let mut value_vendor_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_named_resources => value_named_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_parameters => value_vendor_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRequest {
                    named_resources: value_named_resources,
                    vendor_parameters: value_vendor_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceRequest",
            &[
                "namedResources",
                "vendorParameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRequest",
            self.named_resources.as_ref().map_or(0, |_| 1) +
            self.vendor_parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.named_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namedResources", value)?;
        }
        if let Some(value) = &self.vendor_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorParameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceRequest {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceRequest is a request for resources from one particular driver.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "namedResources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::NamedResourcesRequest>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NamedResources describes a request for resources with the named resources model.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vendorParameters".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorParameters are arbitrary setup parameters for the requested resource. They are ignored while allocating a claim.".into()),
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
