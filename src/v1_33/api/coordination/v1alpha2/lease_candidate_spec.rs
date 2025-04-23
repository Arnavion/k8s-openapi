// Generated from definition io.k8s.api.coordination.v1alpha2.LeaseCandidateSpec

/// LeaseCandidateSpec is a specification of a Lease.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeaseCandidateSpec {
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required.
    pub binary_version: std::string::String,

    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is "OldestEmulationVersion"
    pub emulation_version: Option<std::string::String>,

    /// LeaseName is the name of the lease for which this candidate is contending. This field is immutable.
    pub lease_name: std::string::String,

    /// PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.
    pub ping_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.
    pub renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime>,

    /// Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the strategy provided by the candidate with the latest BinaryVersion will be used. If there is still conflict, this is a user error and coordinated leader election will not operate the Lease until resolved.
    pub strategy: std::string::String,
}

impl crate::DeepMerge for LeaseCandidateSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.binary_version, other.binary_version);
        crate::DeepMerge::merge_from(&mut self.emulation_version, other.emulation_version);
        crate::DeepMerge::merge_from(&mut self.lease_name, other.lease_name);
        crate::DeepMerge::merge_from(&mut self.ping_time, other.ping_time);
        crate::DeepMerge::merge_from(&mut self.renew_time, other.renew_time);
        crate::DeepMerge::merge_from(&mut self.strategy, other.strategy);
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
            Key_renew_time,
            Key_strategy,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "binaryVersion" => Field::Key_binary_version,
                            "emulationVersion" => Field::Key_emulation_version,
                            "leaseName" => Field::Key_lease_name,
                            "pingTime" => Field::Key_ping_time,
                            "renewTime" => Field::Key_renew_time,
                            "strategy" => Field::Key_strategy,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LeaseCandidateSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_binary_version: Option<std::string::String> = None;
                let mut value_emulation_version: Option<std::string::String> = None;
                let mut value_lease_name: Option<std::string::String> = None;
                let mut value_ping_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_renew_time: Option<crate::apimachinery::pkg::apis::meta::v1::MicroTime> = None;
                let mut value_strategy: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_binary_version => value_binary_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_emulation_version => value_emulation_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lease_name => value_lease_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ping_time => value_ping_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_renew_time => value_renew_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LeaseCandidateSpec {
                    binary_version: value_binary_version.unwrap_or_default(),
                    emulation_version: value_emulation_version,
                    lease_name: value_lease_name.unwrap_or_default(),
                    ping_time: value_ping_time,
                    renew_time: value_renew_time,
                    strategy: value_strategy.unwrap_or_default(),
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
                "renewTime",
                "strategy",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LeaseCandidateSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LeaseCandidateSpec",
            3 +
            self.emulation_version.as_ref().map_or(0, |_| 1) +
            self.ping_time.as_ref().map_or(0, |_| 1) +
            self.renew_time.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "binaryVersion", &self.binary_version)?;
        if let Some(value) = &self.emulation_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "emulationVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "leaseName", &self.lease_name)?;
        if let Some(value) = &self.ping_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pingTime", value)?;
        }
        if let Some(value) = &self.renew_time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "renewTime", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LeaseCandidateSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.coordination.v1alpha2.LeaseCandidateSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("LeaseCandidateSpec is a specification of a Lease.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "binaryVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "emulationVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is \"OldestEmulationVersion\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "leaseName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("LeaseName is the name of the lease for which this candidate is contending. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pingTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "renewTime".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::MicroTime>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "strategy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the strategy provided by the candidate with the latest BinaryVersion will be used. If there is still conflict, this is a user error and coordinated leader election will not operate the Lease until resolved.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "binaryVersion".into(),
                    "leaseName".into(),
                    "strategy".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
