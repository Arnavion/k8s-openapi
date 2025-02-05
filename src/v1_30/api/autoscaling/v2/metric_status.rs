// Generated from definition io.k8s.api.autoscaling.v2.MetricStatus

/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricStatus {
    /// container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    pub container_resource: Option<crate::api::autoscaling::v2::ContainerResourceMetricStatus>,

    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    pub external: Option<crate::api::autoscaling::v2::ExternalMetricStatus>,

    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    pub object: Option<crate::api::autoscaling::v2::ObjectMetricStatus>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    pub pods: Option<crate::api::autoscaling::v2::PodsMetricStatus>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    pub resource: Option<crate::api::autoscaling::v2::ResourceMetricStatus>,

    /// type is the type of metric source.  It will be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object. Note: "ContainerResource" type is available on when the feature-gate HPAContainerMetrics is enabled
    pub type_: std::string::String,
}

impl crate::DeepMerge for MetricStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container_resource, other.container_resource);
        crate::DeepMerge::merge_from(&mut self.external, other.external);
        crate::DeepMerge::merge_from(&mut self.object, other.object);
        crate::DeepMerge::merge_from(&mut self.pods, other.pods);
        crate::DeepMerge::merge_from(&mut self.resource, other.resource);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for MetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_resource,
            Key_external,
            Key_object,
            Key_pods,
            Key_resource,
            Key_type_,
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
                            "containerResource" => Field::Key_container_resource,
                            "external" => Field::Key_external,
                            "object" => Field::Key_object,
                            "pods" => Field::Key_pods,
                            "resource" => Field::Key_resource,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MetricStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("MetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_resource: Option<crate::api::autoscaling::v2::ContainerResourceMetricStatus> = None;
                let mut value_external: Option<crate::api::autoscaling::v2::ExternalMetricStatus> = None;
                let mut value_object: Option<crate::api::autoscaling::v2::ObjectMetricStatus> = None;
                let mut value_pods: Option<crate::api::autoscaling::v2::PodsMetricStatus> = None;
                let mut value_resource: Option<crate::api::autoscaling::v2::ResourceMetricStatus> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_resource => value_container_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external => value_external = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pods => value_pods = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricStatus {
                    container_resource: value_container_resource,
                    external: value_external,
                    object: value_object,
                    pods: value_pods,
                    resource: value_resource,
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "MetricStatus",
            &[
                "containerResource",
                "external",
                "object",
                "pods",
                "resource",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricStatus",
            1 +
            self.container_resource.as_ref().map_or(0, |_| 1) +
            self.external.as_ref().map_or(0, |_| 1) +
            self.object.as_ref().map_or(0, |_| 1) +
            self.pods.as_ref().map_or(0, |_| 1) +
            self.resource.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerResource", value)?;
        }
        if let Some(value) = &self.external {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "external", value)?;
        }
        if let Some(value) = &self.object {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", value)?;
        }
        if let Some(value) = &self.pods {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pods", value)?;
        }
        if let Some(value) = &self.resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MetricStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.autoscaling.v2.MetricStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("MetricStatus describes the last-read state of a single metric.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerResource".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ContainerResourceMetricStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "external".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ExternalMetricStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "object".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ObjectMetricStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "pods".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::PodsMetricStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resource".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ResourceMetricStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("type is the type of metric source.  It will be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each corresponds to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "type".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
