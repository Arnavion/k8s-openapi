// Generated from definition io.k8s.api.resource.v1alpha2.VendorParameters

/// VendorParameters are opaque parameters for one particular driver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VendorParameters {
    /// DriverName is the name used by the DRA driver kubelet plugin.
    pub driver_name: Option<String>,

    /// Parameters can be arbitrary setup parameters. They are ignored while allocating a claim.
    pub parameters: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for VendorParameters {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::DeepMerge::merge_from(&mut self.parameters, other.parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VendorParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver_name,
            Key_parameters,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "driverName" => Field::Key_driver_name,
                            "parameters" => Field::Key_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VendorParameters;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VendorParameters")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver_name: Option<String> = None;
                let mut value_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VendorParameters {
                    driver_name: value_driver_name,
                    parameters: value_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "VendorParameters",
            &[
                "driverName",
                "parameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VendorParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VendorParameters",
            self.driver_name.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.driver_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", value)?;
        }
        if let Some(value) = &self.parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VendorParameters {
    fn schema_name() -> String {
        "io.k8s.api.resource.v1alpha2.VendorParameters".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VendorParameters are opaque parameters for one particular driver.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "driverName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName is the name used by the DRA driver kubelet plugin.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parameters".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Parameters can be arbitrary setup parameters. They are ignored while allocating a claim.".to_owned()),
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
