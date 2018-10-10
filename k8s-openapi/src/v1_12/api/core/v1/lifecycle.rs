// Generated from definition io.k8s.api.core.v1.Lifecycle

/// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    pub post_start: Option<::v1_12::api::core::v1::Handler>,

    /// PreStop is called immediately before a container is terminated. The container is terminated after the handler completes. The reason for termination is passed to the handler. Regardless of the outcome of the handler, the container is eventually terminated. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    pub pre_stop: Option<::v1_12::api::core::v1::Handler>,
}

impl<'de> ::serde::Deserialize<'de> for Lifecycle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_post_start,
            Key_pre_stop,
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
                            "postStart" => Field::Key_post_start,
                            "preStop" => Field::Key_pre_stop,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Lifecycle;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Lifecycle")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_post_start: Option<::v1_12::api::core::v1::Handler> = None;
                let mut value_pre_stop: Option<::v1_12::api::core::v1::Handler> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_post_start => value_post_start = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pre_stop => value_pre_stop = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Lifecycle {
                    post_start: value_post_start,
                    pre_stop: value_pre_stop,
                })
            }
        }

        deserializer.deserialize_struct(
            "Lifecycle",
            &[
                "postStart",
                "preStop",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Lifecycle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Lifecycle",
            0 +
            self.post_start.as_ref().map_or(0, |_| 1) +
            self.pre_stop.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.post_start {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "postStart", value)?;
        }
        if let Some(value) = &self.pre_stop {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "preStop", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
