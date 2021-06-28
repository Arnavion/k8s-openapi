// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Fields

/// Fields stores a set of fields in a data structure like a Trie. To understand how this is used, see: https://github.com/kubernetes-sigs/structured-merge-diff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fields(pub crate::serde_json::Value);

impl<'de> crate::serde::Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Fields;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Fields")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(Fields(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Fields", Visitor)
    }
}

impl crate::serde::Serialize for Fields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("Fields", &self.0)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Fields {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Fields stores a set of fields in a data structure like a Trie. To understand how this is used, see: https://github.com/kubernetes-sigs/structured-merge-diff",
          "type": "object"
        })
    }
}
