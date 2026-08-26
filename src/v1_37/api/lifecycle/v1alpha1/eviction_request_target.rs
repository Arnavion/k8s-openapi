// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionRequestTarget

/// EvictionRequestTarget contains a reference to an object that should be evicted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionRequestTarget {
    /// pod references a pod that is subject to eviction/termination. Pods that are part of a PodGroup (.spec.schedulingGroup is set) are not supported.
    pub pod: Option<crate::api::lifecycle::v1alpha1::EvictionRequestPodReference>,
}

impl crate::DeepMerge for EvictionRequestTarget {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.pod, other.pod);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionRequestTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod,
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
                            "pod" => Field::Key_pod,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionRequestTarget;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionRequestTarget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pod: Option<crate::api::lifecycle::v1alpha1::EvictionRequestPodReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod => value_pod = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionRequestTarget {
                    pod: value_pod,
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionRequestTarget",
            &[
                "pod",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionRequestTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionRequestTarget",
            self.pod.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pod", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionRequestTarget {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestTarget".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionRequestTarget contains a reference to an object that should be evicted.",
            "type": "object",
            "properties": {
                "pod": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionRequestPodReference>();
                    schema_obj.ensure_object().insert("description".into(), "pod references a pod that is subject to eviction/termination. Pods that are part of a PodGroup (.spec.schedulingGroup is set) are not supported.".into());
                    schema_obj
                }),
            },
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionRequestTarget {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionRequestTarget".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionRequestTarget contains a reference to an object that should be evicted.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "pod".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionRequestPodReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("pod references a pod that is subject to eviction/termination. Pods that are part of a PodGroup (.spec.schedulingGroup is set) are not supported.".into()),
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
