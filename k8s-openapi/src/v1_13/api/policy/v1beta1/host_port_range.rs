// Generated from definition io.k8s.api.policy.v1beta1.HostPortRange

/// HostPortRange defines a range of host ports that will be enabled by a policy for pods to use.  It requires both the start and end to be defined.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HostPortRange {
    /// max is the end of the range, inclusive.
    pub max: i32,

    /// min is the start of the range, inclusive.
    pub min: i32,
}

impl<'de> ::serde::Deserialize<'de> for HostPortRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max,
            Key_min,
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
                            "max" => Field::Key_max,
                            "min" => Field::Key_min,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HostPortRange;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct HostPortRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_max: Option<i32> = None;
                let mut value_min: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max => value_max = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_min => value_min = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HostPortRange {
                    max: value_max.ok_or_else(|| ::serde::de::Error::missing_field("max"))?,
                    min: value_min.ok_or_else(|| ::serde::de::Error::missing_field("min"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "HostPortRange",
            &[
                "max",
                "min",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for HostPortRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HostPortRange",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "max", &self.max)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "min", &self.min)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
