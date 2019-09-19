// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrBool

/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaPropsOrBool(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for JSONSchemaPropsOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaPropsOrBool;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "JSONSchemaPropsOrBool")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(JSONSchemaPropsOrBool(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("JSONSchemaPropsOrBool", Visitor)
    }
}

impl serde::Serialize for JSONSchemaPropsOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("JSONSchemaPropsOrBool", &self.0)
    }
}
