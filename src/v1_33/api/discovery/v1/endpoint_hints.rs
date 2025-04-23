// Generated from definition io.k8s.api.discovery.v1.EndpointHints

/// EndpointHints provides hints describing how an endpoint should be consumed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointHints {
    /// forNodes indicates the node(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries. This is an Alpha feature and is only used when the PreferSameTrafficDistribution feature gate is enabled.
    pub for_nodes: Option<std::vec::Vec<crate::api::discovery::v1::ForNode>>,

    /// forZones indicates the zone(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries.
    pub for_zones: Option<std::vec::Vec<crate::api::discovery::v1::ForZone>>,
}

impl crate::DeepMerge for EndpointHints {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.for_nodes, other.for_nodes);
        crate::merge_strategies::list::atomic(&mut self.for_zones, other.for_zones);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EndpointHints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_for_nodes,
            Key_for_zones,
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
                            "forNodes" => Field::Key_for_nodes,
                            "forZones" => Field::Key_for_zones,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointHints;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EndpointHints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_for_nodes: Option<std::vec::Vec<crate::api::discovery::v1::ForNode>> = None;
                let mut value_for_zones: Option<std::vec::Vec<crate::api::discovery::v1::ForZone>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_for_nodes => value_for_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_for_zones => value_for_zones = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointHints {
                    for_nodes: value_for_nodes,
                    for_zones: value_for_zones,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointHints",
            &[
                "forNodes",
                "forZones",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointHints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointHints",
            self.for_nodes.as_ref().map_or(0, |_| 1) +
            self.for_zones.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.for_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "forNodes", value)?;
        }
        if let Some(value) = &self.for_zones {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "forZones", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EndpointHints {
    fn schema_name() -> std::string::String {
        "io.k8s.api.discovery.v1.EndpointHints".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("EndpointHints provides hints describing how an endpoint should be consumed.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "forNodes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("forNodes indicates the node(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries. This is an Alpha feature and is only used when the PreferSameTrafficDistribution feature gate is enabled.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::discovery::v1::ForNode>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "forZones".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("forZones indicates the zone(s) this endpoint should be consumed by when using topology aware routing. May contain a maximum of 8 entries.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::discovery::v1::ForZone>()))),
                                ..Default::default()
                            })),
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
