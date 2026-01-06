// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Time

/// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time(pub crate::jiff::Timestamp);

impl crate::DeepMerge for Time {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Time;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Time")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(Time(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Time", Visitor)
    }
}

impl From<crate::jiff::Timestamp> for Time {
    fn from(inner: crate::jiff::Timestamp) -> Self {
        Self(inner)
    }
}

impl crate::serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("Time", &crate::jiff::fmt::strtime::format("%Y-%m-%dT%H:%M:%SZ", self.0).map_err(crate::serde::ser::Error::custom)?)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Time {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.Time".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.",
            "type": "string",
            "format": "date-time",
        })
    }
}
