// Generated from definition io.k8s.api.storage.v1.VolumeNodeResources

/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeNodeResources {
    /// Maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.
    pub count: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for VolumeNodeResources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_count,
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
                            "count" => Field::Key_count,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VolumeNodeResources;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VolumeNodeResources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_count: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_count => value_count = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeNodeResources {
                    count: value_count,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeNodeResources",
            &[
                "count",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for VolumeNodeResources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeNodeResources",
            self.count.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.count {
            serde::ser::SerializeStruct::serialize_field(&mut state, "count", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
