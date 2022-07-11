// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails

/// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusDetails {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    pub causes: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::StatusCause>>,

    /// The group attribute of the resource associated with the status StatusReason.
    pub group: Option<String>,

    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    pub name: Option<String>,

    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    pub retry_after_seconds: Option<i32>,

    /// UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: Option<String>,

}

#[cfg(feature = "dsl")]
impl StatusDetails  {
    /// Set [`Self::causes`]
    pub  fn causes_set(&mut self, causes: impl Into<Option<Vec<crate::apimachinery::pkg::apis::meta::v1::StatusCause>>>) -> &mut Self {
        self.causes = causes.into(); self
    }

    pub  fn causes(&mut self) -> &mut Vec<crate::apimachinery::pkg::apis::meta::v1::StatusCause> {
        if self.causes.is_none() { self.causes = Some(Default::default()) }
        self.causes.as_mut().unwrap()
    }

    /// Modify [`Self::causes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn causes_with(&mut self, func: impl FnOnce(&mut Vec<crate::apimachinery::pkg::apis::meta::v1::StatusCause>)) -> &mut Self {
        if self.causes.is_none() { self.causes = Some(Default::default()) };
        func(self.causes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::causes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn causes_push_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::StatusCause)) -> &mut Self {
        if self.causes.is_none() {
            self.causes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.causes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::causes`]
    pub  fn causes_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apimachinery::pkg::apis::meta::v1::StatusCause]>) -> &mut Self {
         if self.causes.is_none() { self.causes = Some(Vec::new()); }
         let causes = &mut self.causes.as_mut().unwrap();
         for item in other.borrow() {
             causes.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::group`]
    pub  fn group_set(&mut self, group: impl Into<Option<String>>) -> &mut Self {
        self.group = group.into(); self
    }

    pub  fn group(&mut self) -> &mut String {
        if self.group.is_none() { self.group = Some(Default::default()) }
        self.group.as_mut().unwrap()
    }

    /// Modify [`Self::group`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn group_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.group.is_none() { self.group = Some(Default::default()) };
        func(self.group.as_mut().unwrap()); self
    }


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<Option<String>>) -> &mut Self {
        self.kind = kind.into(); self
    }

    pub  fn kind(&mut self) -> &mut String {
        if self.kind.is_none() { self.kind = Some(Default::default()) }
        self.kind.as_mut().unwrap()
    }

    /// Modify [`Self::kind`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn kind_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.kind.is_none() { self.kind = Some(Default::default()) };
        func(self.kind.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<Option<String>>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        if self.name.is_none() { self.name = Some(Default::default()) }
        self.name.as_mut().unwrap()
    }

    /// Modify [`Self::name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.name.is_none() { self.name = Some(Default::default()) };
        func(self.name.as_mut().unwrap()); self
    }


    /// Set [`Self::retry_after_seconds`]
    pub  fn retry_after_seconds_set(&mut self, retry_after_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.retry_after_seconds = retry_after_seconds.into(); self
    }

    pub  fn retry_after_seconds(&mut self) -> &mut i32 {
        if self.retry_after_seconds.is_none() { self.retry_after_seconds = Some(Default::default()) }
        self.retry_after_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::retry_after_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn retry_after_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.retry_after_seconds.is_none() { self.retry_after_seconds = Some(Default::default()) };
        func(self.retry_after_seconds.as_mut().unwrap()); self
    }


    /// Set [`Self::uid`]
    pub  fn uid_set(&mut self, uid: impl Into<Option<String>>) -> &mut Self {
        self.uid = uid.into(); self
    }

    pub  fn uid(&mut self) -> &mut String {
        if self.uid.is_none() { self.uid = Some(Default::default()) }
        self.uid.as_mut().unwrap()
    }

    /// Modify [`Self::uid`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn uid_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.uid.is_none() { self.uid = Some(Default::default()) };
        func(self.uid.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StatusDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_causes,
            Key_group,
            Key_kind,
            Key_name,
            Key_retry_after_seconds,
            Key_uid,
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
                            "causes" => Field::Key_causes,
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "retryAfterSeconds" => Field::Key_retry_after_seconds,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatusDetails;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatusDetails")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_causes: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::StatusCause>> = None;
                let mut value_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_retry_after_seconds: Option<i32> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_causes => value_causes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_retry_after_seconds => value_retry_after_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatusDetails {
                    causes: value_causes,
                    group: value_group,
                    kind: value_kind,
                    name: value_name,
                    retry_after_seconds: value_retry_after_seconds,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatusDetails",
            &[
                "causes",
                "group",
                "kind",
                "name",
                "retryAfterSeconds",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatusDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatusDetails",
            self.causes.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.retry_after_seconds.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.causes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "causes", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.retry_after_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "retryAfterSeconds", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatusDetails {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "causes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::StatusCause>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "group".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The group attribute of the resource associated with the status StatusReason.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "retryAfterSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
