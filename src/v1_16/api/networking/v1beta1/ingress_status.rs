// Generated from definition io.k8s.api.networking.v1beta1.IngressStatus

/// IngressStatus describe the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    pub load_balancer: Option<crate::api::core::v1::LoadBalancerStatus>,
}

impl<'de> serde::Deserialize<'de> for IngressStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
            type Value = IngressStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_load_balancer: Option<crate::api::core::v1::LoadBalancerStatus> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_load_balancer => value_load_balancer = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressStatus {
                    load_balancer: value_load_balancer,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressStatus",
            &[
                "loadBalancer",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IngressStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressStatus",
            self.load_balancer.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.load_balancer {
            serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancer", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
