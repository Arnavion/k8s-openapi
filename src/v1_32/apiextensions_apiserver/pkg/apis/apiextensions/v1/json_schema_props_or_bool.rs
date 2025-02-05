// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrBool

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.
#[derive(Clone, Debug, PartialEq)]
pub enum JSONSchemaPropsOrBool {
    Schema(std::boxed::Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>),
    Bool(bool),
}

impl crate::DeepMerge for JSONSchemaPropsOrBool {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl<'de> crate::serde::Deserialize<'de> for JSONSchemaPropsOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrBool;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JSONSchemaPropsOrBool")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                Ok(JSONSchemaPropsOrBool::Schema(crate::serde::de::Deserialize::deserialize(crate::serde::de::value::MapAccessDeserializer::new(map))?))
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                Ok(JSONSchemaPropsOrBool::Bool(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl crate::serde::Serialize for JSONSchemaPropsOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        match self {
            JSONSchemaPropsOrBool::Schema(value) => value.serialize(serializer),
            JSONSchemaPropsOrBool::Bool(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONSchemaPropsOrBool {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrBool".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.".into()),
                ..Default::default()
            })),
            subschemas: Some(std::boxed::Box::new(crate::schemars::schema::SubschemaValidation {
                one_of: Some(std::vec![
                    __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>(),
                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
