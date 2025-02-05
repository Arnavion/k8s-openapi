// Generated from definition io.k8s.api.core.v1.WeightedPodAffinityTerm

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeightedPodAffinityTerm {
    /// Required. A pod affinity term, associated with the corresponding weight.
    pub pod_affinity_term: crate::api::core::v1::PodAffinityTerm,

    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    pub weight: i32,
}

impl crate::DeepMerge for WeightedPodAffinityTerm {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.pod_affinity_term, other.pod_affinity_term);
        crate::DeepMerge::merge_from(&mut self.weight, other.weight);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WeightedPodAffinityTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_affinity_term,
            Key_weight,
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
                            "podAffinityTerm" => Field::Key_pod_affinity_term,
                            "weight" => Field::Key_weight,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WeightedPodAffinityTerm;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WeightedPodAffinityTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pod_affinity_term: Option<crate::api::core::v1::PodAffinityTerm> = None;
                let mut value_weight: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_affinity_term => value_pod_affinity_term = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_weight => value_weight = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WeightedPodAffinityTerm {
                    pod_affinity_term: value_pod_affinity_term.unwrap_or_default(),
                    weight: value_weight.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "WeightedPodAffinityTerm",
            &[
                "podAffinityTerm",
                "weight",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WeightedPodAffinityTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WeightedPodAffinityTerm",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podAffinityTerm", &self.pod_affinity_term)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "weight", &self.weight)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WeightedPodAffinityTerm {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.WeightedPodAffinityTerm".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "podAffinityTerm".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodAffinityTerm>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required. A pod affinity term, associated with the corresponding weight.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "weight".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("weight associated with matching the corresponding podAffinityTerm, in the range 1-100.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "podAffinityTerm".into(),
                    "weight".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
