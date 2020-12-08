// Generated from definition io.k8s.api.core.v1.ServiceStatus

/// ServiceStatus represents the current status of a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceStatus {
    /// Current service state
    pub conditions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    pub load_balancer: Option<crate::api::core::v1::LoadBalancerStatus>,
}

impl<'de> serde::Deserialize<'de> for ServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_load_balancer,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "conditions" => Field::Key_conditions,
                            "loadBalancer" => Field::Key_load_balancer,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_load_balancer: Option<crate::api::core::v1::LoadBalancerStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer => value_load_balancer = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceStatus {
                    conditions: value_conditions,
                    load_balancer: value_load_balancer,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceStatus",
            &[
                "conditions",
                "loadBalancer",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceStatus",
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.load_balancer.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.load_balancer {
            serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancer", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
