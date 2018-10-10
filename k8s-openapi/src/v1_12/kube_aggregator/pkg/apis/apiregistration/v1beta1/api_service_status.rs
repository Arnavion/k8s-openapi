// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceStatus

/// APIServiceStatus contains derived information about an API server
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIServiceStatus {
    /// Current service state of apiService.
    pub conditions: Option<Vec<::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceCondition>>,
}

impl<'de> ::serde::Deserialize<'de> for APIServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
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
                            "conditions" => Field::Key_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIServiceStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<::v1_12::kube_aggregator::pkg::apis::apiregistration::v1beta1::APIServiceCondition>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIServiceStatus {
                    conditions: value_conditions,
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

impl ::serde::Serialize for APIServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIServiceStatus",
            0 +
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
