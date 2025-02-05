// Generated from definition io.k8s.api.core.v1.PersistentVolumeStatus

/// PersistentVolumeStatus is the current status of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeStatus {
    /// lastPhaseTransitionTime is the time the phase transitioned from one to another and automatically resets to current time everytime a volume phase transitions. This is a beta field and requires the PersistentVolumeLastPhaseTransitionTime feature to be enabled (enabled by default).
    pub last_phase_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// message is a human-readable message indicating details about why the volume is in this state.
    pub message: Option<std::string::String>,

    /// phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
    pub phase: Option<std::string::String>,

    /// reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    pub reason: Option<std::string::String>,
}

impl crate::DeepMerge for PersistentVolumeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.last_phase_transition_time, other.last_phase_transition_time);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.phase, other.phase);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_phase_transition_time,
            Key_message,
            Key_phase,
            Key_reason,
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
                            "lastPhaseTransitionTime" => Field::Key_last_phase_transition_time,
                            "message" => Field::Key_message,
                            "phase" => Field::Key_phase,
                            "reason" => Field::Key_reason,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PersistentVolumeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_last_phase_transition_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_phase: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_phase_transition_time => value_last_phase_transition_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeStatus {
                    last_phase_transition_time: value_last_phase_transition_time,
                    message: value_message,
                    phase: value_phase,
                    reason: value_reason,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeStatus",
            &[
                "lastPhaseTransitionTime",
                "message",
                "phase",
                "reason",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeStatus",
            self.last_phase_transition_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.last_phase_transition_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastPhaseTransitionTime", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PersistentVolumeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeStatus is the current status of a persistent volume.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "lastPhaseTransitionTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("lastPhaseTransitionTime is the time the phase transitioned from one to another and automatically resets to current time everytime a volume phase transitions. This is a beta field and requires the PersistentVolumeLastPhaseTransitionTime feature to be enabled (enabled by default).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "message".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("message is a human-readable message indicating details about why the volume is in this state.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "phase".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase".into()),
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
                                description: Some("reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
