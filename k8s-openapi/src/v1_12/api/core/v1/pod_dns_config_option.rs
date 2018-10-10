// Generated from definition io.k8s.api.core.v1.PodDNSConfigOption

/// PodDNSConfigOption defines DNS resolver options of a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDNSConfigOption {
    /// Required.
    pub name: Option<String>,

    pub value: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for PodDNSConfigOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
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
                            "name" => Field::Key_name,
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
            type Value = PodDNSConfigOption;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodDNSConfigOption")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDNSConfigOption {
                    name: value_name,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDNSConfigOption",
            &[
                "name",
                "value",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodDNSConfigOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDNSConfigOption",
            0 +
            self.name.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.value {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
