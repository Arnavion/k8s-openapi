// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroupList

/// APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIGroupList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// groups is a list of APIGroup.
    pub groups: Vec<::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for APIGroupList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_groups,
            Key_kind,
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
                            "groups" => Field::Key_groups,
                            "kind" => Field::Key_kind,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIGroupList;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIGroupList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_groups: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::APIGroup>> = None;
                let mut value_kind: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIGroupList {
                    api_version: value_api_version,
                    groups: value_groups.ok_or_else(|| ::serde::de::Error::missing_field("groups"))?,
                    kind: value_kind,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIGroupList",
            &[
                "apiVersion",
                "groups",
                "kind",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for APIGroupList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIGroupList",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            1 +
            self.kind.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
