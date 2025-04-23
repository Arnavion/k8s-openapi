// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrStringArray

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Clone, Debug, PartialEq)]
pub enum JSONSchemaPropsOrStringArray {
    Schema(std::boxed::Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>),
    Strings(std::vec::Vec<std::string::String>),
}

impl crate::DeepMerge for JSONSchemaPropsOrStringArray {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl<'de> crate::serde::Deserialize<'de> for JSONSchemaPropsOrStringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrStringArray;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JSONSchemaPropsOrStringArray")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                Ok(JSONSchemaPropsOrStringArray::Schema(crate::serde::de::Deserialize::deserialize(crate::serde::de::value::MapAccessDeserializer::new(map))?))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::SeqAccess<'de> {
                Ok(JSONSchemaPropsOrStringArray::Strings(crate::serde::de::Deserialize::deserialize(crate::serde::de::value::SeqAccessDeserializer::new(seq))?))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl crate::serde::Serialize for JSONSchemaPropsOrStringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        match self {
            JSONSchemaPropsOrStringArray::Schema(value) => value.serialize(serializer),
            JSONSchemaPropsOrStringArray::Strings(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONSchemaPropsOrStringArray {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrStringArray".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.".into()),
                ..Default::default()
            })),
            subschemas: Some(std::boxed::Box::new(crate::schemars::schema::SubschemaValidation {
                one_of: Some(std::vec![
                    __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>(),
                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                        array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                            items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                    instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                    ..Default::default()
                                }),
                            ))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
