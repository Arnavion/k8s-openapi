// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSON

/// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, \[\]interface{}, map\[string\]interface{} and nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSON(pub ::serde_json::Value);

impl<'de> ::serde::Deserialize<'de> for JSON {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JSON;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "JSON")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: ::serde::Deserializer<'de> {
                Ok(JSON(::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("JSON", Visitor)
    }
}

impl ::serde::Serialize for JSON {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_newtype_struct("JSON", &self.0)
    }
}
