// Generated from definition io.k8s.api.extensions.v1beta1.IPBlock

/// DEPRECATED 1.9 - This group version of IPBlock is deprecated by networking/v1/IPBlock. IPBlock describes a particular CIDR (Ex. "192.168.1.1/24") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IPBlock {
    /// CIDR is a string representing the IP Block Valid examples are "192.168.1.1/24"
    pub cidr: String,

    /// Except is a slice of CIDRs that should not be included within an IP Block Valid examples are "192.168.1.1/24" Except values will be rejected if they are outside the CIDR range
    pub except: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for IPBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr,
            Key_except,
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
                            "cidr" => Field::Key_cidr,
                            "except" => Field::Key_except,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IPBlock;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct IPBlock")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_cidr: Option<String> = None;
                let mut value_except: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr => value_cidr = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_except => value_except = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IPBlock {
                    cidr: value_cidr.ok_or_else(|| ::serde::de::Error::missing_field("cidr"))?,
                    except: value_except,
                })
            }
        }

        deserializer.deserialize_struct(
            "IPBlock",
            &[
                "cidr",
                "except",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for IPBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IPBlock",
            0 +
            1 +
            self.except.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "cidr", &self.cidr)?;
        if let Some(value) = &self.except {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "except", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
