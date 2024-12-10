// Generated from definition io.k8s.api.coordination.v1alpha1.LeaseCandidateSpec

/// LeaseCandidateSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaseCandidateSpec {
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required when strategy is "OldestEmulationVersion"
    pub binary_version: Option<String>,

    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is "OldestEmulationVersion"
    pub emulation_version: Option<String>,

    /// LeaseName is the name of the lease for which this candidate is contending. This field is immutable.
    pub lease_name: String,

    /// PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.
    pub ping_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// PreferredStrategies indicates the list of strategies for picking the leader for coordinated leader election. The list is ordered, and the first strategy supersedes all other strategies. The list is used by coordinated leader election to make a decision about the final election strategy. This follows as - If all clients have strategy X as the first element in this list, strategy X will be used. - If a candidate has strategy \[X\] and another candidate has strategy \[Y, X\], Y supersedes X and strategy Y
    ///   will be used.
    /// - If a candidate has strategy \[X, Y\] and another candidate has strategy \[Y, X\], this is a user error and leader
    ///   election will not operate the Lease until resolved.
    /// (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
    pub preferred_strategies: Vec<String>,

    /// RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.
    pub renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,
}

impl crate::DeepMerge for LeaseCandidateSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.binary_version, other.binary_version);
        crate::DeepMerge::merge_from(&mut self.emulation_version, other.emulation_version);
        crate::DeepMerge::merge_from(&mut self.lease_name, other.lease_name);
        crate::DeepMerge::merge_from(&mut self.ping_time, other.ping_time);
        crate::merge_strategies::list::atomic(&mut self.preferred_strategies, other.preferred_strategies);
        crate::DeepMerge::merge_from(&mut self.renew_time, other.renew_time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LeaseCandidateSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_binary_version,
            Key_emulation_version,
            Key_lease_name,
            Key_ping_time,
            Key_preferred_strategies,
            Key_renew_time,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "binaryVersion" => Field::Key_binary_version,
                            "emulationVersion" => Field::Key_emulation_version,
                            "leaseName" => Field::Key_lease_name,
                            "pingTime" => Field::Key_ping_time,
                            "preferredStrategies" => Field::Key_preferred_strategies,
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
            type Value = LeaseCandidateSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LeaseCandidateSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_binary_version: Option<String> = None;
                let mut value_emulation_version: Option<String> = None;
                let mut value_lease_name: Option<String> = None;
                let mut value_ping_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_preferred_strategies: Option<Vec<String>> = None;
                let mut value_renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_binary_version => value_binary_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_emulation_version => value_emulation_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_name => value_lease_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ping_time => value_ping_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_preferred_strategies => value_preferred_strategies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_renew_time => value_renew_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LeaseCandidateSpec {
                    binary_version: value_binary_version,
                    emulation_version: value_emulation_version,
                    lease_name: value_lease_name.unwrap_or_default(),
                    ping_time: value_ping_time,
                    preferred_strategies: value_preferred_strategies.unwrap_or_default(),
                    renew_time: value_renew_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "LeaseCandidateSpec",
            &[
                "binaryVersion",
                "emulationVersion",
                "leaseName",
                "pingTime",
                "preferredStrategies",
                "renewTime",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LeaseCandidateSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LeaseCandidateSpec",
            2 +
            self.binary_version.as_ref().map_or(0, |_| 1) +
            self.emulation_version.as_ref().map_or(0, |_| 1) +
            self.ping_time.as_ref().map_or(0, |_| 1) +
            self.renew_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.binary_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "binaryVersion", value)?;
        }
        if let Some(value) = &self.emulation_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "emulationVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "leaseName", &self.lease_name)?;
        if let Some(value) = &self.ping_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pingTime", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "preferredStrategies", &self.preferred_strategies)?;
        if let Some(value) = &self.renew_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "renewTime", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LeaseCandidateSpec {
    fn schema_name() -> String {
        "io.k8s.api.coordination.v1alpha1.LeaseCandidateSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LeaseCandidateSpec is a specification of a Lease.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "binaryVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required when strategy is \"OldestEmulationVersion\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "emulationVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is \"OldestEmulationVersion\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "leaseName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("LeaseName is the name of the lease for which this candidate is contending. This field is immutable.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pingTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "preferredStrategies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PreferredStrategies indicates the list of strategies for picking the leader for coordinated leader election. The list is ordered, and the first strategy supersedes all other strategies. The list is used by coordinated leader election to make a decision about the final election strategy. This follows as - If all clients have strategy X as the first element in this list, strategy X will be used. - If a candidate has strategy [X] and another candidate has strategy [Y, X], Y supersedes X and strategy Y\n  will be used.\n- If a candidate has strategy [X, Y] and another candidate has strategy [Y, X], this is a user error and leader\n  election will not operate the Lease until resolved.\n(Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "renewTime".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "leaseName".to_owned(),
                    "preferredStrategies".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
