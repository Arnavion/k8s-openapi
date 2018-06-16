// Generated from definition io.k8s.api.core.v1.Toleration

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Toleration {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: Option<String>,

    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    pub key: Option<String>,

    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    pub operator: Option<String>,

    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    pub toleration_seconds: Option<i64>,

    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    pub value: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for Toleration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_operator,
            Key_toleration_seconds,
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
                            "operator" => Field::Key_operator,
                            "tolerationSeconds" => Field::Key_toleration_seconds,
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
            type Value = Toleration;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Toleration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_effect: Option<String> = None;
                let mut value_key: Option<String> = None;
                let mut value_operator: Option<String> = None;
                let mut value_toleration_seconds: Option<i64> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operator => value_operator = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_toleration_seconds => value_toleration_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Toleration {
                    effect: value_effect,
                    key: value_key,
                    operator: value_operator,
                    toleration_seconds: value_toleration_seconds,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "Toleration",
            &[
                "effect",
                "key",
                "operator",
                "tolerationSeconds",
                "value",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Toleration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Toleration",
            0 +
            self.effect.as_ref().map_or(0, |_| 1) +
            self.key.as_ref().map_or(0, |_| 1) +
            self.operator.as_ref().map_or(0, |_| 1) +
            self.toleration_seconds.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.effect {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", value)?;
        }
        if let Some(value) = &self.key {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        if let Some(value) = &self.operator {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", value)?;
        }
        if let Some(value) = &self.toleration_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerationSeconds", value)?;
        }
        if let Some(value) = &self.value {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
