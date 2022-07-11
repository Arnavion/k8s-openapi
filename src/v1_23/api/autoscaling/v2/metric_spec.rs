// Generated from definition io.k8s.api.autoscaling.v2.MetricSpec

/// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetricSpec {
    /// containerResource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source. This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.
    pub container_resource: Option<crate::api::autoscaling::v2::ContainerResourceMetricSource>,

    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    pub external: Option<crate::api::autoscaling::v2::ExternalMetricSource>,

    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    pub object: Option<crate::api::autoscaling::v2::ObjectMetricSource>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    pub pods: Option<crate::api::autoscaling::v2::PodsMetricSource>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    pub resource: Option<crate::api::autoscaling::v2::ResourceMetricSource>,

    /// type is the type of metric source.  It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object. Note: "ContainerResource" type is available on when the feature-gate HPAContainerMetrics is enabled
    pub type_: String,

}

#[cfg(feature = "dsl")]
impl MetricSpec  {
    /// Set [`Self::container_resource`]
    pub  fn container_resource_set(&mut self, container_resource: impl Into<Option<crate::api::autoscaling::v2::ContainerResourceMetricSource>>) -> &mut Self {
        self.container_resource = container_resource.into(); self
    }

    pub  fn container_resource(&mut self) -> &mut crate::api::autoscaling::v2::ContainerResourceMetricSource {
        if self.container_resource.is_none() { self.container_resource = Some(Default::default()) }
        self.container_resource.as_mut().unwrap()
    }

    /// Modify [`Self::container_resource`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn container_resource_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2::ContainerResourceMetricSource)) -> &mut Self {
        if self.container_resource.is_none() { self.container_resource = Some(Default::default()) };
        func(self.container_resource.as_mut().unwrap()); self
    }


    /// Set [`Self::external`]
    pub  fn external_set(&mut self, external: impl Into<Option<crate::api::autoscaling::v2::ExternalMetricSource>>) -> &mut Self {
        self.external = external.into(); self
    }

    pub  fn external(&mut self) -> &mut crate::api::autoscaling::v2::ExternalMetricSource {
        if self.external.is_none() { self.external = Some(Default::default()) }
        self.external.as_mut().unwrap()
    }

    /// Modify [`Self::external`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn external_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2::ExternalMetricSource)) -> &mut Self {
        if self.external.is_none() { self.external = Some(Default::default()) };
        func(self.external.as_mut().unwrap()); self
    }


    /// Set [`Self::object`]
    pub  fn object_set(&mut self, object: impl Into<Option<crate::api::autoscaling::v2::ObjectMetricSource>>) -> &mut Self {
        self.object = object.into(); self
    }

    pub  fn object(&mut self) -> &mut crate::api::autoscaling::v2::ObjectMetricSource {
        if self.object.is_none() { self.object = Some(Default::default()) }
        self.object.as_mut().unwrap()
    }

    /// Modify [`Self::object`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn object_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2::ObjectMetricSource)) -> &mut Self {
        if self.object.is_none() { self.object = Some(Default::default()) };
        func(self.object.as_mut().unwrap()); self
    }


    /// Set [`Self::pods`]
    pub  fn pods_set(&mut self, pods: impl Into<Option<crate::api::autoscaling::v2::PodsMetricSource>>) -> &mut Self {
        self.pods = pods.into(); self
    }

    pub  fn pods(&mut self) -> &mut crate::api::autoscaling::v2::PodsMetricSource {
        if self.pods.is_none() { self.pods = Some(Default::default()) }
        self.pods.as_mut().unwrap()
    }

    /// Modify [`Self::pods`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pods_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2::PodsMetricSource)) -> &mut Self {
        if self.pods.is_none() { self.pods = Some(Default::default()) };
        func(self.pods.as_mut().unwrap()); self
    }


    /// Set [`Self::resource`]
    pub  fn resource_set(&mut self, resource: impl Into<Option<crate::api::autoscaling::v2::ResourceMetricSource>>) -> &mut Self {
        self.resource = resource.into(); self
    }

    pub  fn resource(&mut self) -> &mut crate::api::autoscaling::v2::ResourceMetricSource {
        if self.resource.is_none() { self.resource = Some(Default::default()) }
        self.resource.as_mut().unwrap()
    }

    /// Modify [`Self::resource`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2::ResourceMetricSource)) -> &mut Self {
        if self.resource.is_none() { self.resource = Some(Default::default()) };
        func(self.resource.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<String>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        &mut self.type_
    }

    /// Modify [`Self::type_`] with a `func`
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.type_); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for MetricSpec {
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

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            type Value = MetricSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MetricSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_resource: Option<crate::api::autoscaling::v2::ContainerResourceMetricSource> = None;
                let mut value_external: Option<crate::api::autoscaling::v2::ExternalMetricSource> = None;
                let mut value_object: Option<crate::api::autoscaling::v2::ObjectMetricSource> = None;
                let mut value_pods: Option<crate::api::autoscaling::v2::PodsMetricSource> = None;
                let mut value_resource: Option<crate::api::autoscaling::v2::ResourceMetricSource> = None;
                let mut value_type_: Option<String> = None;

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

                Ok(MetricSpec {
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
            "MetricSpec",
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

impl crate::serde::Serialize for MetricSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricSpec",
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
impl crate::schemars::JsonSchema for MetricSpec {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2.MetricSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerResource".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ContainerResourceMetricSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("containerResource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source. This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "external".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ExternalMetricSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "object".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ObjectMetricSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "pods".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::PodsMetricSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resource".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::ResourceMetricSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("type is the type of metric source.  It should be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each mapping to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "type".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
