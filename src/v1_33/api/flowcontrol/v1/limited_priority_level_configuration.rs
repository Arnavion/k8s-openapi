// Generated from definition io.k8s.api.flowcontrol.v1.LimitedPriorityLevelConfiguration

/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits. It addresses two issues:
///   - How are requests for this priority level limited?
///   - What should be done with requests that exceed the limit?
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LimitedPriorityLevelConfiguration {
    /// `borrowingLimitPercent`, if present, configures a limit on how many seats this priority level can borrow from other priority levels. The limit is known as this level's BorrowingConcurrencyLimit (BorrowingCL) and is a limit on the total number of seats that this level may borrow at any one time. This field holds the ratio of that limit to the level's nominal concurrency limit. When this field is non-nil, it must hold a non-negative integer and the limit is calculated as follows.
    ///
    /// BorrowingCL(i) = round( NominalCL(i) * borrowingLimitPercent(i)/100.0 )
    ///
    /// The value of this field can be more than 100, implying that this priority level can borrow a number of seats that is greater than its own nominal concurrency limit (NominalCL). When this field is left `nil`, the limit is effectively infinite.
    pub borrowing_limit_percent: Option<i32>,

    /// `lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels. The value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    pub lendable_percent: Option<i32>,

    /// `limitResponse` indicates what to do with requests that can not be executed right now
    pub limit_response: Option<crate::api::flowcontrol::v1::LimitResponse>,

    /// `nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats available at this priority level. This is used both for requests dispatched from this priority level as well as requests dispatched from other priority levels borrowing seats from this level. The server's concurrency limit (ServerCL) is divided among the Limited priority levels in proportion to their NCS values:
    ///
    /// NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum\[priority level k\] NCS(k)
    ///
    /// Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level.
    ///
    /// If not specified, this field defaults to a value of 30.
    ///
    /// Setting this field to zero supports the construction of a "jail" for this priority level that is used to hold some request(s)
    pub nominal_concurrency_shares: Option<i32>,
}

impl crate::DeepMerge for LimitedPriorityLevelConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.borrowing_limit_percent, other.borrowing_limit_percent);
        crate::DeepMerge::merge_from(&mut self.lendable_percent, other.lendable_percent);
        crate::DeepMerge::merge_from(&mut self.limit_response, other.limit_response);
        crate::DeepMerge::merge_from(&mut self.nominal_concurrency_shares, other.nominal_concurrency_shares);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LimitedPriorityLevelConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_borrowing_limit_percent,
            Key_lendable_percent,
            Key_limit_response,
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
                            "borrowingLimitPercent" => Field::Key_borrowing_limit_percent,
                            "lendablePercent" => Field::Key_lendable_percent,
                            "limitResponse" => Field::Key_limit_response,
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
            type Value = LimitedPriorityLevelConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LimitedPriorityLevelConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_borrowing_limit_percent: Option<i32> = None;
                let mut value_lendable_percent: Option<i32> = None;
                let mut value_limit_response: Option<crate::api::flowcontrol::v1::LimitResponse> = None;
                let mut value_nominal_concurrency_shares: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_borrowing_limit_percent => value_borrowing_limit_percent = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lendable_percent => value_lendable_percent = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_limit_response => value_limit_response = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nominal_concurrency_shares => value_nominal_concurrency_shares = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LimitedPriorityLevelConfiguration {
                    borrowing_limit_percent: value_borrowing_limit_percent,
                    lendable_percent: value_lendable_percent,
                    limit_response: value_limit_response,
                    nominal_concurrency_shares: value_nominal_concurrency_shares,
                })
            }
        }

        deserializer.deserialize_struct(
            "LimitedPriorityLevelConfiguration",
            &[
                "borrowingLimitPercent",
                "lendablePercent",
                "limitResponse",
                "nominalConcurrencyShares",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LimitedPriorityLevelConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LimitedPriorityLevelConfiguration",
            self.borrowing_limit_percent.as_ref().map_or(0, |_| 1) +
            self.lendable_percent.as_ref().map_or(0, |_| 1) +
            self.limit_response.as_ref().map_or(0, |_| 1) +
            self.nominal_concurrency_shares.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.borrowing_limit_percent {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "borrowingLimitPercent", value)?;
        }
        if let Some(value) = &self.lendable_percent {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lendablePercent", value)?;
        }
        if let Some(value) = &self.limit_response {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "limitResponse", value)?;
        }
        if let Some(value) = &self.nominal_concurrency_shares {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nominalConcurrencyShares", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LimitedPriorityLevelConfiguration {
    fn schema_name() -> std::string::String {
        "io.k8s.api.flowcontrol.v1.LimitedPriorityLevelConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits. It addresses two issues:\n  - How are requests for this priority level limited?\n  - What should be done with requests that exceed the limit?".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "borrowingLimitPercent".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`borrowingLimitPercent`, if present, configures a limit on how many seats this priority level can borrow from other priority levels. The limit is known as this level's BorrowingConcurrencyLimit (BorrowingCL) and is a limit on the total number of seats that this level may borrow at any one time. This field holds the ratio of that limit to the level's nominal concurrency limit. When this field is non-nil, it must hold a non-negative integer and the limit is calculated as follows.\n\nBorrowingCL(i) = round( NominalCL(i) * borrowingLimitPercent(i)/100.0 )\n\nThe value of this field can be more than 100, implying that this priority level can borrow a number of seats that is greater than its own nominal concurrency limit (NominalCL). When this field is left `nil`, the limit is effectively infinite.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lendablePercent".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels. The value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.\n\nLendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "limitResponse".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1::LimitResponse>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`limitResponse` indicates what to do with requests that can not be executed right now".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nominalConcurrencyShares".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats available at this priority level. This is used both for requests dispatched from this priority level as well as requests dispatched from other priority levels borrowing seats from this level. The server's concurrency limit (ServerCL) is divided among the Limited priority levels in proportion to their NCS values:\n\nNominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum[priority level k] NCS(k)\n\nBigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level.\n\nIf not specified, this field defaults to a value of 30.\n\nSetting this field to zero supports the construction of a \"jail\" for this priority level that is used to hold some request(s)".into()),
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
