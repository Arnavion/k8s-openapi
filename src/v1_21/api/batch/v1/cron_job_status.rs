// Generated from definition io.k8s.api.batch.v1.CronJobStatus

/// CronJobStatus represents the current state of a cron job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    pub active: Vec<crate::api::core::v1::ObjectReference>,

    /// Information when was the last time the job was successfully scheduled.
    pub last_schedule_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Information when was the last time the job successfully completed.
    pub last_successful_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> crate::serde::Deserialize<'de> for CronJobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active,
            Key_last_schedule_time,
            Key_last_successful_time,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "active" => Field::Key_active,
                            "lastScheduleTime" => Field::Key_last_schedule_time,
                            "lastSuccessfulTime" => Field::Key_last_successful_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CronJobStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CronJobStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active: Option<Vec<crate::api::core::v1::ObjectReference>> = None;
                let mut value_last_schedule_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_last_successful_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_schedule_time => value_last_schedule_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_successful_time => value_last_successful_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CronJobStatus {
                    active: value_active.unwrap_or_default(),
                    last_schedule_time: value_last_schedule_time,
                    last_successful_time: value_last_successful_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "CronJobStatus",
            &[
                "active",
                "lastScheduleTime",
                "lastSuccessfulTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CronJobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CronJobStatus",
            usize::from(!self.active.is_empty()) +
            self.last_schedule_time.as_ref().map_or(0, |_| 1) +
            self.last_successful_time.as_ref().map_or(0, |_| 1),
        )?;
        if !self.active.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "active", &self.active)?;
        }
        if let Some(value) = &self.last_schedule_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastScheduleTime", value)?;
        }
        if let Some(value) = &self.last_successful_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastSuccessfulTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CronJobStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CronJobStatus represents the current state of a cron job.",
          "properties": {
            "active": {
              "description": "A list of pointers to currently running jobs.",
              "items": crate::api::core::v1::ObjectReference::schema(),
              "type": "array"
            },
            "lastScheduleTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Information when was the last time the job was successfully scheduled."),
            "lastSuccessfulTime": crate::schema_ref_with_description(crate::apimachinery::pkg::apis::meta::v1::Time::schema(), "Information when was the last time the job successfully completed.")
          },
          "type": "object"
        })
    }
}
