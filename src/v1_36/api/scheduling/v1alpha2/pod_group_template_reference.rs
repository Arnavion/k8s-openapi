// Generated from definition io.k8s.api.scheduling.v1alpha2.PodGroupTemplateReference

/// PodGroupTemplateReference references a PodGroup template defined in some object (e.g. Workload). Exactly one reference must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodGroupTemplateReference {
    /// Workload references the PodGroupTemplate within the Workload object that was used to create the PodGroup.
    pub workload: Option<crate::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference>,
}

impl crate::DeepMerge for PodGroupTemplateReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.workload, other.workload);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodGroupTemplateReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_workload,
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
                            "workload" => Field::Key_workload,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodGroupTemplateReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodGroupTemplateReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_workload: Option<crate::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_workload => value_workload = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodGroupTemplateReference {
                    workload: value_workload,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodGroupTemplateReference",
            &[
                "workload",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodGroupTemplateReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodGroupTemplateReference",
            self.workload.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.workload {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workload", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodGroupTemplateReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha2.PodGroupTemplateReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodGroupTemplateReference references a PodGroup template defined in some object (e.g. Workload). Exactly one reference must be set.",
            "type": "object",
            "properties": {
                "workload": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference>();
                    schema_obj.ensure_object().insert("description".into(), "Workload references the PodGroupTemplate within the Workload object that was used to create the PodGroup.".into());
                    schema_obj
                }),
            },
        })
    }
}
