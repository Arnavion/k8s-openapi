// Generated from definition io.k8s.api.coordination.v1.LeaseSpec

/// LeaseSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaseSpec {
    /// acquireTime is a time when the current lease was acquired.
    pub acquire_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// holderIdentity contains the identity of the holder of a current lease.
    pub holder_identity: Option<String>,

    /// leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measure against time of last observed RenewTime.
    pub lease_duration_seconds: Option<i32>,

    /// leaseTransitions is the number of transitions of a lease between holders.
    pub lease_transitions: Option<i32>,

    /// renewTime is a time when the current holder of a lease has last updated the lease.
    pub renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,
}

impl<'de> serde::Deserialize<'de> for LeaseSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_acquire_time,
            Key_holder_identity,
            Key_lease_duration_seconds,
            Key_lease_transitions,
            Key_renew_time,
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
                            "acquireTime" => Field::Key_acquire_time,
                            "holderIdentity" => Field::Key_holder_identity,
                            "leaseDurationSeconds" => Field::Key_lease_duration_seconds,
                            "leaseTransitions" => Field::Key_lease_transitions,
                            "renewTime" => Field::Key_renew_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LeaseSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LeaseSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_acquire_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_holder_identity: Option<String> = None;
                let mut value_lease_duration_seconds: Option<i32> = None;
                let mut value_lease_transitions: Option<i32> = None;
                let mut value_renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_acquire_time => value_acquire_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_holder_identity => value_holder_identity = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_duration_seconds => value_lease_duration_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_transitions => value_lease_transitions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_renew_time => value_renew_time = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LeaseSpec {
                    acquire_time: value_acquire_time,
                    holder_identity: value_holder_identity,
                    lease_duration_seconds: value_lease_duration_seconds,
                    lease_transitions: value_lease_transitions,
                    renew_time: value_renew_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "LeaseSpec",
            &[
                "acquireTime",
                "holderIdentity",
                "leaseDurationSeconds",
                "leaseTransitions",
                "renewTime",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for LeaseSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LeaseSpec",
            self.acquire_time.as_ref().map_or(0, |_| 1) +
            self.holder_identity.as_ref().map_or(0, |_| 1) +
            self.lease_duration_seconds.as_ref().map_or(0, |_| 1) +
            self.lease_transitions.as_ref().map_or(0, |_| 1) +
            self.renew_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.acquire_time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "acquireTime", value)?;
        }
        if let Some(value) = &self.holder_identity {
            serde::ser::SerializeStruct::serialize_field(&mut state, "holderIdentity", value)?;
        }
        if let Some(value) = &self.lease_duration_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "leaseDurationSeconds", value)?;
        }
        if let Some(value) = &self.lease_transitions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "leaseTransitions", value)?;
        }
        if let Some(value) = &self.renew_time {
            serde::ser::SerializeStruct::serialize_field(&mut state, "renewTime", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
