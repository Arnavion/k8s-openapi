// Generated from definition io.k8s.api.flowcontrol.v1.ExemptPriorityLevelConfiguration

/// ExemptPriorityLevelConfiguration describes the configurable aspects of the handling of exempt requests. In the mandatory exempt configuration object the values in the fields here can be modified by authorized users, unlike the rest of the `spec`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExemptPriorityLevelConfiguration {
    /// `lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels.  This value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    pub lendable_percent: Option<i32>,

    /// `nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats nominally reserved for this priority level. This DOES NOT limit the dispatching from this priority level but affects the other priority levels through the borrowing mechanism. The server's concurrency limit (ServerCL) is divided among all the priority levels in proportion to their NCS values:
    ///
    /// NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum\[priority level k\] NCS(k)
    ///
    /// Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level. This field has a default value of zero.
    pub nominal_concurrency_shares: Option<i32>,
}

impl crate::DeepMerge for ExemptPriorityLevelConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.lendable_percent, other.lendable_percent);
        crate::DeepMerge::merge_from(&mut self.nominal_concurrency_shares, other.nominal_concurrency_shares);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ExemptPriorityLevelConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_lendable_percent,
            Key_nominal_concurrency_shares,
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
                            "lendablePercent" => Field::Key_lendable_percent,
                            "nominalConcurrencyShares" => Field::Key_nominal_concurrency_shares,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ExemptPriorityLevelConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ExemptPriorityLevelConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_lendable_percent: Option<i32> = None;
                let mut value_nominal_concurrency_shares: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_lendable_percent => value_lendable_percent = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nominal_concurrency_shares => value_nominal_concurrency_shares = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExemptPriorityLevelConfiguration {
                    lendable_percent: value_lendable_percent,
                    nominal_concurrency_shares: value_nominal_concurrency_shares,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExemptPriorityLevelConfiguration",
            &[
                "lendablePercent",
                "nominalConcurrencyShares",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ExemptPriorityLevelConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExemptPriorityLevelConfiguration",
            self.lendable_percent.as_ref().map_or(0, |_| 1) +
            self.nominal_concurrency_shares.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.lendable_percent {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lendablePercent", value)?;
        }
        if let Some(value) = &self.nominal_concurrency_shares {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nominalConcurrencyShares", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ExemptPriorityLevelConfiguration {
    fn schema_name() -> std::string::String {
        "io.k8s.api.flowcontrol.v1.ExemptPriorityLevelConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ExemptPriorityLevelConfiguration describes the configurable aspects of the handling of exempt requests. In the mandatory exempt configuration object the values in the fields here can be modified by authorized users, unlike the rest of the `spec`.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "lendablePercent".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels.  This value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.\n\nLendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nominalConcurrencyShares".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats nominally reserved for this priority level. This DOES NOT limit the dispatching from this priority level but affects the other priority levels through the borrowing mechanism. The server's concurrency limit (ServerCL) is divided among all the priority levels in proportion to their NCS values:\n\nNominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum[priority level k] NCS(k)\n\nBigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level. This field has a default value of zero.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
