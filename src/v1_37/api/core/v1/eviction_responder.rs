// Generated from definition io.k8s.api.core.v1.EvictionResponder

/// EvictionResponder allows you to specify the responder reacting to an Eviction. Responders should observe and communicate through the Eviction Resource API to help with the graceful eviction of a target (e.g. termination of a pod).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvictionResponder {
    /// name allows you to identify the responder responding to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field must be unique for each responder. This field is required.
    pub name: std::string::String,

    /// priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.
    ///
    /// The responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.
    ///
    /// The minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required.
    pub priority: i32,
}

impl crate::DeepMerge for EvictionResponder {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.priority, other.priority);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EvictionResponder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_priority,
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
                            "priority" => Field::Key_priority,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EvictionResponder;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EvictionResponder")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_priority: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority => value_priority = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EvictionResponder {
                    name: value_name.unwrap_or_default(),
                    priority: value_priority.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "EvictionResponder",
            &[
                "name",
                "priority",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EvictionResponder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EvictionResponder",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priority", &self.priority)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EvictionResponder {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.EvictionResponder".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "EvictionResponder allows you to specify the responder reacting to an Eviction. Responders should observe and communicate through the Eviction Resource API to help with the graceful eviction of a target (e.g. termination of a pod).",
            "type": "object",
            "properties": {
                "name": {
                    "description": "name allows you to identify the responder responding to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field must be unique for each responder. This field is required.",
                    "type": "string",
                },
                "priority": {
                    "description": "priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.\n\nThe responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.\n\nThe minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required.",
                    "type": "integer",
                    "format": "int32",
                },
            },
            "required": [
                "name",
                "priority",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for EvictionResponder {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.EvictionResponder".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("EvictionResponder allows you to specify the responder reacting to an Eviction. Responders should observe and communicate through the Eviction Resource API to help with the graceful eviction of a target (e.g. termination of a pod).".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("name allows you to identify the responder responding to the Eviction.\n\nIt must be a valid domain-prefixed key (such as \"acme.io/foo\"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field must be unique for each responder. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "priority".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.\n\nThe responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.\n\nThe minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Integer))),
                            format: Some("int32".into()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "name".into(),
                    "priority".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
