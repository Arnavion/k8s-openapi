// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrBool

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.
#[derive(Clone, Debug, PartialEq)]
pub enum JSONSchemaPropsOrBool {
    Schema(Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>),
    Bool(bool),
}

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrBool;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaPropsOrBool")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                Ok(JSONSchemaPropsOrBool::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(JSONSchemaPropsOrBool::Bool(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            JSONSchemaPropsOrBool::Schema(value) => value.serialize(serializer),
            JSONSchemaPropsOrBool::Bool(value) => value.serialize(serializer),
        }
    }
}
