// Generated from definition io.k8s.api.scheduling.v1alpha2.PodGroupSchedulingPolicy

/// PodGroupSchedulingPolicy defines the scheduling configuration for a PodGroup. Exactly one policy must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupSchedulingPolicy {
    /// Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.
    pub basic: Option<crate::api::scheduling::v1alpha2::BasicSchedulingPolicy>,

    /// Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.
    pub gang: Option<crate::api::scheduling::v1alpha2::GangSchedulingPolicy>,
}

impl crate::DeepMerge for PodGroupSchedulingPolicy {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.basic, other.basic);
        crate::DeepMerge::merge_from(&mut self.gang, other.gang);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupSchedulingPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_basic,
            Key_gang,
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
                            "basic" => Field::Key_basic,
                            "gang" => Field::Key_gang,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodGroupSchedulingPolicy;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupSchedulingPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_basic: Option<crate::api::scheduling::v1alpha2::BasicSchedulingPolicy> = None;
                let mut value_gang: Option<crate::api::scheduling::v1alpha2::GangSchedulingPolicy> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_basic => value_basic = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gang => value_gang = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupSchedulingPolicy {
                    basic: value_basic,
                    gang: value_gang,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupSchedulingPolicy",
            &[
                "basic",
                "gang",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodGroupSchedulingPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodGroupSchedulingPolicy",
            self.basic.as_ref().map_or(0, |_| 1) +
            self.gang.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.basic {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "basic", value)?;
        }
        if let Some(value) = &self.gang {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gang", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupSchedulingPolicy {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha2.PodGroupSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupSchedulingPolicy defines the scheduling configuration for a PodGroup. Exactly one policy must be set.",
            "type": "object",
            "properties": {
                "basic": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::BasicSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.".into());
                    schema_obj
                }),
                "gang": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::GangSchedulingPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for PodGroupSchedulingPolicy {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha2.PodGroupSchedulingPolicy".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("PodGroupSchedulingPolicy defines the scheduling configuration for a PodGroup. Exactly one policy must be set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "basic".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::BasicSchedulingPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "gang".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::GangSchedulingPolicy>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
