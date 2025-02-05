// Generated from definition io.k8s.api.resource.v1alpha2.ResourceClass

/// ResourceClass is used by administrators to influence how resources are allocated.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation feature gate.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceClass {
    /// DriverName defines the name of the dynamic resource driver that is used for allocation of a ResourceClaim that uses this class.
    ///
    /// Resource drivers have a unique name in forward domain order (acme.example.com).
    pub driver_name: std::string::String,

    /// Standard object metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// ParametersRef references an arbitrary separate object that may hold parameters that will be used by the driver when allocating a resource that uses this class. A dynamic resource driver can distinguish between parameters stored here and and those stored in ResourceClaimSpec.
    pub parameters_ref: Option<crate::api::resource::v1alpha2::ResourceClassParametersReference>,

    /// If and only if allocation of claims using this class is handled via structured parameters, then StructuredParameters must be set to true.
    pub structured_parameters: Option<bool>,

    /// Only nodes matching the selector will be considered by the scheduler when trying to find a Node that fits a Pod when that Pod uses a ResourceClaim that has not been allocated yet.
    ///
    /// Setting this field is optional. If null, all nodes are candidates.
    pub suitable_nodes: Option<crate::api::core::v1::NodeSelector>,
}

impl crate::Resource for ResourceClass {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const KIND: &'static str = "ResourceClass";
    const VERSION: &'static str = "v1alpha2";
    const URL_PATH_SEGMENT: &'static str = "resourceclasses";
    type Scope = crate::ClusterResourceScope;
}

impl crate::ListableResource for ResourceClass {
    const LIST_KIND: &'static str = "ResourceClassList";
}

impl crate::Metadata for ResourceClass {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for ResourceClass {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::DeepMerge::merge_from(&mut self.parameters_ref, other.parameters_ref);
        crate::DeepMerge::merge_from(&mut self.structured_parameters, other.structured_parameters);
        crate::DeepMerge::merge_from(&mut self.suitable_nodes, other.suitable_nodes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_driver_name,
            Key_metadata,
            Key_parameters_ref,
            Key_structured_parameters,
            Key_suitable_nodes,
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
                            "driverName" => Field::Key_driver_name,
                            "metadata" => Field::Key_metadata,
                            "parametersRef" => Field::Key_parameters_ref,
                            "structuredParameters" => Field::Key_structured_parameters,
                            "suitableNodes" => Field::Key_suitable_nodes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceClass;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver_name: Option<std::string::String> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_parameters_ref: Option<crate::api::resource::v1alpha2::ResourceClassParametersReference> = None;
                let mut value_structured_parameters: Option<bool> = None;
                let mut value_suitable_nodes: Option<crate::api::core::v1::NodeSelector> = None;

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
                        Field::Key_driver_name => value_driver_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters_ref => value_parameters_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_structured_parameters => value_structured_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suitable_nodes => value_suitable_nodes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceClass {
                    driver_name: value_driver_name.unwrap_or_default(),
                    metadata: value_metadata.unwrap_or_default(),
                    parameters_ref: value_parameters_ref,
                    structured_parameters: value_structured_parameters,
                    suitable_nodes: value_suitable_nodes,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "driverName",
                "metadata",
                "parametersRef",
                "structuredParameters",
                "suitableNodes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.parameters_ref.as_ref().map_or(0, |_| 1) +
            self.structured_parameters.as_ref().map_or(0, |_| 1) +
            self.suitable_nodes.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driverName", &self.driver_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.parameters_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parametersRef", value)?;
        }
        if let Some(value) = &self.structured_parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "structuredParameters", value)?;
        }
        if let Some(value) = &self.suitable_nodes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "suitableNodes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceClass {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha2.ResourceClass".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceClass is used by administrators to influence how resources are allocated.\n\nThis is an alpha type and requires enabling the DynamicResourceAllocation feature gate.".into()),
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
                        "driverName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DriverName defines the name of the dynamic resource driver that is used for allocation of a ResourceClaim that uses this class.\n\nResource drivers have a unique name in forward domain order (acme.example.com).".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
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
                        "parametersRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha2::ResourceClassParametersReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParametersRef references an arbitrary separate object that may hold parameters that will be used by the driver when allocating a resource that uses this class. A dynamic resource driver can distinguish between parameters stored here and and those stored in ResourceClaimSpec.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "structuredParameters".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("If and only if allocation of claims using this class is handled via structured parameters, then StructuredParameters must be set to true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "suitableNodes".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Only nodes matching the selector will be considered by the scheduler when trying to find a Node that fits a Pod when that Pod uses a ResourceClaim that has not been allocated yet.\n\nSetting this field is optional. If null, all nodes are candidates.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "driverName".into(),
                    "metadata".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
