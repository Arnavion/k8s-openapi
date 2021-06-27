// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIServiceStatus

/// APIServiceStatus contains derived information about an API server
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIServiceStatus {
    /// Current service state of apiService.
    pub conditions: Vec<crate::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition>,
}

impl<'de> crate::serde::Deserialize<'de> for APIServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = APIServiceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("APIServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIServiceStatus {
                    conditions: value_conditions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "APIServiceStatus",
            &[
                "conditions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for APIServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIServiceStatus",
            usize::from(!self.conditions.is_empty()),
        )?;
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl APIServiceStatus {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "APIServiceStatus contains derived information about an API server",
          "properties": {
            "conditions": {
              "description": "Current service state of apiService.",
              "items": crate::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceCondition::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
