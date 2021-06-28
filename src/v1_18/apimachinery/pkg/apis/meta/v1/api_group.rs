// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

/// APIGroup contains the name, the supported versions, and the preferred version of a group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIGroup {
    /// name is the name of the group.
    pub name: String,

    /// preferredVersion is the version preferred by the API server, which probably is the storage version.
    pub preferred_version: Option<crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,

    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    pub server_address_by_client_cidrs: Vec<crate::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>,

    /// versions are the versions supported in this group.
    pub versions: Vec<crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,
}

impl crate::Resource for APIGroup {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "APIGroup";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = crate::ClusterResourceScope;
}

impl<'de> crate::serde::Deserialize<'de> for APIGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = APIGroup;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_preferred_version: Option<crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery> = None;
                let mut value_server_address_by_client_cidrs: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>> = None;
                let mut value_versions: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_preferred_version => value_preferred_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_address_by_client_cidrs => value_server_address_by_client_cidrs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIGroup {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    preferred_version: value_preferred_version,
                    server_address_by_client_cidrs: value_server_address_by_client_cidrs.unwrap_or_default(),
                    versions: value_versions.ok_or_else(|| crate::serde::de::Error::missing_field("versions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
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

impl crate::serde::Serialize for APIGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.preferred_version.as_ref().map_or(0, |_| 1) +
            usize::from(!self.server_address_by_client_cidrs.is_empty()),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.preferred_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preferredVersion", value)?;
        }
        if !self.server_address_by_client_cidrs.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serverAddressByClientCIDRs", &self.server_address_by_client_cidrs)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for APIGroup {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "APIGroup contains the name, the supported versions, and the preferred version of a group.",
          "x-kubernetes-group-version-kind": [
            {
              "group": "",
              "kind": "APIGroup",
              "version": "v1"
            }
          ],
          "properties": {
            "apiVersion": {
              "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
              "type": "string"
            },
            "kind": {
              "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
              "type": "string"
            },
            "name": {
              "description": "name is the name of the group.",
              "type": "string"
            },
            "preferredVersion": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery::schema(), "preferredVersion is the version preferred by the API server, which probably is the storage version."),
            "serverAddressByClientCIDRs": {
              "description": "a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.",
              "items": crate::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR::schema(),
              "type": "array"
            },
            "versions": {
              "description": "versions are the versions supported in this group.",
              "items": crate::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery::schema(),
              "type": "array"
            }
          },
          "required": [
            "name",
            "versions"
          ],
          "type": "object"
        })
    }
}
