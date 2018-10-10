// Generated from definition io.k8s.api.core.v1.ResourceQuotaStatus

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    pub hard: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,

    /// Used is the current observed total usage of the resource in the namespace.
    pub used: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>>,
}

impl<'de> ::serde::Deserialize<'de> for ResourceQuotaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hard,
            Key_used,
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
                            "hard" => Field::Key_hard,
                            "used" => Field::Key_used,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ResourceQuotaStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_hard: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_used: Option<::std::collections::BTreeMap<String, ::v1_12::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hard => value_hard = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_used => value_used = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceQuotaStatus {
                    hard: value_hard,
                    used: value_used,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceQuotaStatus",
            &[
                "hard",
                "used",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ResourceQuotaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaStatus",
            0 +
            self.hard.as_ref().map_or(0, |_| 1) +
            self.used.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hard {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "hard", value)?;
        }
        if let Some(value) = &self.used {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "used", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
