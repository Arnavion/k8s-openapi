// Generated from definition io.k8s.api.core.v1.ResourceStatus

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceStatus {
    /// Name of the resource. Must be unique within the pod and match one of the resources from the pod spec.
    pub name: std::string::String,

    /// List of unique Resources health. Each element in the list contains an unique resource ID and resource health. At a minimum, ResourceID must uniquely identify the Resource allocated to the Pod on the Node for the lifetime of a Pod. See ResourceID type for it's definition.
    pub resources: Option<std::vec::Vec<crate::api::core::v1::ResourceHealth>>,
}

impl crate::DeepMerge for ResourceStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::map(
            &mut self.resources,
            other.resources,
            &[|lhs, rhs| lhs.resource_id == rhs.resource_id],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_resources,
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
                            "name" => Field::Key_name,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_resources: Option<std::vec::Vec<crate::api::core::v1::ResourceHealth>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceStatus {
                    name: value_name.unwrap_or_default(),
                    resources: value_resources,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceStatus",
            &[
                "name",
                "resources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceStatus",
            1 +
            self.resources.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ResourceStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "type": "object",
            "properties": {
                "name": {
                    "description": "Name of the resource. Must be unique within the pod and match one of the resources from the pod spec.",
                    "type": "string",
                },
                "resources": {
                    "description": "List of unique Resources health. Each element in the list contains an unique resource ID and resource health. At a minimum, ResourceID must uniquely identify the Resource allocated to the Pod on the Node for the lifetime of a Pod. See ResourceID type for it's definition.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ResourceHealth>()),
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
