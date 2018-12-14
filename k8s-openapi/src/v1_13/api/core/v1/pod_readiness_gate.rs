// Generated from definition io.k8s.api.core.v1.PodReadinessGate

/// PodReadinessGate contains the reference to a pod condition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodReadinessGate {
    /// ConditionType refers to a condition in the pod's condition list with matching type.
    pub condition_type: String,
}

impl<'de> ::serde::Deserialize<'de> for PodReadinessGate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_condition_type,
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
                            "conditionType" => Field::Key_condition_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodReadinessGate;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodReadinessGate")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_condition_type: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_condition_type => value_condition_type = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodReadinessGate {
                    condition_type: value_condition_type.ok_or_else(|| ::serde::de::Error::missing_field("conditionType"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodReadinessGate",
            &[
                "conditionType",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodReadinessGate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodReadinessGate",
            0 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditionType", &self.condition_type)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
