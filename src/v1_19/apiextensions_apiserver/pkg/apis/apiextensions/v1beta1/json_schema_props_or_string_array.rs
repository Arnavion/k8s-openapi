// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrStringArray

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Clone, Debug, PartialEq)]
pub enum JSONSchemaPropsOrStringArray {
    Schema(Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::JSONSchemaProps>),
    Strings(Vec<String>),
}

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrStringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrStringArray;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaPropsOrStringArray")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                Ok(JSONSchemaPropsOrStringArray::Schema(serde::de::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(JSONSchemaPropsOrStringArray::Strings(serde::de::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrStringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            JSONSchemaPropsOrStringArray::Schema(value) => value.serialize(serializer),
            JSONSchemaPropsOrStringArray::Strings(value) => value.serialize(serializer),
        }
    }
}
