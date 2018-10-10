// Generated from definition io.k8s.api.core.v1.LoadBalancerStatus

/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    pub ingress: Option<Vec<::v1_12::api::core::v1::LoadBalancerIngress>>,
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ingress,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct LoadBalancerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_ingress: Option<Vec<::v1_12::api::core::v1::LoadBalancerIngress>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for LoadBalancerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LoadBalancerStatus",
            0 +
            self.ingress.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ingress {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
