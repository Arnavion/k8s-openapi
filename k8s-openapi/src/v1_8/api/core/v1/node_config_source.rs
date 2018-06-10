// Generated from definition io.k8s.api.core.v1.NodeConfigSource

/// NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeConfigSource {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    pub config_map_ref: Option<::v1_8::api::core::v1::ObjectReference>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for NodeConfigSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_config_map_ref,
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
                            "configMapRef" => Field::Key_config_map_ref,
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
            type Value = NodeConfigSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct NodeConfigSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_config_map_ref: Option<::v1_8::api::core::v1::ObjectReference> = None;
                let mut value_kind: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_map_ref => value_config_map_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeConfigSource {
                    api_version: value_api_version,
                    config_map_ref: value_config_map_ref,
                    kind: value_kind,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeConfigSource",
            &[
                "apiVersion",
                "configMapRef",
                "kind",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for NodeConfigSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeConfigSource",
            0 +
            (if self.api_version.is_some() { 1 } else { 0 }) +
            (if self.config_map_ref.is_some() { 1 } else { 0 }) +
            (if self.kind.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.config_map_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "configMapRef", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
