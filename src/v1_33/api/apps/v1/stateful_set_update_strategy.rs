// Generated from definition io.k8s.api.apps.v1.StatefulSetUpdateStrategy

/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetUpdateStrategy {
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    pub rolling_update: Option<crate::api::apps::v1::RollingUpdateStatefulSetStrategy>,

    /// Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
    pub type_: Option<std::string::String>,
}

impl crate::DeepMerge for StatefulSetUpdateStrategy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.rolling_update, other.rolling_update);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StatefulSetUpdateStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rolling_update,
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
                            "rollingUpdate" => Field::Key_rolling_update,
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
            type Value = StatefulSetUpdateStrategy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StatefulSetUpdateStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_rolling_update: Option<crate::api::apps::v1::RollingUpdateStatefulSetStrategy> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rolling_update => value_rolling_update = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetUpdateStrategy {
                    rolling_update: value_rolling_update,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetUpdateStrategy",
            &[
                "rollingUpdate",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetUpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetUpdateStrategy",
            self.rolling_update.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.rolling_update {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rollingUpdate", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetUpdateStrategy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.apps.v1.StatefulSetUpdateStrategy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "rollingUpdate".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::RollingUpdateStatefulSetStrategy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
