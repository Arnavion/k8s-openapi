// Generated from definition io.k8s.api.events.v1.EventSeries

/// EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time. How often to update the EventSeries is up to the event reporters. The default event reporter in "k8s.io/client-go/tools/events/event_broadcaster.go" shows how this struct is updated on heartbeats and can guide customized reporter implementations.
#[derive(Clone, Debug, PartialEq)]
pub struct EventSeries {
    /// count is the number of occurrences in this series up to the last heartbeat time.
    pub count: i32,

    /// lastObservedTime is the time when last Event from the series was seen before last heartbeat.
    pub last_observed_time: crate::apimachinery::pkg::apis::meta::v1::MicroTime,
}

impl crate::DeepMerge for EventSeries {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.count, other.count);
        crate::DeepMerge::merge_from(&mut self.last_observed_time, other.last_observed_time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EventSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_count,
            Key_last_observed_time,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EventSeries;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EventSeries")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_count: Option<i32> = None;
                let mut value_last_observed_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_count => value_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_observed_time => value_last_observed_time = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EventSeries {
                    count: value_count.unwrap_or_default(),
                    last_observed_time: value_last_observed_time.ok_or_else(|| crate::serde::de::Error::missing_field("lastObservedTime"))?,
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

impl crate::serde::Serialize for EventSeries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EventSeries",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "count", &self.count)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastObservedTime", &self.last_observed_time)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EventSeries {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.events.v1.EventSeries".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time. How often to update the EventSeries is up to the event reporters. The default event reporter in \"k8s.io/client-go/tools/events/event_broadcaster.go\" shows how this struct is updated on heartbeats and can guide customized reporter implementations.",
            "type": "object",
            "properties": {
                "count": {
                    "description": "count is the number of occurrences in this series up to the last heartbeat time.",
                    "type": "integer",
                    "format": "int32",
                },
                "lastObservedTime": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>();
                    schema_obj.ensure_object().insert("description".into(), "lastObservedTime is the time when last Event from the series was seen before last heartbeat.".into());
                    schema_obj
                }),
            },
            "required": [
                "count",
                "lastObservedTime",
            ],
        })
    }
}
