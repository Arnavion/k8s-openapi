// Generated from definition io.k8s.api.core.v1.ResourceRequirements

/// ResourceRequirements describes the compute resource requirements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    pub limits: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,

    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    pub requests: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,
}

impl<'de> ::serde::Deserialize<'de> for ResourceRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_limits,
            Key_requests,
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
                            "limits" => Field::Key_limits,
                            "requests" => Field::Key_requests,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceRequirements;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ResourceRequirements")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_limits: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_requests: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_limits => value_limits = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceRequirements {
                    limits: value_limits,
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceRequirements",
            &[
                "limits",
                "requests",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ResourceRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceRequirements",
            0 +
            self.limits.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.limits {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "limits", value)?;
        }
        if let Some(value) = &self.requests {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
