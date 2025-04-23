// Generated from definition io.k8s.api.core.v1.ResourceHealth

/// ResourceHealth represents the health of a resource. It has the latest device health information. This is a part of KEP https://kep.k8s.io/4680.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceHealth {
    /// Health of the resource. can be one of:
    ///  - Healthy: operates as normal
    ///  - Unhealthy: reported unhealthy. We consider this a temporary health issue
    ///               since we do not have a mechanism today to distinguish
    ///               temporary and permanent issues.
    ///  - Unknown: The status cannot be determined.
    ///             For example, Device Plugin got unregistered and hasn't been re-registered since.
    ///
    /// In future we may want to introduce the PermanentlyUnhealthy Status.
    pub health: Option<std::string::String>,

    /// ResourceID is the unique identifier of the resource. See the ResourceID type for more information.
    pub resource_id: std::string::String,
}

impl crate::DeepMerge for ResourceHealth {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.health, other.health);
        crate::DeepMerge::merge_from(&mut self.resource_id, other.resource_id);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceHealth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_health,
            Key_resource_id,
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
                            "health" => Field::Key_health,
                            "resourceID" => Field::Key_resource_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceHealth;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceHealth")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_health: Option<std::string::String> = None;
                let mut value_resource_id: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_health => value_health = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_id => value_resource_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceHealth {
                    health: value_health,
                    resource_id: value_resource_id.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceHealth",
            &[
                "health",
                "resourceID",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceHealth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceHealth",
            1 +
            self.health.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.health {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "health", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceID", &self.resource_id)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceHealth {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ResourceHealth".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ResourceHealth represents the health of a resource. It has the latest device health information. This is a part of KEP https://kep.k8s.io/4680.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "health".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Health of the resource. can be one of:\n - Healthy: operates as normal\n - Unhealthy: reported unhealthy. We consider this a temporary health issue\n              since we do not have a mechanism today to distinguish\n              temporary and permanent issues.\n - Unknown: The status cannot be determined.\n            For example, Device Plugin got unregistered and hasn't been re-registered since.\n\nIn future we may want to introduce the PermanentlyUnhealthy Status.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceID".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceID is the unique identifier of the resource. See the ResourceID type for more information.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "resourceID".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
