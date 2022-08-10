// Generated from definition io.k8s.api.core.v1.Taint

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Taint {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    ///
    pub effect: String,

    /// Required. The taint key to be applied to a node.
    pub key: String,

    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    pub time_added: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// The taint value corresponding to the taint key.
    pub value: Option<String>,
}

impl crate::DeepMerge for Taint {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.effect, other.effect);
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::DeepMerge::merge_from(&mut self.time_added, other.time_added);
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Taint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_time_added,
            Key_value,
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
                            "effect" => Field::Key_effect,
                            "key" => Field::Key_key,
                            "timeAdded" => Field::Key_time_added,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Taint;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Taint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_effect: Option<String> = None;
                let mut value_key: Option<String> = None;
                let mut value_time_added: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_value: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time_added => value_time_added = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Taint {
                    effect: value_effect.unwrap_or_default(),
                    key: value_key.unwrap_or_default(),
                    time_added: value_time_added,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "Taint",
            &[
                "effect",
                "key",
                "timeAdded",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Taint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Taint",
            2 +
            self.time_added.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", &self.effect)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        if let Some(value) = &self.time_added {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeAdded", value)?;
        }
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Taint {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Taint".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("The node this Taint is attached to has the \"effect\" on any pod that does not tolerate the Taint.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "effect".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Required. The taint key to be applied to a node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeAdded".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "value".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The taint value corresponding to the taint key.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "effect".to_owned(),
                    "key".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
