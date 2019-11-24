// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionSpec

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionSpec {
    /// AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. Optional, the global columns for all versions. Top-level and per-version columns are mutually exclusive.
    pub additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>>,

    /// `conversion` defines conversion settings for the CRD.
    pub conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion>,

    /// Group is the group this resource belongs in
    pub group: String,

    /// Names are the names used to describe this custom resource
    pub names: crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames,

    /// preserveUnknownFields disables pruning of object fields which are not specified in the OpenAPI schema. apiVersion, kind, metadata and known fields inside metadata are always preserved. Defaults to true in v1beta and will default to false in v1.
    pub preserve_unknown_fields: Option<bool>,

    /// Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced
    pub scope: String,

    /// Subresources describes the subresources for CustomResource Optional, the global subresources for all versions. Top-level and per-version subresources are mutually exclusive.
    pub subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>,

    /// Validation describes the validation methods for CustomResources Optional, the global validation schema for all versions. Top-level and per-version schemas are mutually exclusive.
    pub validation: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>,

    /// Version is the version this resource belongs in Should be always first item in Versions field if provided. Optional, but at least one of Version or Versions must be set. Deprecated: Please use `Versions`.
    pub version: Option<String>,

    /// Versions is the list of all supported versions for this resource. If Version field is provided, this field is optional. Validation: All versions must use the same validation schema for now. i.e., top level Validation field is applied to all of these versions. Order: The version name will be used to compute the order. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    pub versions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>>,
}

impl<'de> serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_additional_printer_columns,
            Key_conversion,
            Key_group,
            Key_names,
            Key_preserve_unknown_fields,
            Key_scope,
            Key_subresources,
            Key_validation,
            Key_version,
            Key_versions,
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
                            "conversion" => Field::Key_conversion,
                            "group" => Field::Key_group,
                            "names" => Field::Key_names,
                            "preserveUnknownFields" => Field::Key_preserve_unknown_fields,
                            "scope" => Field::Key_scope,
                            "subresources" => Field::Key_subresources,
                            "validation" => Field::Key_validation,
                            "version" => Field::Key_version,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>> = None;
                let mut value_conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion> = None;
                let mut value_group: Option<String> = None;
                let mut value_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames> = None;
                let mut value_preserve_unknown_fields: Option<bool> = None;
                let mut value_scope: Option<String> = None;
                let mut value_subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources> = None;
                let mut value_validation: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation> = None;
                let mut value_version: Option<String> = None;
                let mut value_versions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_printer_columns => value_additional_printer_columns = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conversion => value_conversion = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_names => value_names = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_preserve_unknown_fields => value_preserve_unknown_fields = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subresources => value_subresources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validation => value_validation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionSpec {
                    additional_printer_columns: value_additional_printer_columns,
                    conversion: value_conversion,
                    group: value_group.ok_or_else(|| serde::de::Error::missing_field("group"))?,
                    names: value_names.ok_or_else(|| serde::de::Error::missing_field("names"))?,
                    preserve_unknown_fields: value_preserve_unknown_fields,
                    scope: value_scope.ok_or_else(|| serde::de::Error::missing_field("scope"))?,
                    subresources: value_subresources,
                    validation: value_validation,
                    version: value_version,
                    versions: value_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionSpec",
            &[
                "additionalPrinterColumns",
                "conversion",
                "group",
                "names",
                "preserveUnknownFields",
                "scope",
                "subresources",
                "validation",
                "version",
                "versions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionSpec",
            3 +
            self.additional_printer_columns.as_ref().map_or(0, |_| 1) +
            self.conversion.as_ref().map_or(0, |_| 1) +
            self.preserve_unknown_fields.as_ref().map_or(0, |_| 1) +
            self.subresources.as_ref().map_or(0, |_| 1) +
            self.validation.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1) +
            self.versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.additional_printer_columns {
            serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", value)?;
        }
        if let Some(value) = &self.conversion {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conversion", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        if let Some(value) = &self.preserve_unknown_fields {
            serde::ser::SerializeStruct::serialize_field(&mut state, "preserveUnknownFields", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "scope", &self.scope)?;
        if let Some(value) = &self.subresources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        if let Some(value) = &self.validation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "validation", value)?;
        }
        if let Some(value) = &self.version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        if let Some(value) = &self.versions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "versions", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
