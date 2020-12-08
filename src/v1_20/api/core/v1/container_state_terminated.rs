// Generated from definition io.k8s.api.core.v1.ContainerStateTerminated

/// ContainerStateTerminated is a terminated state of a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStateTerminated {
    /// Container's ID in the format 'docker://\<container_id\>'
    pub container_id: Option<String>,

    /// Exit status from the last termination of the container
    pub exit_code: i32,

    /// Time at which the container last terminated
    pub finished_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Message regarding the last termination of the container
    pub message: Option<String>,

    /// (brief) reason from the last termination of the container
    pub reason: Option<String>,

    /// Signal from the last termination of the container
    pub signal: Option<i32>,

    /// Time at which previous execution of the container started
    pub started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl<'de> serde::Deserialize<'de> for ContainerStateTerminated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_id,
            Key_exit_code,
            Key_finished_at,
            Key_message,
            Key_reason,
            Key_signal,
            Key_started_at,
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
                            "containerID" => Field::Key_container_id,
                            "exitCode" => Field::Key_exit_code,
                            "finishedAt" => Field::Key_finished_at,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "signal" => Field::Key_signal,
                            "startedAt" => Field::Key_started_at,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStateTerminated;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStateTerminated")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_container_id: Option<String> = None;
                let mut value_exit_code: Option<i32> = None;
                let mut value_finished_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_signal: Option<i32> = None;
                let mut value_started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_id => value_container_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exit_code => value_exit_code = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_finished_at => value_finished_at = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signal => value_signal = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_started_at => value_started_at = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStateTerminated {
                    container_id: value_container_id,
                    exit_code: value_exit_code.ok_or_else(|| serde::de::Error::missing_field("exitCode"))?,
                    finished_at: value_finished_at,
                    message: value_message,
                    reason: value_reason,
                    signal: value_signal,
                    started_at: value_started_at,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStateTerminated",
            &[
                "containerID",
                "exitCode",
                "finishedAt",
                "message",
                "reason",
                "signal",
                "startedAt",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ContainerStateTerminated {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStateTerminated",
            1 +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.finished_at.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.signal.as_ref().map_or(0, |_| 1) +
            self.started_at.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "exitCode", &self.exit_code)?;
        if let Some(value) = &self.finished_at {
            serde::ser::SerializeStruct::serialize_field(&mut state, "finishedAt", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.signal {
            serde::ser::SerializeStruct::serialize_field(&mut state, "signal", value)?;
        }
        if let Some(value) = &self.started_at {
            serde::ser::SerializeStruct::serialize_field(&mut state, "startedAt", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
