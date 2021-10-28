// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Time

/// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time(pub crate::time::OffsetDateTime);

impl<'de> crate::serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Time;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Time")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = crate::time::OffsetDateTime;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("OffsetDateTime")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        crate::time::OffsetDateTime::parse(s, &crate::time::format_description::well_known::Rfc3339).map_err(serde::de::Error::custom)
                    }
                }

                Ok(Time(deserializer.deserialize_str(Visitor)?))
            }
        }

        deserializer.deserialize_newtype_struct("Time", Visitor)
    }
}

impl crate::serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("Time", &self.0.format(crate::time2::RFC3339_SUBSECONDS_ZERO).map_err(crate::serde::ser::Error::custom)?)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Time {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.Time".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
            format: Some("date-time".to_owned()),
            ..Default::default()
        })
    }
}
