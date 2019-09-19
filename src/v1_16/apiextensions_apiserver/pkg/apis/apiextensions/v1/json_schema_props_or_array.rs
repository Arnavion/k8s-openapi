// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrArray

/// JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps or an array of JSONSchemaProps. Mainly here for serialization purposes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaPropsOrArray(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrArray;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "JSONSchemaPropsOrArray")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(JSONSchemaPropsOrArray(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("JSONSchemaPropsOrArray", Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("JSONSchemaPropsOrArray", &self.0)
    }
}
