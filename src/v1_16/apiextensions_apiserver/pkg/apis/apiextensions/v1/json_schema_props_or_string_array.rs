// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrStringArray

/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaPropsOrStringArray(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrStringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrStringArray;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "JSONSchemaPropsOrStringArray")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(JSONSchemaPropsOrStringArray(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("JSONSchemaPropsOrStringArray", Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrStringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("JSONSchemaPropsOrStringArray", &self.0)
    }
}
