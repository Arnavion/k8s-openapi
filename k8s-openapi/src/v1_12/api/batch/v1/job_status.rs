// Generated from definition io.k8s.api.batch.v1.JobStatus

/// JobStatus represents the current state of a Job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobStatus {
    /// The number of actively running pods.
    pub active: Option<i32>,

    /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    pub completion_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub conditions: Option<Vec<::v1_12::api::batch::v1::JobCondition>>,

    /// The number of pods which reached phase Failed.
    pub failed: Option<i32>,

    /// Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    pub start_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// The number of pods which reached phase Succeeded.
    pub succeeded: Option<i32>,
}

impl<'de> ::serde::Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active,
            Key_completion_time,
            Key_conditions,
            Key_failed,
            Key_start_time,
            Key_succeeded,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "active" => Field::Key_active,
                            "completionTime" => Field::Key_completion_time,
                            "conditions" => Field::Key_conditions,
                            "failed" => Field::Key_failed,
                            "startTime" => Field::Key_start_time,
                            "succeeded" => Field::Key_succeeded,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct JobStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_active: Option<i32> = None;
                let mut value_completion_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_conditions: Option<Vec<::v1_12::api::batch::v1::JobCondition>> = None;
                let mut value_failed: Option<i32> = None;
                let mut value_start_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_succeeded: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completion_time => value_completion_time = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed => value_failed = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_succeeded => value_succeeded = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JobStatus {
                    active: value_active,
                    completion_time: value_completion_time,
                    conditions: value_conditions,
                    failed: value_failed,
                    start_time: value_start_time,
                    succeeded: value_succeeded,
                })
            }
        }

        deserializer.deserialize_struct(
            "JobStatus",
            &[
                "active",
                "completionTime",
                "conditions",
                "failed",
                "startTime",
                "succeeded",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JobStatus",
            0 +
            self.active.as_ref().map_or(0, |_| 1) +
            self.completion_time.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.failed.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1) +
            self.succeeded.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "active", value)?;
        }
        if let Some(value) = &self.completion_time {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "completionTime", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.failed {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "failed", value)?;
        }
        if let Some(value) = &self.start_time {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        if let Some(value) = &self.succeeded {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "succeeded", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
