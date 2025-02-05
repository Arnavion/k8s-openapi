// Generated from definition io.k8s.api.resource.v1alpha2.NamedResourcesFilter

/// NamedResourcesFilter is used in ResourceFilterModel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamedResourcesFilter {
    /// Selector is a CEL expression which must evaluate to true if a resource instance is suitable. The language is as defined in https://kubernetes.io/docs/reference/using-api/cel/
    ///
    /// In addition, for each type NamedResourcesin AttributeValue there is a map that resolves to the corresponding value of the instance under evaluation. For example:
    ///
    ///    attributes.quantity\["a"\].isGreaterThan(quantity("0")) &&
    ///    attributes.stringslice\["b"\].isSorted()
    pub selector: std::string::String,
}

impl crate::DeepMerge for NamedResourcesFilter {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamedResourcesFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_selector,
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
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamedResourcesFilter;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamedResourcesFilter")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_selector: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamedResourcesFilter {
                    selector: value_selector.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "NamedResourcesFilter",
            &[
                "selector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamedResourcesFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamedResourcesFilter",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamedResourcesFilter {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.NamedResourcesFilter".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("NamedResourcesFilter is used in ResourceFilterModel.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "selector".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selector is a CEL expression which must evaluate to true if a resource instance is suitable. The language is as defined in https://kubernetes.io/docs/reference/using-api/cel/\n\nIn addition, for each type NamedResourcesin AttributeValue there is a map that resolves to the corresponding value of the instance under evaluation. For example:\n\n   attributes.quantity[\"a\"].isGreaterThan(quantity(\"0\")) &&\n   attributes.stringslice[\"b\"].isSorted()".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "selector".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
