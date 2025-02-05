// Generated from definition io.k8s.api.autoscaling.v2.ContainerResourceMetricStatus

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerResourceMetricStatus {
    /// container is the name of the container in the pods of the scaling target
    pub container: std::string::String,

    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2::MetricValueStatus,

    /// name is the name of the resource in question.
    pub name: std::string::String,
}

impl crate::DeepMerge for ContainerResourceMetricStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container, other.container);
        crate::DeepMerge::merge_from(&mut self.current, other.current);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerResourceMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container,
            Key_current,
            Key_name,
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
                            "container" => Field::Key_container,
                            "current" => Field::Key_current,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerResourceMetricStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerResourceMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container: Option<std::string::String> = None;
                let mut value_current: Option<crate::api::autoscaling::v2::MetricValueStatus> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container => value_container = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current => value_current = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerResourceMetricStatus {
                    container: value_container.unwrap_or_default(),
                    current: value_current.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerResourceMetricStatus",
            &[
                "container",
                "current",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerResourceMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerResourceMetricStatus",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "container", &self.container)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerResourceMetricStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.ContainerResourceMetricStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "container".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("container is the name of the container in the pods of the scaling target".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "current".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::MetricValueStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("current contains the current value for the given metric".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("name is the name of the resource in question.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "container".into(),
                    "current".into(),
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
