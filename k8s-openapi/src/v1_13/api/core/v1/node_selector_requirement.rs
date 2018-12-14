// Generated from definition io.k8s.api.core.v1.NodeSelectorRequirement

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSelectorRequirement {
    /// The label key that the selector applies to.
    pub key: String,

    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,

    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    pub values: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for NodeSelectorRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_operator,
            Key_values,
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
                            "key" => Field::Key_key,
                            "operator" => Field::Key_operator,
                            "values" => Field::Key_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSelectorRequirement;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeSelectorRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_operator: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_operator => value_operator = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_values => value_values = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSelectorRequirement {
                    key: value_key.ok_or_else(|| ::serde::de::Error::missing_field("key"))?,
                    operator: value_operator.ok_or_else(|| ::serde::de::Error::missing_field("operator"))?,
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSelectorRequirement",
            &[
                "key",
                "operator",
                "values",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeSelectorRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSelectorRequirement",
            0 +
            1 +
            1 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        if let Some(value) = &self.values {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
