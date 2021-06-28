// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionVersion

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionVersion {
    /// additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.
    pub additional_printer_columns: Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition>,

    /// name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/\<group\>/\<version\>/...` if `served` is true.
    pub name: String,

    /// schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
    pub schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation>,

    /// served is a flag enabling/disabling this version from being served via REST APIs
    pub served: bool,

    /// storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.
    pub storage: bool,

    /// subresources specify what subresources this version of the defined custom resource have.
    pub subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_additional_printer_columns,
            Key_name,
            Key_schema,
            Key_served,
            Key_storage,
            Key_subresources,
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
                            "additionalPrinterColumns" => Field::Key_additional_printer_columns,
                            "name" => Field::Key_name,
                            "schema" => Field::Key_schema,
                            "served" => Field::Key_served,
                            "storage" => Field::Key_storage,
                            "subresources" => Field::Key_subresources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition>> = None;
                let mut value_name: Option<String> = None;
                let mut value_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation> = None;
                let mut value_served: Option<bool> = None;
                let mut value_storage: Option<bool> = None;
                let mut value_subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_printer_columns => value_additional_printer_columns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_schema => value_schema = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_served => value_served = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_storage => value_storage = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subresources => value_subresources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionVersion {
                    additional_printer_columns: value_additional_printer_columns.unwrap_or_default(),
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    schema: value_schema,
                    served: value_served.ok_or_else(|| crate::serde::de::Error::missing_field("served"))?,
                    storage: value_storage.ok_or_else(|| crate::serde::de::Error::missing_field("storage"))?,
                    subresources: value_subresources,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionVersion",
            &[
                "additionalPrinterColumns",
                "name",
                "schema",
                "served",
                "storage",
                "subresources",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceDefinitionVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionVersion",
            3 +
            usize::from(!self.additional_printer_columns.is_empty()) +
            self.schema.as_ref().map_or(0, |_| 1) +
            self.subresources.as_ref().map_or(0, |_| 1),
        )?;
        if !self.additional_printer_columns.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", &self.additional_printer_columns)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.schema {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schema", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "served", &self.served)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storage", &self.storage)?;
        if let Some(value) = &self.subresources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CustomResourceDefinitionVersion {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CustomResourceDefinitionVersion describes a version for CRD.",
          "properties": {
            "additionalPrinterColumns": {
              "description": "additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.",
              "items": crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition::schema(),
              "type": "array"
            },
            "name": {
              "description": "name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.",
              "type": "string"
            },
            "schema": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation::schema(), "schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource."),
            "served": {
              "description": "served is a flag enabling/disabling this version from being served via REST APIs",
              "type": "boolean"
            },
            "storage": {
              "description": "storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.",
              "type": "boolean"
            },
            "subresources": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources::schema(), "subresources specify what subresources this version of the defined custom resource have.")
          },
          "required": [
            "name",
            "served",
            "storage"
          ],
          "type": "object"
        })
    }
}
