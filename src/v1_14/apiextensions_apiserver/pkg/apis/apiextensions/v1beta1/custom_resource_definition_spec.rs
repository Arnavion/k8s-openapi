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

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_additional_printer_columns,
            Key_conversion,
            Key_group,
            Key_names,
            Key_scope,
            Key_subresources,
            Key_validation,
            Key_version,
            Key_versions,
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
                            "conversion" => Field::Key_conversion,
                            "group" => Field::Key_group,
                            "names" => Field::Key_names,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>> = None;
                let mut value_conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion> = None;
                let mut value_group: Option<String> = None;
                let mut value_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames> = None;
                let mut value_scope: Option<String> = None;
                let mut value_subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources> = None;
                let mut value_validation: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation> = None;
                let mut value_version: Option<String> = None;
                let mut value_versions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_printer_columns => value_additional_printer_columns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conversion => value_conversion = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_names => value_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subresources => value_subresources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validation => value_validation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionSpec {
                    additional_printer_columns: value_additional_printer_columns,
                    conversion: value_conversion,
                    group: value_group.unwrap_or_default(),
                    names: value_names.unwrap_or_default(),
                    scope: value_scope.unwrap_or_default(),
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

impl crate::serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionSpec",
            3 +
            self.additional_printer_columns.as_ref().map_or(0, |_| 1) +
            self.conversion.as_ref().map_or(0, |_| 1) +
            self.subresources.as_ref().map_or(0, |_| 1) +
            self.validation.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1) +
            self.versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.additional_printer_columns {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", value)?;
        }
        if let Some(value) = &self.conversion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scope", &self.scope)?;
        if let Some(value) = &self.subresources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        if let Some(value) = &self.validation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validation", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        if let Some(value) = &self.versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceDefinitionSpec {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CustomResourceDefinitionSpec describes how a user wants their resource to appear".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: IntoIterator::into_iter([
                    (
                        "additionalPrinterColumns".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AdditionalPrinterColumns are additional columns shown e.g. in kubectl next to the name. Defaults to a created-at column. Optional, the global columns for all versions. Top-level and per-version columns are mutually exclusive.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conversion".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`conversion` defines conversion settings for the CRD.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Group is the group this resource belongs in".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "names".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Names are the names used to describe this custom resource".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scope".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subresources".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Subresources describes the subresources for CustomResource Optional, the global subresources for all versions. Top-level and per-version subresources are mutually exclusive.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "validation".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Validation describes the validation methods for CustomResources Optional, the global validation schema for all versions. Top-level and per-version schemas are mutually exclusive.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "version".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Version is the version this resource belongs in Should be always first item in Versions field if provided. Optional, but at least one of Version or Versions must be set. Deprecated: Please use `Versions`.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "versions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Versions is the list of all supported versions for this resource. If Version field is provided, this field is optional. Validation: All versions must use the same validation schema for now. i.e., top level Validation field is applied to all of these versions. Order: The version name will be used to compute the order. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ]).collect(),
                required: IntoIterator::into_iter([
                    "group",
                    "names",
                    "scope",
                ]).map(std::borrow::ToOwned::to_owned).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
