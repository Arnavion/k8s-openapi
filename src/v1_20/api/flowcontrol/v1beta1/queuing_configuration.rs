// Generated from definition io.k8s.api.flowcontrol.v1beta1.QueuingConfiguration

/// QueuingConfiguration holds the configuration parameters for queuing
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueuingConfiguration {
    /// `handSize` is a small positive number that configures the shuffle sharding of requests into queues.  When enqueuing a request at this priority level the request's flow identifier (a string pair) is hashed and the hash value is used to shuffle the list of queues and deal a hand of the size specified here.  The request is put into one of the shortest queues in that hand. `handSize` must be no larger than `queues`, and should be significantly smaller (so that a few heavy flows do not saturate most of the queues).  See the user-facing documentation for more extensive guidance on setting this field.  This field has a default value of 8.
    pub hand_size: Option<i32>,

    /// `queueLengthLimit` is the maximum number of requests allowed to be waiting in a given queue of this priority level at a time; excess requests are rejected.  This value must be positive.  If not specified, it will be defaulted to 50.
    pub queue_length_limit: Option<i32>,

    /// `queues` is the number of queues for this priority level. The queues exist independently at each apiserver. The value must be positive.  Setting it to 1 effectively precludes shufflesharding and thus makes the distinguisher method of associated flow schemas irrelevant.  This field has a default value of 64.
    pub queues: Option<i32>,

}

#[cfg(feature = "dsl")]
impl QueuingConfiguration  {
    /// Set [`Self::hand_size`]
    pub  fn hand_size_set(&mut self, hand_size: impl Into<Option<i32>>) -> &mut Self {
        self.hand_size = hand_size.into(); self
    }

    pub  fn hand_size(&mut self) -> &mut i32 {
        if self.hand_size.is_none() { self.hand_size = Some(Default::default()) }
        self.hand_size.as_mut().unwrap()
    }

    /// Modify [`Self::hand_size`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn hand_size_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.hand_size.is_none() { self.hand_size = Some(Default::default()) };
        func(self.hand_size.as_mut().unwrap()); self
    }


    /// Set [`Self::queue_length_limit`]
    pub  fn queue_length_limit_set(&mut self, queue_length_limit: impl Into<Option<i32>>) -> &mut Self {
        self.queue_length_limit = queue_length_limit.into(); self
    }

    pub  fn queue_length_limit(&mut self) -> &mut i32 {
        if self.queue_length_limit.is_none() { self.queue_length_limit = Some(Default::default()) }
        self.queue_length_limit.as_mut().unwrap()
    }

    /// Modify [`Self::queue_length_limit`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn queue_length_limit_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.queue_length_limit.is_none() { self.queue_length_limit = Some(Default::default()) };
        func(self.queue_length_limit.as_mut().unwrap()); self
    }


    /// Set [`Self::queues`]
    pub  fn queues_set(&mut self, queues: impl Into<Option<i32>>) -> &mut Self {
        self.queues = queues.into(); self
    }

    pub  fn queues(&mut self) -> &mut i32 {
        if self.queues.is_none() { self.queues = Some(Default::default()) }
        self.queues.as_mut().unwrap()
    }

    /// Modify [`Self::queues`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn queues_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.queues.is_none() { self.queues = Some(Default::default()) };
        func(self.queues.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for QueuingConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hand_size,
            Key_queue_length_limit,
            Key_queues,
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
                            "handSize" => Field::Key_hand_size,
                            "queueLengthLimit" => Field::Key_queue_length_limit,
                            "queues" => Field::Key_queues,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = QueuingConfiguration;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("QueuingConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hand_size: Option<i32> = None;
                let mut value_queue_length_limit: Option<i32> = None;
                let mut value_queues: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hand_size => value_hand_size = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_queue_length_limit => value_queue_length_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_queues => value_queues = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(QueuingConfiguration {
                    hand_size: value_hand_size,
                    queue_length_limit: value_queue_length_limit,
                    queues: value_queues,
                })
            }
        }

        deserializer.deserialize_struct(
            "QueuingConfiguration",
            &[
                "handSize",
                "queueLengthLimit",
                "queues",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for QueuingConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "QueuingConfiguration",
            self.hand_size.as_ref().map_or(0, |_| 1) +
            self.queue_length_limit.as_ref().map_or(0, |_| 1) +
            self.queues.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hand_size {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "handSize", value)?;
        }
        if let Some(value) = &self.queue_length_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "queueLengthLimit", value)?;
        }
        if let Some(value) = &self.queues {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "queues", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for QueuingConfiguration {
    fn schema_name() -> String {
        "io.k8s.api.flowcontrol.v1beta1.QueuingConfiguration".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("QueuingConfiguration holds the configuration parameters for queuing".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "handSize".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`handSize` is a small positive number that configures the shuffle sharding of requests into queues.  When enqueuing a request at this priority level the request's flow identifier (a string pair) is hashed and the hash value is used to shuffle the list of queues and deal a hand of the size specified here.  The request is put into one of the shortest queues in that hand. `handSize` must be no larger than `queues`, and should be significantly smaller (so that a few heavy flows do not saturate most of the queues).  See the user-facing documentation for more extensive guidance on setting this field.  This field has a default value of 8.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "queueLengthLimit".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`queueLengthLimit` is the maximum number of requests allowed to be waiting in a given queue of this priority level at a time; excess requests are rejected.  This value must be positive.  If not specified, it will be defaulted to 50.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "queues".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`queues` is the number of queues for this priority level. The queues exist independently at each apiserver. The value must be positive.  Setting it to 1 effectively precludes shufflesharding and thus makes the distinguisher method of associated flow schemas irrelevant.  This field has a default value of 64.".to_owned()),
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
