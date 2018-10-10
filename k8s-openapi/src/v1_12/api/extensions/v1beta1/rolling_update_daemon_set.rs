// Generated from definition io.k8s.api.extensions.v1beta1.RollingUpdateDaemonSet

/// Spec to control the desired behavior of daemon set rolling update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0. Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.
    pub max_unavailable: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString>,
}

impl<'de> ::serde::Deserialize<'de> for RollingUpdateDaemonSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_unavailable,
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
                            "maxUnavailable" => Field::Key_max_unavailable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RollingUpdateDaemonSet;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct RollingUpdateDaemonSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_max_unavailable: Option<::v1_12::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_unavailable => value_max_unavailable = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollingUpdateDaemonSet {
                    max_unavailable: value_max_unavailable,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollingUpdateDaemonSet",
            &[
                "maxUnavailable",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for RollingUpdateDaemonSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingUpdateDaemonSet",
            0 +
            self.max_unavailable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_unavailable {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
