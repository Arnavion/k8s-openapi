// Generated from definition io.k8s.api.core.v1.Taint

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Taint {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,

    /// Required. The taint key to be applied to a node.
    pub key: String,

    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    pub time_added: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// Required. The taint value corresponding to the taint key.
    pub value: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Taint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_time_added,
            Key_value,
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
                            "effect" => Field::Key_effect,
                            "key" => Field::Key_key,
                            "timeAdded" => Field::Key_time_added,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Taint;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Taint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_effect: Option<String> = None;
                let mut value_key: Option<String> = None;
                let mut value_time_added: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_key => value_key = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_time_added => value_time_added = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Taint {
                    effect: value_effect.ok_or_else(|| ::serde::de::Error::missing_field("effect"))?,
                    key: value_key.ok_or_else(|| ::serde::de::Error::missing_field("key"))?,
                    time_added: value_time_added,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "Taint",
            &[
                "effect",
                "key",
                "timeAdded",
                "value",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Taint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Taint",
            0 +
            1 +
            1 +
            self.time_added.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", &self.effect)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.time_added {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "timeAdded", value)?;
        }
        if let Some(value) = &self.value {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
