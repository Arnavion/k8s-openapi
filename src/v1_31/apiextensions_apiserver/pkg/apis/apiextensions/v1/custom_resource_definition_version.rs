// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionVersion

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionVersion {
    /// additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.
    pub additional_printer_columns: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition>>,

    /// deprecated indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false.
    pub deprecated: Option<bool>,

    /// deprecationWarning overrides the default warning returned to API clients. May only be set when `deprecated` is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists.
    pub deprecation_warning: Option<std::string::String>,

    /// name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/\<group\>/\<version\>/...` if `served` is true.
    pub name: std::string::String,

    /// schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
    pub schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation>,

    /// selectableFields specifies paths to fields that may be used as field selectors. A maximum of 8 selectable fields are allowed. See https://kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors
    pub selectable_fields: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField>>,

    /// served is a flag enabling/disabling this version from being served via REST APIs
    pub served: bool,

    /// storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.
    pub storage: bool,

    /// subresources specify what subresources this version of the defined custom resource have.
    pub subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources>,
}

impl crate::DeepMerge for CustomResourceDefinitionVersion {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.additional_printer_columns, other.additional_printer_columns);
        crate::DeepMerge::merge_from(&mut self.deprecated, other.deprecated);
        crate::DeepMerge::merge_from(&mut self.deprecation_warning, other.deprecation_warning);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.schema, other.schema);
        crate::merge_strategies::list::atomic(&mut self.selectable_fields, other.selectable_fields);
        crate::DeepMerge::merge_from(&mut self.served, other.served);
        crate::DeepMerge::merge_from(&mut self.storage, other.storage);
        crate::DeepMerge::merge_from(&mut self.subresources, other.subresources);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_additional_printer_columns,
            Key_deprecated,
            Key_deprecation_warning,
            Key_name,
            Key_schema,
            Key_selectable_fields,
            Key_served,
            Key_storage,
            Key_subresources,
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
                            "additionalPrinterColumns" => Field::Key_additional_printer_columns,
                            "deprecated" => Field::Key_deprecated,
                            "deprecationWarning" => Field::Key_deprecation_warning,
                            "name" => Field::Key_name,
                            "schema" => Field::Key_schema,
                            "selectableFields" => Field::Key_selectable_fields,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CustomResourceDefinitionVersion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_additional_printer_columns: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition>> = None;
                let mut value_deprecated: Option<bool> = None;
                let mut value_deprecation_warning: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_schema: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation> = None;
                let mut value_selectable_fields: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField>> = None;
                let mut value_served: Option<bool> = None;
                let mut value_storage: Option<bool> = None;
                let mut value_subresources: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_additional_printer_columns => value_additional_printer_columns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated => value_deprecated = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecation_warning => value_deprecation_warning = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schema => value_schema = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectable_fields => value_selectable_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_served => value_served = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage => value_storage = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subresources => value_subresources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionVersion {
                    additional_printer_columns: value_additional_printer_columns,
                    deprecated: value_deprecated,
                    deprecation_warning: value_deprecation_warning,
                    name: value_name.unwrap_or_default(),
                    schema: value_schema,
                    selectable_fields: value_selectable_fields,
                    served: value_served.unwrap_or_default(),
                    storage: value_storage.unwrap_or_default(),
                    subresources: value_subresources,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionVersion",
            &[
                "additionalPrinterColumns",
                "deprecated",
                "deprecationWarning",
                "name",
                "schema",
                "selectableFields",
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
            self.additional_printer_columns.as_ref().map_or(0, |_| 1) +
            self.deprecated.as_ref().map_or(0, |_| 1) +
            self.deprecation_warning.as_ref().map_or(0, |_| 1) +
            self.schema.as_ref().map_or(0, |_| 1) +
            self.selectable_fields.as_ref().map_or(0, |_| 1) +
            self.subresources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.additional_printer_columns {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalPrinterColumns", value)?;
        }
        if let Some(value) = &self.deprecated {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecated", value)?;
        }
        if let Some(value) = &self.deprecation_warning {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecationWarning", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.schema {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "schema", value)?;
        }
        if let Some(value) = &self.selectable_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectableFields", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "served", &self.served)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storage", &self.storage)?;
        if let Some(value) = &self.subresources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceDefinitionVersion {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionVersion".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("CustomResourceDefinitionVersion describes a version for CRD.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "additionalPrinterColumns".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deprecated".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecated indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "deprecationWarning".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecationWarning overrides the default warning returned to API clients. May only be set when `deprecated` is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "schema".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "selectableFields".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("selectableFields specifies paths to fields that may be used as field selectors. A maximum of 8 selectable fields are allowed. See https://kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "served".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("served is a flag enabling/disabling this version from being served via REST APIs".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storage".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subresources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("subresources specify what subresources this version of the defined custom resource have.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "served".into(),
                    "storage".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
