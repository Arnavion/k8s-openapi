// Generated from definition io.k8s.api.networking.v1alpha1.ServiceCIDRSpec

/// ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceCIDRSpec {
    /// CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64") from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family. This field is immutable.
    pub cidrs: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for ServiceCIDRSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.cidrs, other.cidrs);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServiceCIDRSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidrs,
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
                            "cidrs" => Field::Key_cidrs,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceCIDRSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ServiceCIDRSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cidrs: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidrs => value_cidrs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceCIDRSpec {
                    cidrs: value_cidrs,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceCIDRSpec",
            &[
                "cidrs",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceCIDRSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceCIDRSpec",
            self.cidrs.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cidrs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cidrs", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceCIDRSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1alpha1.ServiceCIDRSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.",
            "type": "object",
            "properties": {
                "cidrs": {
                    "description": "CIDRs defines the IP blocks in CIDR notation (e.g. \"192.168.0.0/24\" or \"2001:db8::/64\") from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family. This field is immutable.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
