// Generated from definition io.k8s.api.core.v1.PersistentVolumeStatus

/// PersistentVolumeStatus is the current status of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeStatus {
    /// A human-readable message indicating details about why the volume is in this state.
    pub message: Option<String>,

    /// Phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
    pub phase: Option<String>,

    /// Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    pub reason: Option<String>,
}

impl<'de> serde::Deserialize<'de> for PersistentVolumeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_message,
            Key_phase,
            Key_reason,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_message: Option<String> = None;
                let mut value_phase: Option<String> = None;
                let mut value_reason: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeStatus {
                    message: value_message,
                    phase: value_phase,
                    reason: value_reason,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeStatus",
            &[
                "message",
                "phase",
                "reason",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PersistentVolumeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeStatus",
            self.message.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.phase {
            serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
