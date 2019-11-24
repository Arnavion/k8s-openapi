// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray

/// JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps or an array of JSONSchemaProps. Mainly here for serialization purposes.
#[derive(Clone, Debug, PartialEq)]
pub enum JSONSchemaPropsOrArray {
    Schema(Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>),
    Schemas(Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>),
}

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrArray;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaPropsOrArray")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                Ok(JSONSchemaPropsOrArray::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(JSONSchemaPropsOrArray::Schemas(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            JSONSchemaPropsOrArray::Schema(value) => value.serialize(serializer),
            JSONSchemaPropsOrArray::Schemas(value) => value.serialize(serializer),
        }
    }
}
