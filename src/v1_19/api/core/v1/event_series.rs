// Generated from definition io.k8s.api.core.v1.EventSeries

/// EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time
    pub count: Option<i32>,

    /// Time of the last occurrence observed
    pub last_observed_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,
}

impl<'de> serde::Deserialize<'de> for EventSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_count,
            Key_last_observed_time,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "count" => Field::Key_count,
                            "lastObservedTime" => Field::Key_last_observed_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EventSeries;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EventSeries")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_count: Option<i32> = None;
                let mut value_last_observed_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_count => value_count = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_observed_time => value_last_observed_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EventSeries {
                    count: value_count,
                    last_observed_time: value_last_observed_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "EventSeries",
            &[
                "count",
                "lastObservedTime",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EventSeries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EventSeries",
            self.count.as_ref().map_or(0, |_| 1) +
            self.last_observed_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.count {
            serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        if let Some(value) = &self.last_observed_time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastObservedTime", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
