// Generated from definition io.k8s.api.core.v1.ScopedResourceSelectorRequirement

/// A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopedResourceSelectorRequirement {
    /// Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
    pub operator: String,

    /// The name of the scope that the selector applies to.
    pub scope_name: String,

    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    pub values: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for ScopedResourceSelectorRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_operator,
            Key_scope_name,
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
                            "operator" => Field::Key_operator,
                            "scopeName" => Field::Key_scope_name,
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
            type Value = ScopedResourceSelectorRequirement;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ScopedResourceSelectorRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_operator: Option<String> = None;
                let mut value_scope_name: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_operator => value_operator = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_scope_name => value_scope_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_values => value_values = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScopedResourceSelectorRequirement {
                    operator: value_operator.ok_or_else(|| ::serde::de::Error::missing_field("operator"))?,
                    scope_name: value_scope_name.ok_or_else(|| ::serde::de::Error::missing_field("scopeName"))?,
                    values: value_values,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScopedResourceSelectorRequirement",
            &[
                "operator",
                "scopeName",
                "values",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ScopedResourceSelectorRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScopedResourceSelectorRequirement",
            0 +
            1 +
            1 +
            self.values.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", &self.operator)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "scopeName", &self.scope_name)?;
        if let Some(value) = &self.values {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "values", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
