// Generated from definition io.k8s.api.core.v1.ResourceFieldSelector

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    pub container_name: Option<String>,

    /// Specifies the output format of the exposed resources, defaults to "1"
    pub divisor: Option<crate::apimachinery::pkg::api::resource::Quantity>,

    /// Required: resource to select
    pub resource: String,
}

impl<'de> crate::serde::Deserialize<'de> for ResourceFieldSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_name,
            Key_divisor,
            Key_resource,
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
                            "containerName" => Field::Key_container_name,
                            "divisor" => Field::Key_divisor,
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
            type Value = ResourceFieldSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ResourceFieldSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_name: Option<String> = None;
                let mut value_divisor: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_resource: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_name => value_container_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_divisor => value_divisor = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceFieldSelector {
                    container_name: value_container_name,
                    divisor: value_divisor,
                    resource: value_resource.ok_or_else(|| crate::serde::de::Error::missing_field("resource"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceFieldSelector",
            &[
                "containerName",
                "divisor",
                "resource",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceFieldSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceFieldSelector",
            1 +
            self.container_name.as_ref().map_or(0, |_| 1) +
            self.divisor.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", value)?;
        }
        if let Some(value) = &self.divisor {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "divisor", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ResourceFieldSelector {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ResourceFieldSelector represents container resources (cpu, memory) and their output format",
          "properties": {
            "containerName": {
              "description": "Container name: required for volumes, optional for env vars",
              "type": "string"
            },
            "divisor": crate::schema_ref_with_values(crate::apimachinery::pkg::api::resource::Quantity::schema(), serde_json::json!({"description": "Specifies the output format of the exposed resources, defaults to \"1\""})),
            "resource": {
              "description": "Required: resource to select",
              "type": "string"
            }
          },
          "required": [
            "resource"
          ],
          "type": "object"
        })
    }
}
