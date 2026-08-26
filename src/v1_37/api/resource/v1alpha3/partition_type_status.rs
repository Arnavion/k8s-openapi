// Generated from definition io.k8s.api.resource.v1alpha3.PartitionTypeStatus

/// PartitionTypeStatus reports allocatability for a single partition type, identified by the value of a grouping attribute.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartitionTypeStatus {
    /// Allocatable is the number of additional devices of this partition type that could still be allocated given current shared-counter consumption.
    pub allocatable: i32,

    /// Attribute is the fully qualified name of the device attribute whose value groups this entry. It is the PartitionTypeAttribute declared by the devices' own slice, or the default named in the request when their slice declares none.
    pub attribute: std::string::String,

    /// Total is the number of devices of this partition type in the pool.
    pub total: i32,

    /// Type is the partition type value (e.g. "Full" or "Half").
    pub type_: std::string::String,
}

impl crate::DeepMerge for PartitionTypeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocatable, other.allocatable);
        crate::DeepMerge::merge_from(&mut self.attribute, other.attribute);
        crate::DeepMerge::merge_from(&mut self.total, other.total);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PartitionTypeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocatable,
            Key_attribute,
            Key_total,
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
                            "allocatable" => Field::Key_allocatable,
                            "attribute" => Field::Key_attribute,
                            "total" => Field::Key_total,
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
            type Value = PartitionTypeStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PartitionTypeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocatable: Option<i32> = None;
                let mut value_attribute: Option<std::string::String> = None;
                let mut value_total: Option<i32> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocatable => value_allocatable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attribute => value_attribute = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_total => value_total = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PartitionTypeStatus {
                    allocatable: value_allocatable.unwrap_or_default(),
                    attribute: value_attribute.unwrap_or_default(),
                    total: value_total.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PartitionTypeStatus",
            &[
                "allocatable",
                "attribute",
                "total",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PartitionTypeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PartitionTypeStatus",
            4,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", &self.allocatable)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attribute", &self.attribute)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "total", &self.total)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PartitionTypeStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.PartitionTypeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PartitionTypeStatus reports allocatability for a single partition type, identified by the value of a grouping attribute.",
            "type": "object",
            "properties": {
                "allocatable": {
                    "description": "Allocatable is the number of additional devices of this partition type that could still be allocated given current shared-counter consumption.",
                    "type": "integer",
                    "format": "int32",
                },
                "attribute": {
                    "description": "Attribute is the fully qualified name of the device attribute whose value groups this entry. It is the PartitionTypeAttribute declared by the devices' own slice, or the default named in the request when their slice declares none.",
                    "type": "string",
                },
                "total": {
                    "description": "Total is the number of devices of this partition type in the pool.",
                    "type": "integer",
                    "format": "int32",
                },
                "type": {
                    "description": "Type is the partition type value (e.g. \"Full\" or \"Half\").",
                    "type": "string",
                },
            },
            "required": [
                "allocatable",
                "attribute",
                "total",
                "type",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for PartitionTypeStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.PartitionTypeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("PartitionTypeStatus reports allocatability for a single partition type, identified by the value of a grouping attribute.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "allocatable".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Allocatable is the number of additional devices of this partition type that could still be allocated given current shared-counter consumption.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "attribute".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Attribute is the fully qualified name of the device attribute whose value groups this entry. It is the PartitionTypeAttribute declared by the devices' own slice, or the default named in the request when their slice declares none.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "total".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Total is the number of devices of this partition type in the pool.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Type is the partition type value (e.g. \"Full\" or \"Half\").".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "allocatable".into(),
                    "attribute".into(),
                    "total".into(),
                    "type".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
