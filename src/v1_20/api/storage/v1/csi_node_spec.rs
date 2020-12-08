// Generated from definition io.k8s.api.storage.v1.CSINodeSpec

/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
    pub drivers: Vec<crate::api::storage::v1::CSINodeDriver>,
}

impl<'de> serde::Deserialize<'de> for CSINodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_drivers,
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
                            "drivers" => Field::Key_drivers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSINodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_drivers: Option<Vec<crate::api::storage::v1::CSINodeDriver>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_drivers => value_drivers = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeSpec {
                    drivers: value_drivers.ok_or_else(|| serde::de::Error::missing_field("drivers"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeSpec",
            &[
                "drivers",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CSINodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeSpec",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "drivers", &self.drivers)?;
        serde::ser::SerializeStruct::end(state)
    }
}
