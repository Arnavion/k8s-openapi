// Generated from definition io.k8s.api.core.v1.PodDNSConfig

/// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    pub nameservers: Option<Vec<String>>,

    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    pub options: Option<Vec<::v1_12::api::core::v1::PodDNSConfigOption>>,

    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    pub searches: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for PodDNSConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_nameservers,
            Key_options,
            Key_searches,
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
                            "nameservers" => Field::Key_nameservers,
                            "options" => Field::Key_options,
                            "searches" => Field::Key_searches,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodDNSConfig;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodDNSConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_nameservers: Option<Vec<String>> = None;
                let mut value_options: Option<Vec<::v1_12::api::core::v1::PodDNSConfigOption>> = None;
                let mut value_searches: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_nameservers => value_nameservers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_searches => value_searches = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDNSConfig {
                    nameservers: value_nameservers,
                    options: value_options,
                    searches: value_searches,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDNSConfig",
            &[
                "nameservers",
                "options",
                "searches",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodDNSConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDNSConfig",
            0 +
            self.nameservers.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.searches.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.nameservers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "nameservers", value)?;
        }
        if let Some(value) = &self.options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.searches {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "searches", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
