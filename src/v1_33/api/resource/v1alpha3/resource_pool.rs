// Generated from definition io.k8s.api.resource.v1alpha3.ResourcePool

/// ResourcePool describes the pool that ResourceSlices belong to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePool {
    /// Generation tracks the change in a pool over time. Whenever a driver changes something about one or more of the resources in a pool, it must change the generation in all ResourceSlices which are part of that pool. Consumers of ResourceSlices should only consider resources from the pool with the highest generation number. The generation may be reset by drivers, which should be fine for consumers, assuming that all ResourceSlices in a pool are updated to match or deleted.
    ///
    /// Combined with ResourceSliceCount, this mechanism enables consumers to detect pools which are comprised of multiple ResourceSlices and are in an incomplete state.
    pub generation: i64,

    /// Name is used to identify the pool. For node-local devices, this is often the node name, but this is not required.
    ///
    /// It must not be longer than 253 characters and must consist of one or more DNS sub-domains separated by slashes. This field is immutable.
    pub name: std::string::String,

    /// ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number. Must be greater than zero.
    ///
    /// Consumers can use this to check whether they have seen all ResourceSlices belonging to the same pool.
    pub resource_slice_count: i64,
}

impl crate::DeepMerge for ResourcePool {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.generation, other.generation);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.resource_slice_count, other.resource_slice_count);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourcePool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_generation,
            Key_name,
            Key_resource_slice_count,
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
                            "generation" => Field::Key_generation,
                            "name" => Field::Key_name,
                            "resourceSliceCount" => Field::Key_resource_slice_count,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePool;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourcePool")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_generation: Option<i64> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_resource_slice_count: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_generation => value_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_slice_count => value_resource_slice_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourcePool {
                    generation: value_generation.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    resource_slice_count: value_resource_slice_count.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourcePool",
            &[
                "generation",
                "name",
                "resourceSliceCount",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourcePool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourcePool",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "generation", &self.generation)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceSliceCount", &self.resource_slice_count)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourcePool {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.ResourcePool".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourcePool describes the pool that ResourceSlices belong to.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "generation".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Generation tracks the change in a pool over time. Whenever a driver changes something about one or more of the resources in a pool, it must change the generation in all ResourceSlices which are part of that pool. Consumers of ResourceSlices should only consider resources from the pool with the highest generation number. The generation may be reset by drivers, which should be fine for consumers, assuming that all ResourceSlices in a pool are updated to match or deleted.\n\nCombined with ResourceSliceCount, this mechanism enables consumers to detect pools which are comprised of multiple ResourceSlices and are in an incomplete state.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is used to identify the pool. For node-local devices, this is often the node name, but this is not required.\n\nIt must not be longer than 253 characters and must consist of one or more DNS sub-domains separated by slashes. This field is immutable.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceSliceCount".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number. Must be greater than zero.\n\nConsumers can use this to check whether they have seen all ResourceSlices belonging to the same pool.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "generation".into(),
                    "name".into(),
                    "resourceSliceCount".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
