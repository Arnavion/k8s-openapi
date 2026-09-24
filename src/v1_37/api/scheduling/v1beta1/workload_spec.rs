// Generated from definition io.k8s.api.scheduling.v1beta1.WorkloadSpec

/// WorkloadSpec defines the desired state of a Workload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadSpec {
    /// compositePodGroupTemplates is the list of CompositePodGroup templates that make up the Workload. The maximum number of templates is 8. This field is immutable. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.
    ///
    /// This field is used only when the CompositePodGroup feature gate is enabled.
    pub composite_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1beta1::CompositePodGroupTemplate>>,

    /// controllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. This field is immutable.
    pub controller_ref: Option<crate::api::scheduling::v1beta1::TypedLocalObjectReference>,

    /// podGroupTemplates is the list of templates that make up the Workload. The maximum number of templates is 8. Templates cannot be added or removed after the workload is created. Existing templates may still be updated where their individual fields allow it. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.
    pub pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1beta1::PodGroupTemplate>>,
}

impl crate::DeepMerge for WorkloadSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.composite_pod_group_templates,
            other.composite_pod_group_templates,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.controller_ref, other.controller_ref);
        crate::merge_strategies::list::map(
            &mut self.pod_group_templates,
            other.pod_group_templates,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_composite_pod_group_templates,
            Key_controller_ref,
            Key_pod_group_templates,
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
                            "compositePodGroupTemplates" => Field::Key_composite_pod_group_templates,
                            "controllerRef" => Field::Key_controller_ref,
                            "podGroupTemplates" => Field::Key_pod_group_templates,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_composite_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1beta1::CompositePodGroupTemplate>> = None;
                let mut value_controller_ref: Option<crate::api::scheduling::v1beta1::TypedLocalObjectReference> = None;
                let mut value_pod_group_templates: Option<std::vec::Vec<crate::api::scheduling::v1beta1::PodGroupTemplate>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_composite_pod_group_templates => value_composite_pod_group_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller_ref => value_controller_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_group_templates => value_pod_group_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadSpec {
                    composite_pod_group_templates: value_composite_pod_group_templates,
                    controller_ref: value_controller_ref,
                    pod_group_templates: value_pod_group_templates,
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadSpec",
            &[
                "compositePodGroupTemplates",
                "controllerRef",
                "podGroupTemplates",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadSpec",
            self.composite_pod_group_templates.as_ref().map_or(0, |_| 1) +
            self.controller_ref.as_ref().map_or(0, |_| 1) +
            self.pod_group_templates.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.composite_pod_group_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "compositePodGroupTemplates", value)?;
        }
        if let Some(value) = &self.controller_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controllerRef", value)?;
        }
        if let Some(value) = &self.pod_group_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroupTemplates", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1beta1.WorkloadSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadSpec defines the desired state of a Workload.",
            "type": "object",
            "properties": {
                "compositePodGroupTemplates": {
                    "description": "compositePodGroupTemplates is the list of CompositePodGroup templates that make up the Workload. The maximum number of templates is 8. This field is immutable. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.\n\nThis field is used only when the CompositePodGroup feature gate is enabled.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1beta1::CompositePodGroupTemplate>()),
                },
                "controllerRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1beta1::TypedLocalObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "controllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. This field is immutable.".into());
                    schema_obj
                }),
                "podGroupTemplates": {
                    "description": "podGroupTemplates is the list of templates that make up the Workload. The maximum number of templates is 8. Templates cannot be added or removed after the workload is created. Existing templates may still be updated where their individual fields allow it. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1beta1::PodGroupTemplate>()),
                },
            },
        })
    }
}
