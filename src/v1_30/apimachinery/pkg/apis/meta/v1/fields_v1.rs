// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.FieldsV1

/// FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.
///
/// Each key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:\<name\>', where \<name\> is the name of a field in a struct, or key in a map 'v:\<value\>', where \<value\> is the exact json formatted value of a list item 'i:\<index\>', where \<index\> is position of a item in a list 'k:\<keys\>', where \<keys\> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.
///
/// The exact format is defined in sigs.k8s.io/structured-merge-diff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldsV1(pub crate::serde_json::Value);

impl crate::DeepMerge for FieldsV1 {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FieldsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FieldsV1;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FieldsV1")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(FieldsV1(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("FieldsV1", Visitor)
    }
}

impl crate::serde::Serialize for FieldsV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("FieldsV1", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FieldsV1 {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.FieldsV1".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.\n\nEach key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:<name>', where <name> is the name of a field in a struct, or key in a map 'v:<value>', where <value> is the exact json formatted value of a list item 'i:<index>', where <index> is position of a item in a list 'k:<keys>', where <keys> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.\n\nThe exact format is defined in sigs.k8s.io/structured-merge-diff".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
