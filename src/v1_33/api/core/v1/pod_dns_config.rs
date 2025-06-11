// Generated from definition io.k8s.api.core.v1.PodDNSConfig

/// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    pub nameservers: Option<std::vec::Vec<std::string::String>>,

    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    pub options: Option<std::vec::Vec<crate::api::core::v1::PodDNSConfigOption>>,

    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    pub searches: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for PodDNSConfig {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.nameservers, other.nameservers);
        crate::merge_strategies::list::atomic(&mut self.options, other.options);
        crate::merge_strategies::list::atomic(&mut self.searches, other.searches);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodDNSConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_nameservers,
            Key_options,
            Key_searches,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodDNSConfig;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodDNSConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_nameservers: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_options: Option<std::vec::Vec<crate::api::core::v1::PodDNSConfigOption>> = None;
                let mut value_searches: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_nameservers => value_nameservers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_searches => value_searches = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for PodDNSConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDNSConfig",
            self.nameservers.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.searches.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.nameservers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nameservers", value)?;
        }
        if let Some(value) = &self.options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.searches {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "searches", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodDNSConfig {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.PodDNSConfig".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.",
            "type": "object",
            "properties": {
                "nameservers": {
                    "description": "A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "options": {
                    "description": "A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::PodDNSConfigOption>()),
                },
                "searches": {
                    "description": "A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
