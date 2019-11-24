// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceColumnDefinition

/// CustomResourceColumnDefinition specifies a column for server side printing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceColumnDefinition {
    /// JSONPath is a simple JSON path (i.e. with array notation) which is evaluated against each custom resource to produce the value for this column.
    pub json_path: String,

    /// description is a human readable description of this column.
    pub description: Option<String>,

    /// format is an optional OpenAPI type definition for this column. The 'name' format is applied to the primary identifier column to assist in clients identifying column is the resource name. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
    pub format: Option<String>,

    /// name is a human readable name for the column.
    pub name: String,

    /// priority is an integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a priority greater than 0.
    pub priority: Option<i32>,

    /// type is an OpenAPI type definition for this column. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
    pub type_: String,
}

impl<'de> serde::Deserialize<'de> for CustomResourceColumnDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_json_path,
            Key_description,
            Key_format,
            Key_name,
            Key_priority,
            Key_type_,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "JSONPath" => Field::Key_json_path,
                            "description" => Field::Key_description,
                            "format" => Field::Key_format,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceColumnDefinition;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceColumnDefinition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_json_path: Option<String> = None;
                let mut value_description: Option<String> = None;
                let mut value_format: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_json_path => value_json_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_description => value_description = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_format => value_format = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_priority => value_priority = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceColumnDefinition {
                    json_path: value_json_path.ok_or_else(|| serde::de::Error::missing_field("JSONPath"))?,
                    description: value_description,
                    format: value_format,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    priority: value_priority,
                    type_: value_type_.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceColumnDefinition",
            &[
                "JSONPath",
                "description",
                "format",
                "name",
                "priority",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomResourceColumnDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceColumnDefinition",
            3 +
            self.description.as_ref().map_or(0, |_| 1) +
            self.format.as_ref().map_or(0, |_| 1) +
            self.priority.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "JSONPath", &self.json_path)?;
        if let Some(value) = &self.description {
            serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.format {
            serde::ser::SerializeStruct::serialize_field(&mut state, "format", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.priority {
            serde::ser::SerializeStruct::serialize_field(&mut state, "priority", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        serde::ser::SerializeStruct::end(state)
    }
}
