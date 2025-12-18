// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.GroupResource

/// GroupResource specifies a Group and a Resource, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupResource {
    pub group: std::string::String,

    pub resource: std::string::String,
}

impl crate::DeepMerge for GroupResource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.group, other.group);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
    }
}

impl<'de> crate::serde::Deserialize<'de> for GroupResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_resource,
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
                            "group" => Field::Key_group,
                            "resource" => Field::Key_resource,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = GroupResource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("GroupResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group: Option<std::string::String> = None;
                let mut value_resource: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GroupResource {
                    group: value_group.unwrap_or_default(),
                    resource: value_resource.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "GroupResource",
            &[
                "group",
                "resource",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for GroupResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GroupResource",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GroupResource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.GroupResource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "GroupResource specifies a Group and a Resource, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types",
            "type": "object",
            "properties": {
                "group": {
                    "type": "string",
                },
                "resource": {
                    "type": "string",
                },
            },
            "required": [
                "group",
                "resource",
            ],
        })
    }
}
