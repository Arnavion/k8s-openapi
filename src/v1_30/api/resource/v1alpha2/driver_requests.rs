// Generated from definition io.k8s.api.resource.v1alpha2.DriverRequests

/// DriverRequests describes all resources that are needed from one particular driver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DriverRequests {
    /// DriverName is the name used by the DRA driver kubelet plugin.
    pub driver_name: Option<std::string::String>,

    /// Requests describes all resources that are needed from the driver.
    pub requests: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceRequest>>,

    /// VendorParameters are arbitrary setup parameters for all requests of the claim. They are ignored while allocating the claim.
    pub vendor_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for DriverRequests {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::merge_strategies::list::atomic(&mut self.requests, other.requests);
        crate::DeepMerge::merge_from(&mut self.vendor_parameters, other.vendor_parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DriverRequests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver_name,
            Key_requests,
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
                            "driverName" => Field::Key_driver_name,
                            "requests" => Field::Key_requests,
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
            type Value = DriverRequests;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DriverRequests")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver_name: Option<std::string::String> = None;
                let mut value_requests: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceRequest>> = None;
                let mut value_vendor_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_parameters => value_vendor_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DriverRequests {
                    driver_name: value_driver_name,
                    requests: value_requests,
                    vendor_parameters: value_vendor_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "DriverRequests",
            &[
                "driverName",
                "requests",
                "vendorParameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DriverRequests {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DriverRequests",
            self.driver_name.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1) +
            self.vendor_parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.driver_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        if let Some(value) = &self.vendor_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorParameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DriverRequests {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.DriverRequests".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DriverRequests describes all resources that are needed from one particular driver.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "driverName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName is the name used by the DRA driver kubelet plugin.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "requests".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Requests describes all resources that are needed from the driver.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::ResourceRequest>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vendorParameters".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorParameters are arbitrary setup parameters for all requests of the claim. They are ignored while allocating the claim.".into()),
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
