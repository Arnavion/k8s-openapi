// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionSpec

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionSpec {
    /// additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If present, this field configures columns for all versions. Top-level and per-version columns are mutually exclusive. If no top-level or per-version columns are specified, a single column displaying the age of the custom resource is used.
    pub additional_printer_columns: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>>,

    /// conversion defines conversion settings for the CRD.
    pub conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion>,

    /// group is the API group of the defined custom resource. The custom resources are served under `/apis/\<group\>/...`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`).
    pub group: String,

    /// names specify the resource and kind names for the custom resource.
    pub names: crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames,

    /// preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. If false, schemas must be defined for all versions. Defaults to true in v1beta for backwards compatibility. Deprecated: will be required to be false in v1. Preservation of unknown fields can be specified in the validation schema using the `x-kubernetes-preserve-unknown-fields: true` extension. See https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/#pruning-versus-preserving-unknown-fields for details.
    pub preserve_unknown_fields: Option<bool>,

    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`. Default is `Namespaced`.
    pub scope: String,

    /// subresources specify what subresources the defined custom resource has. If present, this field configures subresources for all versions. Top-level and per-version subresources are mutually exclusive.
    pub subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>,

    /// validation describes the schema used for validation and pruning of the custom resource. If present, this validation schema is used to validate all versions. Top-level and per-version schemas are mutually exclusive.
    pub validation: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>,

    /// version is the API version of the defined custom resource. The custom resources are served under `/apis/\<group\>/\<version\>/...`. Must match the name of the first item in the `versions` list if `version` and `versions` are both specified. Optional if `versions` is specified. Deprecated: use `versions` instead.
    pub version: Option<String>,

    /// versions is the list of all API versions of the defined custom resource. Optional if `version` is specified. The name of the first item in the `versions` list must match the `version` field if `version` and `versions` are both specified. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    pub versions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>>,

}

#[cfg(feature = "dsl")]
impl CustomResourceDefinitionSpec  {
    /// Set [`Self::additional_printer_columns`]
    pub  fn additional_printer_columns_set(&mut self, additional_printer_columns: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>>>) -> &mut Self {
        self.additional_printer_columns = additional_printer_columns.into(); self
    }

    pub  fn additional_printer_columns(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition> {
        if self.additional_printer_columns.is_none() { self.additional_printer_columns = Some(Default::default()) }
        self.additional_printer_columns.as_mut().unwrap()
    }

    /// Modify [`Self::additional_printer_columns`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn additional_printer_columns_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition>)) -> &mut Self {
        if self.additional_printer_columns.is_none() { self.additional_printer_columns = Some(Default::default()) };
        func(self.additional_printer_columns.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::additional_printer_columns`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn additional_printer_columns_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition)) -> &mut Self {
        if self.additional_printer_columns.is_none() {
            self.additional_printer_columns = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.additional_printer_columns.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::additional_printer_columns`]
    pub  fn additional_printer_columns_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceColumnDefinition]>) -> &mut Self {
         if self.additional_printer_columns.is_none() { self.additional_printer_columns = Some(Vec::new()); }
         let additional_printer_columns = &mut self.additional_printer_columns.as_mut().unwrap();
         for item in other.borrow() {
             additional_printer_columns.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::conversion`]
    pub  fn conversion_set(&mut self, conversion: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion>>) -> &mut Self {
        self.conversion = conversion.into(); self
    }

    pub  fn conversion(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion {
        if self.conversion.is_none() { self.conversion = Some(Default::default()) }
        self.conversion.as_mut().unwrap()
    }

    /// Modify [`Self::conversion`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conversion_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceConversion)) -> &mut Self {
        if self.conversion.is_none() { self.conversion = Some(Default::default()) };
        func(self.conversion.as_mut().unwrap()); self
    }


    /// Set [`Self::group`]
    pub  fn group_set(&mut self, group: impl Into<String>) -> &mut Self {
        self.group = group.into(); self
    }

    pub  fn group(&mut self) -> &mut String {
        &mut self.group
    }

    /// Modify [`Self::group`] with a `func`
    pub  fn group_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.group); self
    }


    /// Set [`Self::names`]
    pub  fn names_set(&mut self, names: impl Into<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames>) -> &mut Self {
        self.names = names.into(); self
    }

    pub  fn names(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames {
        &mut self.names
    }

    /// Modify [`Self::names`] with a `func`
    pub  fn names_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames)) -> &mut Self {
        func(&mut self.names); self
    }


    /// Set [`Self::preserve_unknown_fields`]
    pub  fn preserve_unknown_fields_set(&mut self, preserve_unknown_fields: impl Into<Option<bool>>) -> &mut Self {
        self.preserve_unknown_fields = preserve_unknown_fields.into(); self
    }

    pub  fn preserve_unknown_fields(&mut self) -> &mut bool {
        if self.preserve_unknown_fields.is_none() { self.preserve_unknown_fields = Some(Default::default()) }
        self.preserve_unknown_fields.as_mut().unwrap()
    }

    /// Modify [`Self::preserve_unknown_fields`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn preserve_unknown_fields_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.preserve_unknown_fields.is_none() { self.preserve_unknown_fields = Some(Default::default()) };
        func(self.preserve_unknown_fields.as_mut().unwrap()); self
    }


    /// Set [`Self::scope`]
    pub  fn scope_set(&mut self, scope: impl Into<String>) -> &mut Self {
        self.scope = scope.into(); self
    }

    pub  fn scope(&mut self) -> &mut String {
        &mut self.scope
    }

    /// Modify [`Self::scope`] with a `func`
    pub  fn scope_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.scope); self
    }


    /// Set [`Self::subresources`]
    pub  fn subresources_set(&mut self, subresources: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>>) -> &mut Self {
        self.subresources = subresources.into(); self
    }

    pub  fn subresources(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources {
        if self.subresources.is_none() { self.subresources = Some(Default::default()) }
        self.subresources.as_mut().unwrap()
    }

    /// Modify [`Self::subresources`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn subresources_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources)) -> &mut Self {
        if self.subresources.is_none() { self.subresources = Some(Default::default()) };
        func(self.subresources.as_mut().unwrap()); self
    }


    /// Set [`Self::validation`]
    pub  fn validation_set(&mut self, validation: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>>) -> &mut Self {
        self.validation = validation.into(); self
    }

    pub  fn validation(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation {
        if self.validation.is_none() { self.validation = Some(Default::default()) }
        self.validation.as_mut().unwrap()
    }

    /// Modify [`Self::validation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn validation_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation)) -> &mut Self {
        if self.validation.is_none() { self.validation = Some(Default::default()) };
        func(self.validation.as_mut().unwrap()); self
    }


    /// Set [`Self::version`]
    pub  fn version_set(&mut self, version: impl Into<Option<String>>) -> &mut Self {
        self.version = version.into(); self
    }

    pub  fn version(&mut self) -> &mut String {
        if self.version.is_none() { self.version = Some(Default::default()) }
        self.version.as_mut().unwrap()
    }

    /// Modify [`Self::version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.version.is_none() { self.version = Some(Default::default()) };
        func(self.version.as_mut().unwrap()); self
    }


    /// Set [`Self::versions`]
    pub  fn versions_set(&mut self, versions: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>>>) -> &mut Self {
        self.versions = versions.into(); self
    }

    pub  fn versions(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion> {
        if self.versions.is_none() { self.versions = Some(Default::default()) }
        self.versions.as_mut().unwrap()
    }

    /// Modify [`Self::versions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn versions_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion>)) -> &mut Self {
        if self.versions.is_none() { self.versions = Some(Default::default()) };
        func(self.versions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn versions_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion)) -> &mut Self {
        if self.versions.is_none() {
            self.versions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.versions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::versions`]
    pub  fn versions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionVersion]>) -> &mut Self {
         if self.versions.is_none() { self.versions = Some(Vec::new()); }
         let versions = &mut self.versions.as_mut().unwrap();
         for item in other.borrow() {
             versions.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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
                let mut value_preserve_unknown_fields: Option<bool> = None;
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
                        Field::Key_preserve_unknown_fields => value_preserve_unknown_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    preserve_unknown_fields: value_preserve_unknown_fields,
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

impl crate::serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
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
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", value)?;
        }
        if let Some(value) = &self.conversion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        if let Some(value) = &self.preserve_unknown_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preserveUnknownFields", value)?;
        }
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
                properties: [
                    (
                        "additionalPrinterColumns".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If present, this field configures columns for all versions. Top-level and per-version columns are mutually exclusive. If no top-level or per-version columns are specified, a single column displaying the age of the custom resource is used.".to_owned()),
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
                                description: Some("conversion defines conversion settings for the CRD.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("group is the API group of the defined custom resource. The custom resources are served under `/apis/<group>/...`. Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).".to_owned()),
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
                                description: Some("names specify the resource and kind names for the custom resource.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "preserveUnknownFields".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. If false, schemas must be defined for all versions. Defaults to true in v1beta for backwards compatibility. Deprecated: will be required to be false in v1. Preservation of unknown fields can be specified in the validation schema using the `x-kubernetes-preserve-unknown-fields: true` extension. See https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/#pruning-versus-preserving-unknown-fields for details.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "scope".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`. Default is `Namespaced`.".to_owned()),
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
                                description: Some("subresources specify what subresources the defined custom resource has. If present, this field configures subresources for all versions. Top-level and per-version subresources are mutually exclusive.".to_owned()),
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
                                description: Some("validation describes the schema used for validation and pruning of the custom resource. If present, this validation schema is used to validate all versions. Top-level and per-version schemas are mutually exclusive.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "version".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("version is the API version of the defined custom resource. The custom resources are served under `/apis/<group>/<version>/...`. Must match the name of the first item in the `versions` list if `version` and `versions` are both specified. Optional if `versions` is specified. Deprecated: use `versions` instead.".to_owned()),
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
                                description: Some("versions is the list of all API versions of the defined custom resource. Optional if `version` is specified. The name of the first item in the `versions` list must match the `version` field if `version` and `versions` are both specified. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.".to_owned()),
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
                ].into(),
                required: [
                    "group".to_owned(),
                    "names".to_owned(),
                    "scope".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
