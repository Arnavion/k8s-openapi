// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionPodReference

/// EvictionPodReference contains enough information to locate the referenced pod inside the same namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionPodReference {
    /// name of the target. This field is required.
    pub name: std::string::String,

    /// uid of the target. It can be found in .metadata.uid of the target and is a lowercase UUID in 8-4-4-4-12 format. This field is required.
    pub uid: std::string::String,
}

impl crate::DeepMerge for EvictionPodReference {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionPodReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_uid,
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
                            "name" => Field::Key_name,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionPodReference;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionPodReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_uid: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionPodReference {
                    name: value_name.unwrap_or_default(),
                    uid: value_uid.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionPodReference",
            &[
                "name",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionPodReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionPodReference",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionPodReference {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionPodReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionPodReference contains enough information to locate the referenced pod inside the same namespace.",
            "type": "object",
            "properties": {
                "name": {
                    "description": "name of the target. This field is required.",
                    "type": "string",
                },
                "uid": {
                    "description": "uid of the target. It can be found in .metadata.uid of the target and is a lowercase UUID in 8-4-4-4-12 format. This field is required.",
                    "type": "string",
                },
            },
            "required": [
                "name",
                "uid",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionPodReference {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionPodReference".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionPodReference contains enough information to locate the referenced pod inside the same namespace.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name of the target. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("uid of the target. It can be found in .metadata.uid of the target and is a lowercase UUID in 8-4-4-4-12 format. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "uid".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
