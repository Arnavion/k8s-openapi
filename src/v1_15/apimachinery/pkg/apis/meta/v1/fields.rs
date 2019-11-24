// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Fields

/// Fields stores a set of fields in a data structure like a Trie. To understand how this is used, see: https://github.com/kubernetes-sigs/structured-merge-diff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fields(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Fields;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Fields")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(Fields(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Fields", Visitor)
    }
}

impl serde::Serialize for Fields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("Fields", &self.0)
    }
}
