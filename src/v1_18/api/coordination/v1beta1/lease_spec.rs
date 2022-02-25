// Generated from definition io.k8s.api.coordination.v1beta1.LeaseSpec

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

impl<'de> crate::serde::Deserialize<'de> for LeaseSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_acquire_time,
            Key_holder_identity,
            Key_lease_duration_seconds,
            Key_lease_transitions,
            Key_renew_time,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LeaseSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LeaseSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_acquire_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_holder_identity: Option<String> = None;
                let mut value_lease_duration_seconds: Option<i32> = None;
                let mut value_lease_transitions: Option<i32> = None;
                let mut value_renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_acquire_time => value_acquire_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_holder_identity => value_holder_identity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_duration_seconds => value_lease_duration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_transitions => value_lease_transitions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_renew_time => value_renew_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for LeaseSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LeaseSpec",
            self.acquire_time.as_ref().map_or(0, |_| 1) +
            self.holder_identity.as_ref().map_or(0, |_| 1) +
            self.lease_duration_seconds.as_ref().map_or(0, |_| 1) +
            self.lease_transitions.as_ref().map_or(0, |_| 1) +
            self.renew_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.acquire_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "acquireTime", value)?;
        }
        if let Some(value) = &self.holder_identity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "holderIdentity", value)?;
        }
        if let Some(value) = &self.lease_duration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "leaseDurationSeconds", value)?;
        }
        if let Some(value) = &self.lease_transitions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "leaseTransitions", value)?;
        }
        if let Some(value) = &self.renew_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "renewTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LeaseSpec {
    fn schema_name() -> String {
        "io.k8s.api.coordination.v1beta1.LeaseSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LeaseSpec is a specification of a Lease.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "acquireTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("acquireTime is a time when the current lease was acquired.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "holderIdentity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("holderIdentity contains the identity of the holder of a current lease.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "leaseDurationSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measure against time of last observed RenewTime.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "leaseTransitions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("leaseTransitions is the number of transitions of a lease between holders.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "renewTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("renewTime is a time when the current holder of a lease has last updated the lease.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
