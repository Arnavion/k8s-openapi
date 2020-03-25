// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.FieldsV1

/// FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.
///
/// Each key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:\<name\>', where \<name\> is the name of a field in a struct, or key in a map 'v:\<value\>', where \<value\> is the exact json formatted value of a list item 'i:\<index\>', where \<index\> is position of a item in a list 'k:\<keys\>', where \<keys\> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.
///
/// The exact format is defined in sigs.k8s.io/structured-merge-diff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldsV1(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for FieldsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FieldsV1;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FieldsV1")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(FieldsV1(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("FieldsV1", Visitor)
    }
}

impl serde::Serialize for FieldsV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("FieldsV1", &self.0)
    }
}
