// Generated from definition io.k8s.api.apps.v1.StatefulSetOrdinals

/// StatefulSetOrdinals describes the policy used for replica ordinal assignment in this StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetOrdinals {
    /// start is the number representing the first replica's index. It may be used to number replicas from an alternate index (eg: 1-indexed) over the default 0-indexed names, or to orchestrate progressive movement of replicas from one StatefulSet to another. If set, replica indices will be in the range:
    ///   \[.spec.ordinals.start, .spec.ordinals.start + .spec.replicas).
    /// If unset, defaults to 0. Replica indices will be in the range:
    ///   \[0, .spec.replicas).
    pub start: Option<i32>,
}

impl crate::DeepMerge for StatefulSetOrdinals {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.start, other.start);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StatefulSetOrdinals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_start,
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
                            "start" => Field::Key_start,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetOrdinals;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StatefulSetOrdinals")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_start: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_start => value_start = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetOrdinals {
                    start: value_start,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetOrdinals",
            &[
                "start",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetOrdinals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetOrdinals",
            self.start.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.start {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "start", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetOrdinals {
    fn schema_name() -> std::string::String {
        "io.k8s.api.apps.v1.StatefulSetOrdinals".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("StatefulSetOrdinals describes the policy used for replica ordinal assignment in this StatefulSet.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "start".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("start is the number representing the first replica's index. It may be used to number replicas from an alternate index (eg: 1-indexed) over the default 0-indexed names, or to orchestrate progressive movement of replicas from one StatefulSet to another. If set, replica indices will be in the range:\n  [.spec.ordinals.start, .spec.ordinals.start + .spec.replicas).\nIf unset, defaults to 0. Replica indices will be in the range:\n  [0, .spec.replicas).".into()),
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
