// Generated from definition io.k8s.api.scheduling.v1alpha3.WorkloadReference

/// WorkloadReference references the Workload object together with the template that was used to create a particular PodGroup.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkloadReference {
    /// templateName is the name of a template within the Workload object that was used to create a pod group. It must be a DNS label. This field is required.
    pub template_name: std::string::String,

    /// workloadName is the name of the Workload object that contains a template that was used when creating a pod group. It must be a DNS name. This field is required.
    pub workload_name: std::string::String,
}

impl crate::DeepMerge for WorkloadReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.template_name, other.template_name);
        crate::DeepMerge::merge_from(&mut self.workload_name, other.workload_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WorkloadReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_template_name,
            Key_workload_name,
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
                            "templateName" => Field::Key_template_name,
                            "workloadName" => Field::Key_workload_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WorkloadReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WorkloadReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_template_name: Option<std::string::String> = None;
                let mut value_workload_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_template_name => value_template_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_workload_name => value_workload_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WorkloadReference {
                    template_name: value_template_name.unwrap_or_default(),
                    workload_name: value_workload_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "WorkloadReference",
            &[
                "templateName",
                "workloadName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WorkloadReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WorkloadReference",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "templateName", &self.template_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workloadName", &self.workload_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WorkloadReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.scheduling.v1alpha3.WorkloadReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "WorkloadReference references the Workload object together with the template that was used to create a particular PodGroup.",
            "type": "object",
            "properties": {
                "templateName": {
                    "description": "templateName is the name of a template within the Workload object that was used to create a pod group. It must be a DNS label. This field is required.",
                    "type": "string",
                },
                "workloadName": {
                    "description": "workloadName is the name of the Workload object that contains a template that was used when creating a pod group. It must be a DNS name. This field is required.",
                    "type": "string",
                },
            },
            "required": [
                "templateName",
                "workloadName",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for WorkloadReference {
    fn schema_name() -> std::string::String {
        "io.k8s.api.scheduling.v1alpha3.WorkloadReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("WorkloadReference references the Workload object together with the template that was used to create a particular PodGroup.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "templateName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("templateName is the name of a template within the Workload object that was used to create a pod group. It must be a DNS label. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "workloadName".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("workloadName is the name of the Workload object that contains a template that was used when creating a pod group. It must be a DNS name. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "templateName".into(),
                    "workloadName".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
