// Generated from definition io.k8s.api.scheduling.v1alpha1.TypedLocalObjectReference

/// TypedLocalObjectReference allows to reference typed object inside the same namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.
    pub api_group: Option<std::string::String>,

    /// Kind is the type of resource being referenced. It must be a path segment name.
    pub kind: std::string::String,

    /// Name is the name of resource being referenced. It must be a path segment name.
    pub name: std::string::String,
}

impl crate::DeepMerge for TypedLocalObjectReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_group, other.api_group);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TypedLocalObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_group,
            Key_kind,
            Key_name,
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
                            "apiGroup" => Field::Key_api_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TypedLocalObjectReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TypedLocalObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_group: Option<std::string::String> = None;
                let mut value_kind: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_group => value_api_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TypedLocalObjectReference {
                    api_group: value_api_group,
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "TypedLocalObjectReference",
            &[
                "apiGroup",
                "kind",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TypedLocalObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TypedLocalObjectReference",
            2 +
            self.api_group.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroup", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TypedLocalObjectReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha1.TypedLocalObjectReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TypedLocalObjectReference allows to reference typed object inside the same namespace.",
            "type": "object",
            "properties": {
                "apiGroup": {
                    "description": "APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.",
                    "type": "string",
                },
                "kind": {
                    "description": "Kind is the type of resource being referenced. It must be a path segment name.",
                    "type": "string",
                },
                "name": {
                    "description": "Name is the name of resource being referenced. It must be a path segment name.",
                    "type": "string",
                },
            },
            "required": [
                "kind",
                "name",
            ],
        })
    }
}
