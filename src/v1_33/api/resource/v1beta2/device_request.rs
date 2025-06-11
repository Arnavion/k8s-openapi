// Generated from definition io.k8s.api.resource.v1beta2.DeviceRequest

/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices. With FirstAvailable it is also possible to provide a prioritized list of requests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceRequest {
    /// Exactly specifies the details for a single request that must be met exactly for the request to be satisfied.
    ///
    /// One of Exactly or FirstAvailable must be set.
    pub exactly: Option<crate::api::resource::v1beta2::ExactDeviceRequest>,

    /// FirstAvailable contains subrequests, of which exactly one will be selected by the scheduler. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one can not be used.
    ///
    /// DRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.
    pub first_available: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceSubRequest>>,

    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// References using the name in the DeviceRequest will uniquely identify a request when the Exactly field is set. When the FirstAvailable field is set, a reference to the name of the DeviceRequest will match whatever subrequest is chosen by the scheduler.
    ///
    /// Must be a DNS label.
    pub name: std::string::String,
}

impl crate::DeepMerge for DeviceRequest {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.exactly, other.exactly);
        crate::merge_strategies::list::atomic(&mut self.first_available, other.first_available);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exactly,
            Key_first_available,
            Key_name,
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
                            "exactly" => Field::Key_exactly,
                            "firstAvailable" => Field::Key_first_available,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceRequest;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exactly: Option<crate::api::resource::v1beta2::ExactDeviceRequest> = None;
                let mut value_first_available: Option<std::vec::Vec<crate::api::resource::v1beta2::DeviceSubRequest>> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exactly => value_exactly = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_first_available => value_first_available = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceRequest {
                    exactly: value_exactly,
                    first_available: value_first_available,
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceRequest",
            &[
                "exactly",
                "firstAvailable",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceRequest",
            1 +
            self.exactly.as_ref().map_or(0, |_| 1) +
            self.first_available.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exactly {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exactly", value)?;
        }
        if let Some(value) = &self.first_available {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "firstAvailable", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceRequest {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceRequest".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices. With FirstAvailable it is also possible to provide a prioritized list of requests.",
            "type": "object",
            "properties": {
                "exactly": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1beta2::ExactDeviceRequest>();
                    schema_obj.ensure_object().insert("description".into(), "Exactly specifies the details for a single request that must be met exactly for the request to be satisfied.\n\nOne of Exactly or FirstAvailable must be set.".into());
                    schema_obj
                }),
                "firstAvailable": {
                    "description": "FirstAvailable contains subrequests, of which exactly one will be selected by the scheduler. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one can not be used.\n\nDRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::resource::v1beta2::DeviceSubRequest>()),
                },
                "name": {
                    "description": "Name can be used to reference this request in a pod.spec.containers[].resources.claims entry and in a constraint of the claim.\n\nReferences using the name in the DeviceRequest will uniquely identify a request when the Exactly field is set. When the FirstAvailable field is set, a reference to the name of the DeviceRequest will match whatever subrequest is chosen by the scheduler.\n\nMust be a DNS label.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
