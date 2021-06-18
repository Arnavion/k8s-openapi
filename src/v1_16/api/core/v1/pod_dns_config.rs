// Generated from definition io.k8s.api.core.v1.PodDNSConfig

/// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub nameservers: Vec<String>,

    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::PodDNSConfigOption>::new"))]
    pub options: Vec<crate::api::core::v1::PodDNSConfigOption>,

    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub searches: Vec<String>,
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodDNSConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_nameservers: Option<Vec<String>> = None;
                let mut value_options: Option<Vec<crate::api::core::v1::PodDNSConfigOption>> = None;
                let mut value_searches: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_nameservers => value_nameservers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_searches => value_searches = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDNSConfig {
                    nameservers: value_nameservers.unwrap_or_default(),
                    options: value_options.unwrap_or_default(),
                    searches: value_searches.unwrap_or_default(),
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
            usize::from(!self.nameservers.is_empty()) +
            usize::from(!self.options.is_empty()) +
            usize::from(!self.searches.is_empty()),
        )?;
        if !self.nameservers.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nameservers", &self.nameservers)?;
        }
        if !self.options.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", &self.options)?;
        }
        if !self.searches.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "searches", &self.searches)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
