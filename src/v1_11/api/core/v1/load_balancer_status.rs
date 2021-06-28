// Generated from definition io.k8s.api.core.v1.LoadBalancerStatus

/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    pub ingress: Vec<crate::api::core::v1::LoadBalancerIngress>,
}

impl<'de> crate::serde::Deserialize<'de> for LoadBalancerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ingress,
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
                            "ingress" => Field::Key_ingress,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LoadBalancerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ingress: Option<Vec<crate::api::core::v1::LoadBalancerIngress>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LoadBalancerStatus {
                    ingress: value_ingress.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "LoadBalancerStatus",
            &[
                "ingress",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LoadBalancerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LoadBalancerStatus",
            usize::from(!self.ingress.is_empty()),
        )?;
        if !self.ingress.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", &self.ingress)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for LoadBalancerStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "LoadBalancerStatus represents the status of a load-balancer.",
          "properties": {
            "ingress": {
              "description": "Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.",
              "items": crate::api::core::v1::LoadBalancerIngress::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
