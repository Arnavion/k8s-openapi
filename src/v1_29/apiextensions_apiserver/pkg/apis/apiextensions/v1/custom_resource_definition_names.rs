// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionNames

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionNames {
    /// categories is a list of grouped resources this custom resource belongs to (e.g. 'all'). This is published in API discovery documents, and used by clients to support invocations like `kubectl get all`.
    pub categories: Option<Vec<String>>,

    /// kind is the serialized kind of the resource. It is normally CamelCase and singular. Custom resource instances will use this value as the `kind` attribute in API calls.
    pub kind: String,

    /// listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
    pub list_kind: Option<String>,

    /// plural is the plural name of the resource to serve. The custom resources are served under `/apis/\<group\>/\<version\>/.../\<plural\>`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`). Must be all lowercase.
    pub plural: String,

    /// shortNames are short names for the resource, exposed in API discovery documents, and used by clients to support invocations like `kubectl get \<shortname\>`. It must be all lowercase.
    pub short_names: Option<Vec<String>>,

    /// singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
    pub singular: Option<String>,
}

impl crate::DeepMerge for CustomResourceDefinitionNames {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.categories, other.categories);
        crate::DeepMerge::merge_from(&mut self.kind, other.kind);
        crate::DeepMerge::merge_from(&mut self.list_kind, other.list_kind);
        crate::DeepMerge::merge_from(&mut self.plural, other.plural);
        crate::merge_strategies::list::atomic(&mut self.short_names, other.short_names);
        crate::DeepMerge::merge_from(&mut self.singular, other.singular);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_categories,
            Key_kind,
            Key_list_kind,
            Key_plural,
            Key_short_names,
            Key_singular,
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
                            "categories" => Field::Key_categories,
                            "kind" => Field::Key_kind,
                            "listKind" => Field::Key_list_kind,
                            "plural" => Field::Key_plural,
                            "shortNames" => Field::Key_short_names,
                            "singular" => Field::Key_singular,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionNames;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionNames")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_categories: Option<Vec<String>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_list_kind: Option<String> = None;
                let mut value_plural: Option<String> = None;
                let mut value_short_names: Option<Vec<String>> = None;
                let mut value_singular: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_categories => value_categories = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_list_kind => value_list_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_plural => value_plural = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_short_names => value_short_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_singular => value_singular = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionNames {
                    categories: value_categories,
                    kind: value_kind.unwrap_or_default(),
                    list_kind: value_list_kind,
                    plural: value_plural.unwrap_or_default(),
                    short_names: value_short_names,
                    singular: value_singular,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionNames",
            &[
                "categories",
                "kind",
                "listKind",
                "plural",
                "shortNames",
                "singular",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceDefinitionNames {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionNames",
            2 +
            self.categories.as_ref().map_or(0, |_| 1) +
            self.list_kind.as_ref().map_or(0, |_| 1) +
            self.short_names.as_ref().map_or(0, |_| 1) +
            self.singular.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.categories {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "categories", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        if let Some(value) = &self.list_kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "listKind", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "plural", &self.plural)?;
        if let Some(value) = &self.short_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shortNames", value)?;
        }
        if let Some(value) = &self.singular {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "singular", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceDefinitionNames {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionNames".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "categories".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("categories is a list of grouped resources this custom resource belongs to (e.g. 'all'). This is published in API discovery documents, and used by clients to support invocations like `kubectl get all`.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("kind is the serialized kind of the resource. It is normally CamelCase and singular. Custom resource instances will use this value as the `kind` attribute in API calls.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "listKind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("listKind is the serialized kind of the list for this resource. Defaults to \"`kind`List\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "plural".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("plural is the plural name of the resource to serve. The custom resources are served under `/apis/<group>/<version>/.../<plural>`. Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`). Must be all lowercase.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shortNames".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("shortNames are short names for the resource, exposed in API discovery documents, and used by clients to support invocations like `kubectl get <shortname>`. It must be all lowercase.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "singular".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "kind".to_owned(),
                    "plural".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
