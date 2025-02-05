// Generated from definition io.k8s.api.resource.v1alpha3.ResourceClaimSpec

/// ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimSpec {
    /// Controller is the name of the DRA driver that is meant to handle allocation of this claim. If empty, allocation is handled by the scheduler while scheduling a pod.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    ///
    /// This is an alpha field and requires enabling the DRAControlPlaneController feature gate.
    pub controller: Option<std::string::String>,

    /// Devices defines how to request devices.
    pub devices: Option<crate::api::resource::v1alpha3::DeviceClaim>,
}

impl crate::DeepMerge for ResourceClaimSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.controller, other.controller);
        crate::DeepMerge::merge_from(&mut self.devices, other.devices);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller,
            Key_devices,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceClaimSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_controller: Option<std::string::String> = None;
                let mut value_devices: Option<crate::api::resource::v1alpha3::DeviceClaim> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller => value_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_devices => value_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimSpec {
                    controller: value_controller,
                    devices: value_devices,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceClaimSpec",
            &[
                "controller",
                "devices",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceClaimSpec",
            self.controller.as_ref().map_or(0, |_| 1) +
            self.devices.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        if let Some(value) = &self.devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "devices", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.ResourceClaimSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "controller".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Controller is the name of the DRA driver that is meant to handle allocation of this claim. If empty, allocation is handled by the scheduler while scheduling a pod.\n\nMust be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.\n\nThis is an alpha field and requires enabling the DRAControlPlaneController feature gate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "devices".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::DeviceClaim>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Devices defines how to request devices.".into()),
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
