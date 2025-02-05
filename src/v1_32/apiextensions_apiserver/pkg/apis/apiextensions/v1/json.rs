// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSON

/// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, \[\]interface{}, map\[string\]interface{} and nil.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSON(pub crate::serde_json::Value);

impl crate::DeepMerge for JSON {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for JSON {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSON;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JSON")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(JSON(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("JSON", Visitor)
    }
}

impl crate::serde::Serialize for JSON {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("JSON", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSON {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSON".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
