// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList

/// APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIResourceList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// groupVersion is the group and version this APIResourceList is for.
    pub group_version: String,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// resources contains the name of the resources and if they are namespaced.
    pub resources: Vec<::v1_12::apimachinery::pkg::apis::meta::v1::APIResource>,
}

impl<'de> ::serde::Deserialize<'de> for APIResourceList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_group_version,
            Key_kind,
            Key_resources,
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
                            "groupVersion" => Field::Key_group_version,
                            "kind" => Field::Key_kind,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIResourceList;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIResourceList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_group_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_resources: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::APIResource>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group_version => value_group_version = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIResourceList {
                    api_version: value_api_version,
                    group_version: value_group_version.ok_or_else(|| ::serde::de::Error::missing_field("groupVersion"))?,
                    kind: value_kind,
                    resources: value_resources.ok_or_else(|| ::serde::de::Error::missing_field("resources"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIResourceList",
            &[
                "apiVersion",
                "groupVersion",
                "kind",
                "resources",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for APIResourceList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIResourceList",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            1 +
            self.kind.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "groupVersion", &self.group_version)?;
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", &self.resources)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
