// Generated from definition io.k8s.api.resource.v1beta1.OpaqueDeviceConfiguration

/// OpaqueDeviceConfiguration contains configuration parameters for a driver in a format defined by the driver vendor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpaqueDeviceConfiguration {
    /// Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.
    ///
    /// An admission policy provided by the driver developer could use this to decide whether it needs to validate them.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    pub driver: String,

    /// Parameters can contain arbitrary data. It is the responsibility of the driver developer to handle validation and versioning. Typically this includes self-identification and a version ("kind" + "apiVersion" for Kubernetes types), with conversion between different versions.
    ///
    /// The length of the raw data must be smaller or equal to 10 Ki.
    pub parameters: crate::apimachinery::pkg::runtime::RawExtension,
}

impl crate::DeepMerge for OpaqueDeviceConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.parameters, other.parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for OpaqueDeviceConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_parameters,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "driver" => Field::Key_driver,
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
            type Value = OpaqueDeviceConfiguration;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("OpaqueDeviceConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<String> = None;
                let mut value_parameters: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OpaqueDeviceConfiguration {
                    driver: value_driver.unwrap_or_default(),
                    parameters: value_parameters.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "OpaqueDeviceConfiguration",
            &[
                "driver",
                "parameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for OpaqueDeviceConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "OpaqueDeviceConfiguration",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", &self.parameters)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for OpaqueDeviceConfiguration {
    fn schema_name() -> String {
        "io.k8s.api.resource.v1beta1.OpaqueDeviceConfiguration".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("OpaqueDeviceConfiguration contains configuration parameters for a driver in a format defined by the driver vendor.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "driver".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.\n\nAn admission policy provided by the driver developer could use this to decide whether it needs to validate them.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.".to_owned()),
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
                                description: Some("Parameters can contain arbitrary data. It is the responsibility of the driver developer to handle validation and versioning. Typically this includes self-identification and a version (\"kind\" + \"apiVersion\" for Kubernetes types), with conversion between different versions.\n\nThe length of the raw data must be smaller or equal to 10 Ki.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "driver".to_owned(),
                    "parameters".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
