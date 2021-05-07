// Generated from definition io.k8s.api.core.v1.ResourceQuotaStatus

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    pub hard: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// Used is the current observed total usage of the resource in the namespace.
    pub used: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceQuotaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hard,
            Key_used,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceQuotaStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hard: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_used: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hard => value_hard = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_used => value_used = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for ResourceQuotaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaStatus",
            self.hard.as_ref().map_or(0, |_| 1) +
            self.used.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hard {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hard", value)?;
        }
        if let Some(value) = &self.used {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "used", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
