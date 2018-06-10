// Generated from definition io.k8s.api.core.v1.NodeAddress

/// NodeAddress contains information for the node's address.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeAddress {
    /// The node address.
    pub address: String,

    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for NodeAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address,
            Key_type_,
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
                            "address" => Field::Key_address,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NodeAddress;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeAddress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_address: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address => value_address = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeAddress {
                    address: value_address.ok_or_else(|| ::serde::de::Error::missing_field("address"))?,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeAddress",
            &[
                "address",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeAddress",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "address", &self.address)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
