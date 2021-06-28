// Generated from definition io.k8s.api.core.v1.ServiceStatus

/// ServiceStatus represents the current status of a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceStatus {
    /// Current service state
    pub conditions: Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>,

    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    pub load_balancer: Option<crate::api::core::v1::LoadBalancerStatus>,
}

impl<'de> crate::serde::Deserialize<'de> for ServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
            Key_load_balancer,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_load_balancer: Option<crate::api::core::v1::LoadBalancerStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer => value_load_balancer = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceStatus {
                    conditions: value_conditions.unwrap_or_default(),
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

impl crate::serde::Serialize for ServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceStatus",
            usize::from(!self.conditions.is_empty()) +
            self.load_balancer.as_ref().map_or(0, |_| 1),
        )?;
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        if let Some(value) = &self.load_balancer {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancer", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ServiceStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ServiceStatus represents the current status of a service.",
          "properties": {
            "conditions": {
              "description": "Current service state",
              "items": crate::apimachinery::pkg::apis::meta::v1::Condition::schema(),
              "type": "array"
            },
            "loadBalancer": crate::schema_ref_with_description(crate::api::core::v1::LoadBalancerStatus::schema(), "LoadBalancer contains the current status of the load-balancer, if one is present.")
          },
          "type": "object"
        })
    }
}
