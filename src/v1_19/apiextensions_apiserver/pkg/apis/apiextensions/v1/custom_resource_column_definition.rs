// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceColumnDefinition

/// CustomResourceColumnDefinition specifies a column for server side printing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceColumnDefinition {
    /// description is a human readable description of this column.
    pub description: Option<String>,

    /// format is an optional OpenAPI type definition for this column. The 'name' format is applied to the primary identifier column to assist in clients identifying column is the resource name. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
    pub format: Option<String>,

    /// jsonPath is a simple JSON path (i.e. with array notation) which is evaluated against each custom resource to produce the value for this column.
    pub json_path: String,

    /// name is a human readable name for the column.
    pub name: String,

    /// priority is an integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a priority greater than 0.
    pub priority: Option<i32>,

    /// type is an OpenAPI type definition for this column. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceColumnDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_description,
            Key_format,
            Key_json_path,
            Key_name,
            Key_priority,
            Key_type_,
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
                            "description" => Field::Key_description,
                            "format" => Field::Key_format,
                            "jsonPath" => Field::Key_json_path,
                            "name" => Field::Key_name,
                            "priority" => Field::Key_priority,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceColumnDefinition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceColumnDefinition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_description: Option<String> = None;
                let mut value_format: Option<String> = None;
                let mut value_json_path: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_description => value_description = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_format => value_format = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_json_path => value_json_path = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceColumnDefinition {
                    description: value_description,
                    format: value_format,
                    json_path: value_json_path.ok_or_else(|| crate::serde::de::Error::missing_field("jsonPath"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    priority: value_priority,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceColumnDefinition",
            &[
                "description",
                "format",
                "jsonPath",
                "name",
                "priority",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceColumnDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceColumnDefinition",
            3 +
            self.description.as_ref().map_or(0, |_| 1) +
            self.format.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.description {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.format {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "format", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "jsonPath", &self.json_path)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.priority {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CustomResourceColumnDefinition {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CustomResourceColumnDefinition specifies a column for server side printing.",
          "properties": {
            "description": {
              "description": "description is a human readable description of this column.",
              "type": "string"
            },
            "format": {
              "description": "format is an optional OpenAPI type definition for this column. The 'name' format is applied to the primary identifier column to assist in clients identifying column is the resource name. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.",
              "type": "string"
            },
            "jsonPath": {
              "description": "jsonPath is a simple JSON path (i.e. with array notation) which is evaluated against each custom resource to produce the value for this column.",
              "type": "string"
            },
            "name": {
              "description": "name is a human readable name for the column.",
              "type": "string"
            },
            "priority": {
              "description": "priority is an integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a priority greater than 0.",
              "format": "int32",
              "type": "integer"
            },
            "type": {
              "description": "type is an OpenAPI type definition for this column. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.",
              "type": "string"
            }
          },
          "required": [
            "jsonPath",
            "name",
            "type"
          ],
          "type": "object"
        })
    }
}
