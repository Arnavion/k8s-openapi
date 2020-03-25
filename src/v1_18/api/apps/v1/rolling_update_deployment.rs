// Generated from definition io.k8s.api.apps.v1.RollingUpdateDeployment

/// Spec to control the desired behavior of rolling update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be scheduled above the desired number of pods. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up. Defaults to 25%. Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when the rolling update starts, such that the total number of old and new pods do not exceed 130% of desired pods. Once old pods have been killed, new ReplicaSet can be scaled up further, ensuring that total number of pods running at any time during the update is at most 130% of desired pods.
    pub max_surge: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). Absolute number is calculated from percentage by rounding down. This can not be 0 if MaxSurge is 0. Defaults to 25%. Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods immediately when the rolling update starts. Once new pods are ready, old ReplicaSet can be scaled down further, followed by scaling up the new ReplicaSet, ensuring that the total number of pods available at all times during the update is at least 70% of desired pods.
    pub max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,
}

impl<'de> serde::Deserialize<'de> for RollingUpdateDeployment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_surge,
            Key_max_unavailable,
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
                            "maxSurge" => Field::Key_max_surge,
                            "maxUnavailable" => Field::Key_max_unavailable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RollingUpdateDeployment;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RollingUpdateDeployment")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_max_surge: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_surge => value_max_surge = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_unavailable => value_max_unavailable = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollingUpdateDeployment {
                    max_surge: value_max_surge,
                    max_unavailable: value_max_unavailable,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollingUpdateDeployment",
            &[
                "maxSurge",
                "maxUnavailable",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RollingUpdateDeployment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingUpdateDeployment",
            self.max_surge.as_ref().map_or(0, |_| 1) +
            self.max_unavailable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_surge {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxSurge", value)?;
        }
        if let Some(value) = &self.max_unavailable {
            serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
