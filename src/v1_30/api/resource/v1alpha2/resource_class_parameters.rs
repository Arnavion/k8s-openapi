// Generated from definition io.k8s.api.resource.v1alpha2.ResourceClassParameters

/// ResourceClassParameters defines resource requests for a ResourceClass in an in-tree format understood by Kubernetes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClassParameters {
    /// Filters describes additional contraints that must be met when using the class.
    pub filters: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceFilter>>,

    /// If this object was created from some other resource, then this links back to that resource. This field is used to find the in-tree representation of the class parameters when the parameter reference of the class refers to some unknown type.
    pub generated_from: Option<crate::api::resource::v1alpha2::ResourceClassParametersReference>,

    /// Standard object metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// VendorParameters are arbitrary setup parameters for all claims using this class. They are ignored while allocating the claim. There must not be more than one entry per driver.
    pub vendor_parameters: Option<std::vec::Vec<crate::api::resource::v1alpha2::VendorParameters>>,
}

impl crate::Resource for ResourceClassParameters {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const KIND: &'static str = "ResourceClassParameters";
    const VERSION: &'static str = "v1alpha2";
    const URL_PATH_SEGMENT: &'static str = "resourceclassparameters";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for ResourceClassParameters {
    const LIST_KIND: &'static str = "ResourceClassParametersList";
}

impl crate::Metadata for ResourceClassParameters {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for ResourceClassParameters {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.filters, other.filters);
        crate::DeepMerge::merge_from(&mut self.generated_from, other.generated_from);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::merge_strategies::list::atomic(&mut self.vendor_parameters, other.vendor_parameters);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClassParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_filters,
            Key_generated_from,
            Key_metadata,
            Key_vendor_parameters,
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
                            "filters" => Field::Key_filters,
                            "generatedFrom" => Field::Key_generated_from,
                            "metadata" => Field::Key_metadata,
                            "vendorParameters" => Field::Key_vendor_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClassParameters;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_filters: Option<std::vec::Vec<crate::api::resource::v1alpha2::ResourceFilter>> = None;
                let mut value_generated_from: Option<crate::api::resource::v1alpha2::ResourceClassParametersReference> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_vendor_parameters: Option<std::vec::Vec<crate::api::resource::v1alpha2::VendorParameters>> = None;

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
                        Field::Key_filters => value_filters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_generated_from => value_generated_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor_parameters => value_vendor_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClassParameters {
                    filters: value_filters,
                    generated_from: value_generated_from,
                    metadata: value_metadata.unwrap_or_default(),
                    vendor_parameters: value_vendor_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "filters",
                "generatedFrom",
                "metadata",
                "vendorParameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClassParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.filters.as_ref().map_or(0, |_| 1) +
            self.generated_from.as_ref().map_or(0, |_| 1) +
            self.vendor_parameters.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.filters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "filters", value)?;
        }
        if let Some(value) = &self.generated_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generatedFrom", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.vendor_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vendorParameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClassParameters {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceClassParameters".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClassParameters defines resource requests for a ResourceClass in an in-tree format understood by Kubernetes.".into()),
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
                        "filters".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Filters describes additional contraints that must be met when using the class.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::ResourceFilter>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "generatedFrom".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::ResourceClassParametersReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If this object was created from some other resource, then this links back to that resource. This field is used to find the in-tree representation of the class parameters when the parameter reference of the class refers to some unknown type.".into()),
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
                        "vendorParameters".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VendorParameters are arbitrary setup parameters for all claims using this class. They are ignored while allocating the claim. There must not be more than one entry per driver.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::resource::v1alpha2::VendorParameters>()))),
                                ..Default::default()
                            })),
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
