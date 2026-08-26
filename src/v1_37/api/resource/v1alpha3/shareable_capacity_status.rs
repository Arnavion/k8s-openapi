// Generated from definition io.k8s.api.resource.v1alpha3.ShareableCapacityStatus

/// ShareableCapacityStatus reports aggregate amounts for a single shareable capacity key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShareableCapacityStatus {
    /// Available is Total minus Consumed, never negative.
    pub available: crate::apimachinery::pkg::api::resource::Quantity,

    /// Consumed is the amount drawn by current allocations.
    pub consumed: crate::apimachinery::pkg::api::resource::Quantity,

    /// Name is the capacity name.
    pub name: std::string::String,

    /// Total is the sum of this capacity across shareable devices in the pool.
    pub total: crate::apimachinery::pkg::api::resource::Quantity,
}

impl crate::DeepMerge for ShareableCapacityStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.available, other.available);
        crate::DeepMerge::merge_from(&mut self.consumed, other.consumed);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.total, other.total);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ShareableCapacityStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available,
            Key_consumed,
            Key_name,
            Key_total,
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
                            "available" => Field::Key_available,
                            "consumed" => Field::Key_consumed,
                            "name" => Field::Key_name,
                            "total" => Field::Key_total,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ShareableCapacityStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ShareableCapacityStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_consumed: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_total: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available => value_available = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_consumed => value_consumed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_total => value_total = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ShareableCapacityStatus {
                    available: value_available.unwrap_or_default(),
                    consumed: value_consumed.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    total: value_total.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ShareableCapacityStatus",
            &[
                "available",
                "consumed",
                "name",
                "total",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ShareableCapacityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ShareableCapacityStatus",
            4,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "available", &self.available)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "consumed", &self.consumed)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "total", &self.total)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ShareableCapacityStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.ShareableCapacityStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ShareableCapacityStatus reports aggregate amounts for a single shareable capacity key.",
            "type": "object",
            "properties": {
                "available": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Available is Total minus Consumed, never negative.".into());
                    schema_obj
                }),
                "consumed": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Consumed is the amount drawn by current allocations.".into());
                    schema_obj
                }),
                "name": {
                    "description": "Name is the capacity name.",
                    "type": "string",
                },
                "total": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>();
                    schema_obj.ensure_object().insert("description".into(), "Total is the sum of this capacity across shareable devices in the pool.".into());
                    schema_obj
                }),
            },
            "required": [
                "available",
                "consumed",
                "name",
                "total",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for ShareableCapacityStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.ShareableCapacityStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("ShareableCapacityStatus reports aggregate amounts for a single shareable capacity key.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "available".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Available is Total minus Consumed, never negative.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "consumed".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Consumed is the amount drawn by current allocations.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Name is the capacity name.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "total".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Total is the sum of this capacity across shareable devices in the pool.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "available".into(),
                    "consumed".into(),
                    "name".into(),
                    "total".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
