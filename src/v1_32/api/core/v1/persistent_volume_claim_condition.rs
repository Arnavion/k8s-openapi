// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimCondition

/// PersistentVolumeClaimCondition contains details about state of pvc
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimCondition {
    /// lastProbeTime is the time we probed the condition.
    pub last_probe_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// lastTransitionTime is the time the condition transitioned from one status to another.
    pub last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// message is the human-readable message indicating details about last transition.
    pub message: Option<std::string::String>,

    /// reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "Resizing" that means the underlying persistent volume is being resized.
    pub reason: Option<std::string::String>,

    /// Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=state%20of%20pvc-,conditions.status,-(string)%2C%20required
    pub status: std::string::String,

    /// Type is the type of the condition. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=set%20to%20%27ResizeStarted%27.-,PersistentVolumeClaimCondition,-contains%20details%20about
    pub type_: std::string::String,
}

impl crate::DeepMerge for PersistentVolumeClaimCondition {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.last_probe_time, other.last_probe_time);
        crate::DeepMerge::merge_from(&mut self.last_transition_time, other.last_transition_time);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_probe_time,
            Key_last_transition_time,
            Key_message,
            Key_reason,
            Key_status,
            Key_type_,
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
                            "lastProbeTime" => Field::Key_last_probe_time,
                            "lastTransitionTime" => Field::Key_last_transition_time,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimCondition;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PersistentVolumeClaimCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_last_probe_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_last_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_status: Option<std::string::String> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_probe_time => value_last_probe_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_transition_time => value_last_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimCondition {
                    last_probe_time: value_last_probe_time,
                    last_transition_time: value_last_transition_time,
                    message: value_message,
                    reason: value_reason,
                    status: value_status.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimCondition",
            &[
                "lastProbeTime",
                "lastTransitionTime",
                "message",
                "reason",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeClaimCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimCondition",
            2 +
            self.last_probe_time.as_ref().map_or(0, |_| 1) +
            self.last_transition_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.last_probe_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastProbeTime", value)?;
        }
        if let Some(value) = &self.last_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastTransitionTime", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeClaimCondition {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PersistentVolumeClaimCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeClaimCondition contains details about state of pvc".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "lastProbeTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("lastProbeTime is the time we probed the condition.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "lastTransitionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("lastTransitionTime is the time the condition transitioned from one status to another.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "message".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("message is the human-readable message indicating details about last transition.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"Resizing\" that means the underlying persistent volume is being resized.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "status".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=state%20of%20pvc-,conditions.status,-(string)%2C%20required".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type is the type of the condition. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=set%20to%20%27ResizeStarted%27.-,PersistentVolumeClaimCondition,-contains%20details%20about".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "status".into(),
                    "type".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
