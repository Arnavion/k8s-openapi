// Generated from definition io.k8s.api.lifecycle.v1alpha1.EvictionSpec

/// EvictionSpec is a specification of an Eviction.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionSpec {
    /// target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.
    pub target: crate::api::lifecycle::v1alpha1::EvictionTarget,
}

impl crate::DeepMerge for EvictionSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.target, other.target);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_target,
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
                            "target" => Field::Key_target,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_target: Option<crate::api::lifecycle::v1alpha1::EvictionTarget> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_target => value_target = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionSpec {
                    target: value_target.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionSpec",
            &[
                "target",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionSpec",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.lifecycle.v1alpha1.EvictionSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionSpec is a specification of an Eviction.",
            "type": "object",
            "properties": {
                "target": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionTarget>();
                    schema_obj.ensure_object().insert("description".into(), "target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.".into());
                    schema_obj
                }),
            },
            "required": [
                "target",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.lifecycle.v1alpha1.EvictionSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionSpec is a specification of an Eviction.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "target".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::lifecycle::v1alpha1::EvictionTarget>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.".into()),
                                ..Default::default()
                            }));
                            crate::schemars08::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "target".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
