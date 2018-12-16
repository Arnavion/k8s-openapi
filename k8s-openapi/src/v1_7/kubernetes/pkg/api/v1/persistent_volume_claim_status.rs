// Generated from definition io.k8s.kubernetes.pkg.api.v1.PersistentVolumeClaimStatus

/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimStatus {
    /// AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<Vec<String>>,

    /// Represents the actual resources of the underlying volume.
    pub capacity: Option<std::collections::BTreeMap<String, crate::v1_7::apimachinery::pkg::api::resource::Quantity>>,

    /// Phase represents the current phase of PersistentVolumeClaim.
    pub phase: Option<String>,
}

impl<'de> serde::Deserialize<'de> for PersistentVolumeClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_capacity,
            Key_phase,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "accessModes" => Field::Key_access_modes,
                            "capacity" => Field::Key_capacity,
                            "phase" => Field::Key_phase,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct PersistentVolumeClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<String, crate::v1_7::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_phase: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimStatus {
                    access_modes: value_access_modes,
                    capacity: value_capacity,
                    phase: value_phase,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimStatus",
            &[
                "accessModes",
                "capacity",
                "phase",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PersistentVolumeClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimStatus",
            0 +
            self.access_modes.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_modes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.capacity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.phase {
            serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
