// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimStatus

/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimStatus {
    /// AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Vec<String>,

    /// Represents the actual resources of the underlying volume.
    pub capacity: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
    pub conditions: Vec<crate::api::core::v1::PersistentVolumeClaimCondition>,

    /// Phase represents the current phase of PersistentVolumeClaim.
    pub phase: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_capacity,
            Key_conditions,
            Key_phase,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "accessModes" => Field::Key_access_modes,
                            "capacity" => Field::Key_capacity,
                            "conditions" => Field::Key_conditions,
                            "phase" => Field::Key_phase,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<Vec<crate::api::core::v1::PersistentVolumeClaimCondition>> = None;
                let mut value_phase: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimStatus {
                    access_modes: value_access_modes.unwrap_or_default(),
                    capacity: value_capacity.unwrap_or_default(),
                    conditions: value_conditions.unwrap_or_default(),
                    phase: value_phase,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimStatus",
            &[
                "accessModes",
                "capacity",
                "conditions",
                "phase",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimStatus",
            usize::from(!self.access_modes.is_empty()) +
            usize::from(!self.capacity.is_empty()) +
            usize::from(!self.conditions.is_empty()) +
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if !self.access_modes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", &self.access_modes)?;
        }
        if !self.capacity.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", &self.capacity)?;
        }
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl PersistentVolumeClaimStatus {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PersistentVolumeClaimStatus is the current status of a persistent volume claim.",
          "properties": {
            "accessModes": {
              "description": "AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "capacity": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "Represents the actual resources of the underlying volume.",
              "type": "object"
            },
            "conditions": {
              "description": "Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.",
              "items": crate::api::core::v1::PersistentVolumeClaimCondition::schema(),
              "type": "array"
            },
            "phase": {
              "description": "Phase represents the current phase of PersistentVolumeClaim.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
