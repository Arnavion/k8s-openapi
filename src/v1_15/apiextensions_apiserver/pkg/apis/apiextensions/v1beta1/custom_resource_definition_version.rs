// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionVersion

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionVersion {
    /// AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. Top-level and per-version columns are mutually exclusive. Per-version columns must not all be set to identical values (top-level columns should be used instead) This field is alpha-level and is only honored by servers that enable the CustomResourceWebhookConversion feature. NOTE: CRDs created prior to 1.13 populated the top-level additionalPrinterColumns field by default. To apply an update that changes to per-version additionalPrinterColumns, the top-level additionalPrinterColumns field must be explicitly set to null
    pub additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>>,

    /// Name is the version name, e.g. “v1”, “v2beta1”, etc.
    pub name: String,

    /// Schema describes the schema for CustomResource used in validation, pruning, and defaulting. Top-level and per-version schemas are mutually exclusive. Per-version schemas must not all be set to identical values (top-level validation schema should be used instead) This field is alpha-level and is only honored by servers that enable the CustomResourceWebhookConversion feature.
    pub schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>,

    /// Served is a flag enabling/disabling this version from being served via REST APIs
    pub served: bool,

    /// Storage flags the version as storage version. There must be exactly one flagged as storage version.
    pub storage: bool,

    /// Subresources describes the subresources for CustomResource Top-level and per-version subresources are mutually exclusive. Per-version subresources must not all be set to identical values (top-level subresources should be used instead) This field is alpha-level and is only honored by servers that enable the CustomResourceWebhookConversion feature.
    pub subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>,
}

impl<'de> serde::Deserialize<'de> for CustomResourceDefinitionVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>> = None;
                let mut value_name: Option<String> = None;
                let mut value_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation> = None;
                let mut value_served: Option<bool> = None;
                let mut value_storage: Option<bool> = None;
                let mut value_subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_printer_columns => value_additional_printer_columns = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_schema => value_schema = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_served => value_served = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_storage => value_storage = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subresources => value_subresources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionVersion {
                    additional_printer_columns: value_additional_printer_columns,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    schema: value_schema,
                    served: value_served.ok_or_else(|| serde::de::Error::missing_field("served"))?,
                    storage: value_storage.ok_or_else(|| serde::de::Error::missing_field("storage"))?,
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

impl serde::Serialize for CustomResourceDefinitionVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionVersion",
            3 +
            self.additional_printer_columns.as_ref().map_or(0, |_| 1) +
            self.schema.as_ref().map_or(0, |_| 1) +
            self.subresources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.additional_printer_columns {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.schema {
            serde::ser::SerializeStruct::serialize_field(&mut state, "schema", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "served", &self.served)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "storage", &self.storage)?;
        if let Some(value) = &self.subresources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
