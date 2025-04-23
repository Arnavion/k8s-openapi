// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceSubresourceScale

/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceSubresourceScale {
    /// labelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status` or `.spec`. Must be set to work with HorizontalPodAutoscaler. The field pointed by this JSON path must be a string field (not a complex selector struct) which contains a serialized label selector in string form. More info: https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource If there is no value under the given path in the custom resource, the `status.selector` value in the `/scale` subresource will default to the empty string.
    pub label_selector_path: Option<std::string::String>,

    /// specReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.spec`. If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.
    pub spec_replicas_path: std::string::String,

    /// statusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status`. If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource will default to 0.
    pub status_replicas_path: std::string::String,
}

impl crate::DeepMerge for CustomResourceSubresourceScale {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.label_selector_path, other.label_selector_path);
        crate::DeepMerge::merge_from(&mut self.spec_replicas_path, other.spec_replicas_path);
        crate::DeepMerge::merge_from(&mut self.status_replicas_path, other.status_replicas_path);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceSubresourceScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector_path,
            Key_spec_replicas_path,
            Key_status_replicas_path,
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
                            "labelSelectorPath" => Field::Key_label_selector_path,
                            "specReplicasPath" => Field::Key_spec_replicas_path,
                            "statusReplicasPath" => Field::Key_status_replicas_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceSubresourceScale;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CustomResourceSubresourceScale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_label_selector_path: Option<std::string::String> = None;
                let mut value_spec_replicas_path: Option<std::string::String> = None;
                let mut value_status_replicas_path: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector_path => value_label_selector_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec_replicas_path => value_spec_replicas_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status_replicas_path => value_status_replicas_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceSubresourceScale {
                    label_selector_path: value_label_selector_path,
                    spec_replicas_path: value_spec_replicas_path.unwrap_or_default(),
                    status_replicas_path: value_status_replicas_path.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceSubresourceScale",
            &[
                "labelSelectorPath",
                "specReplicasPath",
                "statusReplicasPath",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceSubresourceScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceSubresourceScale",
            2 +
            self.label_selector_path.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.label_selector_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelectorPath", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "specReplicasPath", &self.spec_replicas_path)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "statusReplicasPath", &self.status_replicas_path)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceSubresourceScale {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceSubresourceScale".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "labelSelectorPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("labelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status` or `.spec`. Must be set to work with HorizontalPodAutoscaler. The field pointed by this JSON path must be a string field (not a complex selector struct) which contains a serialized label selector in string form. More info: https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource If there is no value under the given path in the custom resource, the `status.selector` value in the `/scale` subresource will default to the empty string.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "specReplicasPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("specReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.spec`. If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "statusReplicasPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("statusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status`. If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource will default to 0.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "specReplicasPath".into(),
                    "statusReplicasPath".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
