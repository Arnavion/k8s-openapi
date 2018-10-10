// Generated from definition io.k8s.api.core.v1.ContainerState

/// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerState {
    /// Details about a running container
    pub running: Option<::v1_12::api::core::v1::ContainerStateRunning>,

    /// Details about a terminated container
    pub terminated: Option<::v1_12::api::core::v1::ContainerStateTerminated>,

    /// Details about a waiting container
    pub waiting: Option<::v1_12::api::core::v1::ContainerStateWaiting>,
}

impl<'de> ::serde::Deserialize<'de> for ContainerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_running,
            Key_terminated,
            Key_waiting,
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
                            "running" => Field::Key_running,
                            "terminated" => Field::Key_terminated,
                            "waiting" => Field::Key_waiting,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerState;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ContainerState")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_running: Option<::v1_12::api::core::v1::ContainerStateRunning> = None;
                let mut value_terminated: Option<::v1_12::api::core::v1::ContainerStateTerminated> = None;
                let mut value_waiting: Option<::v1_12::api::core::v1::ContainerStateWaiting> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_running => value_running = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_terminated => value_terminated = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_waiting => value_waiting = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerState {
                    running: value_running,
                    terminated: value_terminated,
                    waiting: value_waiting,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerState",
            &[
                "running",
                "terminated",
                "waiting",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ContainerState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerState",
            0 +
            self.running.as_ref().map_or(0, |_| 1) +
            self.terminated.as_ref().map_or(0, |_| 1) +
            self.waiting.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.running {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "running", value)?;
        }
        if let Some(value) = &self.terminated {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "terminated", value)?;
        }
        if let Some(value) = &self.waiting {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "waiting", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
