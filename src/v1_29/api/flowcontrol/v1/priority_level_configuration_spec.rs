// Generated from definition io.k8s.api.flowcontrol.v1.PriorityLevelConfigurationSpec

/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityLevelConfigurationSpec {
    /// `exempt` specifies how requests are handled for an exempt priority level. This field MUST be empty if `type` is `"Limited"`. This field MAY be non-empty if `type` is `"Exempt"`. If empty and `type` is `"Exempt"` then the default values for `ExemptPriorityLevelConfiguration` apply.
    pub exempt: Option<crate::api::flowcontrol::v1::ExemptPriorityLevelConfiguration>,

    /// `limited` specifies how requests are handled for a Limited priority level. This field must be non-empty if and only if `type` is `"Limited"`.
    pub limited: Option<crate::api::flowcontrol::v1::LimitedPriorityLevelConfiguration>,

    /// `type` indicates whether this priority level is subject to limitation on request execution.  A value of `"Exempt"` means that requests of this priority level are not subject to a limit (and thus are never queued) and do not detract from the capacity made available to other priority levels.  A value of `"Limited"` means that (a) requests of this priority level _are_ subject to limits and (b) some of the server's limited capacity is made available exclusively to this priority level. Required.
    pub type_: std::string::String,
}

impl crate::DeepMerge for PriorityLevelConfigurationSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.exempt, other.exempt);
        crate::DeepMerge::merge_from(&mut self.limited, other.limited);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PriorityLevelConfigurationSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exempt,
            Key_limited,
            Key_type_,
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
                            "exempt" => Field::Key_exempt,
                            "limited" => Field::Key_limited,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PriorityLevelConfigurationSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PriorityLevelConfigurationSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exempt: Option<crate::api::flowcontrol::v1::ExemptPriorityLevelConfiguration> = None;
                let mut value_limited: Option<crate::api::flowcontrol::v1::LimitedPriorityLevelConfiguration> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exempt => value_exempt = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_limited => value_limited = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityLevelConfigurationSpec {
                    exempt: value_exempt,
                    limited: value_limited,
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityLevelConfigurationSpec",
            &[
                "exempt",
                "limited",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PriorityLevelConfigurationSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityLevelConfigurationSpec",
            1 +
            self.exempt.as_ref().map_or(0, |_| 1) +
            self.limited.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exempt {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exempt", value)?;
        }
        if let Some(value) = &self.limited {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "limited", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PriorityLevelConfigurationSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.flowcontrol.v1.PriorityLevelConfigurationSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PriorityLevelConfigurationSpec specifies the configuration of a priority level.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exempt".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1::ExemptPriorityLevelConfiguration>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`exempt` specifies how requests are handled for an exempt priority level. This field MUST be empty if `type` is `\"Limited\"`. This field MAY be non-empty if `type` is `\"Exempt\"`. If empty and `type` is `\"Exempt\"` then the default values for `ExemptPriorityLevelConfiguration` apply.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "limited".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::flowcontrol::v1::LimitedPriorityLevelConfiguration>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`limited` specifies how requests are handled for a Limited priority level. This field must be non-empty if and only if `type` is `\"Limited\"`.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`type` indicates whether this priority level is subject to limitation on request execution.  A value of `\"Exempt\"` means that requests of this priority level are not subject to a limit (and thus are never queued) and do not detract from the capacity made available to other priority levels.  A value of `\"Limited\"` means that (a) requests of this priority level _are_ subject to limits and (b) some of the server's limited capacity is made available exclusively to this priority level. Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "type".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
