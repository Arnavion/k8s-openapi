// Generated from definition io.k8s.api.core.v1.NodeConfigStatus

/// NodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeConfigStatus {
    /// Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
    pub active: Option<crate::api::core::v1::NodeConfigSource>,

    /// Assigned reports the checkpointed config the node will try to use. When Node.Spec.ConfigSource is updated, the node checkpoints the associated config payload to local disk, along with a record indicating intended config. The node refers to this record to choose its config checkpoint, and reports this record in Assigned. Assigned only updates in the status after the record has been checkpointed to disk. When the Kubelet is restarted, it tries to make the Assigned config the Active config by loading and validating the checkpointed payload identified by Assigned.
    pub assigned: Option<crate::api::core::v1::NodeConfigSource>,

    /// Error describes any problems reconciling the Spec.ConfigSource to the Active config. Errors may occur, for example, attempting to checkpoint Spec.ConfigSource to the local Assigned record, attempting to checkpoint the payload associated with Spec.ConfigSource, attempting to load or validate the Assigned config, etc. Errors may occur at different points while syncing config. Earlier errors (e.g. download or checkpointing errors) will not result in a rollback to LastKnownGood, and may resolve across Kubelet retries. Later errors (e.g. loading or validating a checkpointed config) will result in a rollback to LastKnownGood. In the latter case, it is usually possible to resolve the error by fixing the config assigned in Spec.ConfigSource. You can find additional information for debugging by searching the error message in the Kubelet log. Error is a human-readable description of the error state; machines can check whether or not Error is empty, but should not rely on the stability of the Error text across Kubelet versions.
    pub error: Option<String>,

    /// LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
    pub last_known_good: Option<crate::api::core::v1::NodeConfigSource>,
}

impl<'de> serde::Deserialize<'de> for NodeConfigStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_active,
            Key_assigned,
            Key_error,
            Key_last_known_good,
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
                            "active" => Field::Key_active,
                            "assigned" => Field::Key_assigned,
                            "error" => Field::Key_error,
                            "lastKnownGood" => Field::Key_last_known_good,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeConfigStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeConfigStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_active: Option<crate::api::core::v1::NodeConfigSource> = None;
                let mut value_assigned: Option<crate::api::core::v1::NodeConfigSource> = None;
                let mut value_error: Option<String> = None;
                let mut value_last_known_good: Option<crate::api::core::v1::NodeConfigSource> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_active => value_active = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_assigned => value_assigned = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_error => value_error = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_known_good => value_last_known_good = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeConfigStatus {
                    active: value_active,
                    assigned: value_assigned,
                    error: value_error,
                    last_known_good: value_last_known_good,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeConfigStatus",
            &[
                "active",
                "assigned",
                "error",
                "lastKnownGood",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for NodeConfigStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeConfigStatus",
            self.active.as_ref().map_or(0, |_| 1) +
            self.assigned.as_ref().map_or(0, |_| 1) +
            self.error.as_ref().map_or(0, |_| 1) +
            self.last_known_good.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.active {
            serde::ser::SerializeStruct::serialize_field(&mut state, "active", value)?;
        }
        if let Some(value) = &self.assigned {
            serde::ser::SerializeStruct::serialize_field(&mut state, "assigned", value)?;
        }
        if let Some(value) = &self.error {
            serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        if let Some(value) = &self.last_known_good {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastKnownGood", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
