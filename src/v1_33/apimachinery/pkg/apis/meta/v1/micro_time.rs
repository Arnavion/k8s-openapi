// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime

/// MicroTime is version of Time with microsecond level precision.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MicroTime(pub crate::chrono::DateTime<crate::chrono::Utc>);

impl crate::DeepMerge for MicroTime {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for MicroTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MicroTime;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("MicroTime")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(MicroTime(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("MicroTime", Visitor)
    }
}

impl crate::serde::Serialize for MicroTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("MicroTime", &self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true))
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MicroTime {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.MicroTime".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("MicroTime is version of Time with microsecond level precision.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
            format: Some("date-time".into()),
            ..Default::default()
        })
    }
}
