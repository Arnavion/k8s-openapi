// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

/// APIGroup contains the name, the supported versions, and the preferred version of a group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIGroup {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// name is the name of the group.
    pub name: String,

    /// preferredVersion is the version preferred by the API server, which probably is the storage version.
    pub preferred_version: Option<::v1_12::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,

    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    pub server_address_by_client_cidrs: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>>,

    /// versions are the versions supported in this group.
    pub versions: Vec<::v1_12::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,
}

impl<'de> ::serde::Deserialize<'de> for APIGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_preferred_version,
            Key_server_address_by_client_cidrs,
            Key_versions,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "preferredVersion" => Field::Key_preferred_version,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIGroup;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIGroup")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_preferred_version: Option<::v1_12::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery> = None;
                let mut value_server_address_by_client_cidrs: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>> = None;
                let mut value_versions: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_preferred_version => value_preferred_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_address_by_client_cidrs => value_server_address_by_client_cidrs = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIGroup {
                    api_version: value_api_version,
                    kind: value_kind,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    preferred_version: value_preferred_version,
                    server_address_by_client_cidrs: value_server_address_by_client_cidrs,
                    versions: value_versions.ok_or_else(|| ::serde::de::Error::missing_field("versions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIGroup",
            &[
                "apiVersion",
                "kind",
                "name",
                "preferredVersion",
                "serverAddressByClientCIDRs",
                "versions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for APIGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIGroup",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            1 +
            self.preferred_version.as_ref().map_or(0, |_| 1) +
            self.server_address_by_client_cidrs.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.preferred_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "preferredVersion", value)?;
        }
        if let Some(value) = &self.server_address_by_client_cidrs {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "serverAddressByClientCIDRs", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
