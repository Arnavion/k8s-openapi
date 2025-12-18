// Generated from definition io.k8s.api.scheduling.v1alpha1.WorkloadSpec

/// WorkloadSpec defines the desired state of a Workload.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadSpec {
    /// ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.
    pub controller_ref: Option<crate::api::scheduling::v1alpha1::TypedLocalObjectReference>,

    /// PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.
    pub pod_groups: std::vec::Vec<crate::api::scheduling::v1alpha1::PodGroup>,
}

impl crate::DeepMerge for WorkloadSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.controller_ref, other.controller_ref);
        crate::merge_strategies::list::map(
            &mut self.pod_groups,
            other.pod_groups,
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
            Key_controller_ref,
            Key_pod_groups,
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
                            "controllerRef" => Field::Key_controller_ref,
                            "podGroups" => Field::Key_pod_groups,
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
                let mut value_controller_ref: Option<crate::api::scheduling::v1alpha1::TypedLocalObjectReference> = None;
                let mut value_pod_groups: Option<std::vec::Vec<crate::api::scheduling::v1alpha1::PodGroup>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller_ref => value_controller_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_groups => value_pod_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadSpec {
                    controller_ref: value_controller_ref,
                    pod_groups: value_pod_groups.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadSpec",
            &[
                "controllerRef",
                "podGroups",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadSpec",
            1 +
            self.controller_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controllerRef", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podGroups", &self.pod_groups)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha1.WorkloadSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadSpec defines the desired state of a Workload.",
            "type": "object",
            "properties": {
                "controllerRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::scheduling::v1alpha1::TypedLocalObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.".into());
                    schema_obj
                }),
                "podGroups": {
                    "description": "PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::scheduling::v1alpha1::PodGroup>()),
                },
            },
            "required": [
                "podGroups",
            ],
        })
    }
}
