// Generated from definition io.k8s.api.resource.v1alpha2.ResourceClaimParameters

/// ResourceClaimParameters defines resource requests for a ResourceClaim in an in-tree format understood by Kubernetes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClaimParameters {
    /// DriverRequests describes all resources that are needed for the allocated claim. A single claim may use resources coming from different drivers. For each driver, this array has at most one entry which then may have one or more per-driver requests.
    ///
    /// May be empty, in which case the claim can always be allocated.
    pub driver_requests: Option<std::vec::Vec<crate::api::resource::v1alpha2::DriverRequests>>,

    /// If this object was created from some other resource, then this links back to that resource. This field is used to find the in-tree representation of the claim parameters when the parameter reference of the claim refers to some unknown type.
    pub generated_from: Option<crate::api::resource::v1alpha2::ResourceClaimParametersReference>,

    /// Standard object metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Shareable indicates whether the allocated claim is meant to be shareable by multiple consumers at the same time.
    pub shareable: Option<bool>,
}

impl crate::Resource for ResourceClaimParameters {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const KIND: &'static str = "ResourceClaimParameters";
    const VERSION: &'static str = "v1alpha2";
    const URL_PATH_SEGMENT: &'static str = "resourceclaimparameters";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for ResourceClaimParameters {
    const LIST_KIND: &'static str = "ResourceClaimParametersList";
}

impl crate::Metadata for ResourceClaimParameters {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for ResourceClaimParameters {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.driver_requests, other.driver_requests);
        crate::DeepMerge::merge_from(&mut self.generated_from, other.generated_from);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.shareable, other.shareable);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClaimParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_driver_requests,
            Key_generated_from,
            Key_metadata,
            Key_shareable,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "driverRequests" => Field::Key_driver_requests,
                            "generatedFrom" => Field::Key_generated_from,
                            "metadata" => Field::Key_metadata,
                            "shareable" => Field::Key_shareable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClaimParameters;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver_requests: Option<std::vec::Vec<crate::api::resource::v1alpha2::DriverRequests>> = None;
                let mut value_generated_from: Option<crate::api::resource::v1alpha2::ResourceClaimParametersReference> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_shareable: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_driver_requests => value_driver_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generated_from => value_generated_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_shareable => value_shareable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClaimParameters {
                    driver_requests: value_driver_requests,
                    generated_from: value_generated_from,
                    metadata: value_metadata.unwrap_or_default(),
                    shareable: value_shareable,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "driverRequests",
                "generatedFrom",
                "metadata",
                "shareable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClaimParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.driver_requests.as_ref().map_or(0, |_| 1) +
            self.generated_from.as_ref().map_or(0, |_| 1) +
            self.shareable.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.driver_requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverRequests", value)?;
        }
        if let Some(value) = &self.generated_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generatedFrom", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.shareable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareable", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClaimParameters {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceClaimParameters".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClaimParameters defines resource requests for a ResourceClaim in an in-tree format understood by Kubernetes.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "driverRequests".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverRequests describes all resources that are needed for the allocated claim. A single claim may use resources coming from different drivers. For each driver, this array has at most one entry which then may have one or more per-driver requests.\n\nMay be empty, in which case the claim can always be allocated.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::DriverRequests>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "generatedFrom".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::ResourceClaimParametersReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If this object was created from some other resource, then this links back to that resource. This field is used to find the in-tree representation of the claim parameters when the parameter reference of the claim refers to some unknown type.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "kind".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object metadata".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "shareable".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Shareable indicates whether the allocated claim is meant to be shareable by multiple consumers at the same time.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "metadata".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
