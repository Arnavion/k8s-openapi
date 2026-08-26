// Generated from definition io.k8s.api.lifecycle.v1alpha1.TargetResponder

/// TargetResponder allows you to specify the responder reacting to the Eviction. Responders should observe and communicate through the Eviction API (see .state) to help with the graceful eviction of a target (e.g. termination of a pod).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TargetResponder {
    /// name allows you to identify the responder reacting to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field must be unique for each responder. This field is required.
    pub name: std::string::String,

    /// priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.
    ///
    /// The responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.
    ///
    /// The minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required and immutable.
    pub priority: i32,

    /// state specifies a state that is assigned by the evictionrequest-controller. Responders should observe this state in order to navigate their lifecycle. - Inactive means that the responder should not yet process this eviction request. - Active means that the responder is either running or expected to start soon.
    ///   Also, startTime has been set in the ResponderStatus by the evictionrequest-controller.
    ///
    ///   An active responder should currently interact with the eviction process by updating
    ///   .status.responders, where .name is the active responder name. ResponderStatus fields
    ///   should be periodically updated to indicate the progress or completion of the eviction process.
    ///   If .status.responders\[\].heartbeatTime field is not updated within the heartbeat deadline defined
    ///   by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder
    ///      with a lower priority. Only one responder can be active at a time.
    /// - Interrupted means that the responder has failed to start or failed to update
    ///   heartbeatTime in ResponderStatus in a timely manner.
    /// - Canceled means that the responder has been canceled. In other words, there    is no
    ///   EvictionRequest with the same target and Eviction intent in .spec.intent.
    /// - Completed means that the responder has successfully completed and set completionTime
    ///   in ResponderStatus.
    ///
    /// Please refer to the ResponderStatus in .status.responders for more details on each responder.
    pub state: std::string::String,
}

impl crate::DeepMerge for TargetResponder {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
        crate::DeepMerge::merge_from(&mut self.state, other.state);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TargetResponder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_priority,
            Key_state,
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
                            "name" => Field::Key_name,
                            "priority" => Field::Key_priority,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TargetResponder;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TargetResponder")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;
                let mut value_state: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TargetResponder {
                    name: value_name.unwrap_or_default(),
                    priority: value_priority.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "TargetResponder",
            &[
                "name",
                "priority",
                "state",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TargetResponder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TargetResponder",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", &self.priority)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TargetResponder {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.TargetResponder".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TargetResponder allows you to specify the responder reacting to the Eviction. Responders should observe and communicate through the Eviction API (see .state) to help with the graceful eviction of a target (e.g. termination of a pod).",
            "type": "object",
            "properties": {
                "name": {
                    "description": "name allows you to identify the responder reacting to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field must be unique for each responder. This field is required.",
                    "type": "string",
                },
                "priority": {
                    "description": "priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.\n\nThe responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.\n\nThe minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required and immutable.",
                    "type": "integer",
                    "format": "int32",
                },
                "state": {
                    "description": "state specifies a state that is assigned by the evictionrequest-controller. Responders should observe this state in order to navigate their lifecycle. - Inactive means that the responder should not yet process this eviction request. - Active means that the responder is either running or expected to start soon.\n  Also, startTime has been set in the ResponderStatus by the evictionrequest-controller.\n\n  An active responder should currently interact with the eviction process by updating\n  .status.responders, where .name is the active responder name. ResponderStatus fields\n  should be periodically updated to indicate the progress or completion of the eviction process.\n  If .status.responders[].heartbeatTime field is not updated within the heartbeat deadline defined\n  by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder\n\t with a lower priority. Only one responder can be active at a time.\n- Interrupted means that the responder has failed to start or failed to update\n  heartbeatTime in ResponderStatus in a timely manner.\n- Canceled means that the responder has been canceled. In other words, there\tis no\n  EvictionRequest with the same target and Eviction intent in .spec.intent.\n- Completed means that the responder has successfully completed and set completionTime\n  in ResponderStatus.\n\nPlease refer to the ResponderStatus in .status.responders for more details on each responder.",
                    "type": "string",
                },
            },
            "required": [
                "name",
                "priority",
                "state",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for TargetResponder {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.TargetResponder".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("TargetResponder allows you to specify the responder reacting to the Eviction. Responders should observe and communicate through the Eviction API (see .state) to help with the graceful eviction of a target (e.g. termination of a pod).".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name allows you to identify the responder reacting to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). This field must be unique for each responder. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priority".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.\n\nThe responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.\n\nThe minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required and immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "state".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("state specifies a state that is assigned by the evictionrequest-controller. Responders should observe this state in order to navigate their lifecycle. - Inactive means that the responder should not yet process this eviction request. - Active means that the responder is either running or expected to start soon.\n  Also, startTime has been set in the ResponderStatus by the evictionrequest-controller.\n\n  An active responder should currently interact with the eviction process by updating\n  .status.responders, where .name is the active responder name. ResponderStatus fields\n  should be periodically updated to indicate the progress or completion of the eviction process.\n  If .status.responders[].heartbeatTime field is not updated within the heartbeat deadline defined\n  by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder\n\t with a lower priority. Only one responder can be active at a time.\n- Interrupted means that the responder has failed to start or failed to update\n  heartbeatTime in ResponderStatus in a timely manner.\n- Canceled means that the responder has been canceled. In other words, there\tis no\n  EvictionRequest with the same target and Eviction intent in .spec.intent.\n- Completed means that the responder has successfully completed and set completionTime\n  in ResponderStatus.\n\nPlease refer to the ResponderStatus in .status.responders for more details on each responder.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "priority".into(),
                    "state".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
