// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions

/// APIVersions lists the versions that are available, to allow clients to discover the API at /api, which is the root path of the legacy v1 API.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIVersions {
    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    pub server_address_by_client_cidrs: Vec<crate::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>,

    /// versions are the api versions that are available.
    pub versions: Vec<String>,
}

impl crate::Resource for APIVersions {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "APIVersions";
    const VERSION: &'static str = "v1";
}

impl<'de> serde::Deserialize<'de> for APIVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_server_address_by_client_cidrs,
            Key_versions,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "serverAddressByClientCIDRs" => Field::Key_server_address_by_client_cidrs,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = APIVersions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_server_address_by_client_cidrs: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>> = None;
                let mut value_versions: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_server_address_by_client_cidrs => value_server_address_by_client_cidrs = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_versions => value_versions = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIVersions {
                    server_address_by_client_cidrs: value_server_address_by_client_cidrs.ok_or_else(|| serde::de::Error::missing_field("serverAddressByClientCIDRs"))?,
                    versions: value_versions.ok_or_else(|| serde::de::Error::missing_field("versions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "serverAddressByClientCIDRs",
                "versions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for APIVersions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "serverAddressByClientCIDRs", &self.server_address_by_client_cidrs)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        serde::ser::SerializeStruct::end(state)
    }
}
