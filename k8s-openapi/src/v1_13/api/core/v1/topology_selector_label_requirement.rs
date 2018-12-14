// Generated from definition io.k8s.api.core.v1.TopologySelectorLabelRequirement

/// A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySelectorLabelRequirement {
    /// The label key that the selector applies to.
    pub key: String,

    /// An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
    pub values: Vec<String>,
}

impl<'de> ::serde::Deserialize<'de> for TopologySelectorLabelRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
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
            type Value = TopologySelectorLabelRequirement;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct TopologySelectorLabelRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_values => value_values = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TopologySelectorLabelRequirement {
                    key: value_key.ok_or_else(|| ::serde::de::Error::missing_field("key"))?,
                    values: value_values.ok_or_else(|| ::serde::de::Error::missing_field("values"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "TopologySelectorLabelRequirement",
            &[
                "key",
                "values",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for TopologySelectorLabelRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySelectorLabelRequirement",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "values", &self.values)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
