// Generated from definition io.k8s.api.core.v1.LoadBalancerStatus

/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    pub ingress: Option<std::vec::Vec<crate::api::core::v1::LoadBalancerIngress>>,
}

impl crate::DeepMerge for LoadBalancerStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.ingress, other.ingress);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LoadBalancerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ingress: Option<std::vec::Vec<crate::api::core::v1::LoadBalancerIngress>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LoadBalancerStatus {
                    ingress: value_ingress,
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
            self.ingress.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ingress {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LoadBalancerStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.LoadBalancerStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "LoadBalancerStatus represents the status of a load-balancer.",
            "type": "object",
            "properties": {
                "ingress": {
                    "description": "Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::LoadBalancerIngress>()),
                },
            },
        })
    }
}
