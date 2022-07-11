// Generated from definition io.k8s.api.batch.v1.JobStatus

/// JobStatus represents the current state of a Job.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobStatus {
    /// The number of actively running pods.
    pub active: Option<i32>,

    /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    pub completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub conditions: Option<Vec<crate::api::batch::v1::JobCondition>>,

    /// The number of pods which reached phase Failed.
    pub failed: Option<i32>,

    /// Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
    pub start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The number of pods which reached phase Succeeded.
    pub succeeded: Option<i32>,

}

#[cfg(feature = "dsl")]
impl JobStatus  {
    /// Set [`Self::active`]
    pub  fn active_set(&mut self, active: impl Into<Option<i32>>) -> &mut Self {
        self.active = active.into(); self
    }

    pub  fn active(&mut self) -> &mut i32 {
        if self.active.is_none() { self.active = Some(Default::default()) }
        self.active.as_mut().unwrap()
    }

    /// Modify [`Self::active`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn active_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.active.is_none() { self.active = Some(Default::default()) };
        func(self.active.as_mut().unwrap()); self
    }


    /// Set [`Self::completion_time`]
    pub  fn completion_time_set(&mut self, completion_time: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.completion_time = completion_time.into(); self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::batch::v1::JobCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::batch::v1::JobCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::batch::v1::JobCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::batch::v1::JobCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::batch::v1::JobCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::failed`]
    pub  fn failed_set(&mut self, failed: impl Into<Option<i32>>) -> &mut Self {
        self.failed = failed.into(); self
    }

    pub  fn failed(&mut self) -> &mut i32 {
        if self.failed.is_none() { self.failed = Some(Default::default()) }
        self.failed.as_mut().unwrap()
    }

    /// Modify [`Self::failed`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn failed_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.failed.is_none() { self.failed = Some(Default::default()) };
        func(self.failed.as_mut().unwrap()); self
    }


    /// Set [`Self::start_time`]
    pub  fn start_time_set(&mut self, start_time: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.start_time = start_time.into(); self
    }


    /// Set [`Self::succeeded`]
    pub  fn succeeded_set(&mut self, succeeded: impl Into<Option<i32>>) -> &mut Self {
        self.succeeded = succeeded.into(); self
    }

    pub  fn succeeded(&mut self) -> &mut i32 {
        if self.succeeded.is_none() { self.succeeded = Some(Default::default()) }
        self.succeeded.as_mut().unwrap()
    }

    /// Modify [`Self::succeeded`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn succeeded_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.succeeded.is_none() { self.succeeded = Some(Default::default()) };
        func(self.succeeded.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JobStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JobStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_active: Option<i32> = None;
                let mut value_completion_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_conditions: Option<Vec<crate::api::batch::v1::JobCondition>> = None;
                let mut value_failed: Option<i32> = None;
                let mut value_start_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_succeeded: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_completion_time => value_completion_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failed => value_failed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_time => value_start_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_succeeded => value_succeeded = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JobStatus",
            self.active.as_ref().map_or(0, |_| 1) +
            self.completion_time.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.failed.as_ref().map_or(0, |_| 1) +
            self.start_time.as_ref().map_or(0, |_| 1) +
            self.succeeded.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "active", value)?;
        }
        if let Some(value) = &self.completion_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "completionTime", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.failed {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failed", value)?;
        }
        if let Some(value) = &self.start_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startTime", value)?;
        }
        if let Some(value) = &self.succeeded {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "succeeded", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JobStatus {
    fn schema_name() -> String {
        "io.k8s.api.batch.v1.JobStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("JobStatus represents the current state of a Job.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "active".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of actively running pods.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "completionTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::batch::v1::JobCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failed".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods which reached phase Failed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "succeeded".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of pods which reached phase Succeeded.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
