// Generated from definition io.k8s.api.autoscaling.v2.ContainerResourceMetricSource

/// ContainerResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.  Only one "target" type should be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerResourceMetricSource {
    /// container is the name of the container in the pods of the scaling target
    pub container: std::string::String,

    /// name is the name of the resource in question.
    pub name: std::string::String,

    /// target specifies the target value for the given metric
    pub target: crate::api::autoscaling::v2::MetricTarget,
}

impl crate::DeepMerge for ContainerResourceMetricSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container, other.container);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.target, other.target);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerResourceMetricSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container,
            Key_name,
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
                            "container" => Field::Key_container,
                            "name" => Field::Key_name,
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
            type Value = ContainerResourceMetricSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerResourceMetricSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_target: Option<crate::api::autoscaling::v2::MetricTarget> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container => value_container = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target => value_target = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerResourceMetricSource {
                    container: value_container.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                    target: value_target.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerResourceMetricSource",
            &[
                "container",
                "name",
                "target",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerResourceMetricSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerResourceMetricSource",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "container", &self.container)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerResourceMetricSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.autoscaling.v2.ContainerResourceMetricSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.  Only one \"target\" type should be set.",
            "type": "object",
            "properties": {
                "container": {
                    "description": "container is the name of the container in the pods of the scaling target",
                    "type": "string",
                },
                "name": {
                    "description": "name is the name of the resource in question.",
                    "type": "string",
                },
                "target": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::autoscaling::v2::MetricTarget>();
                    schema_obj.ensure_object().insert("description".into(), "target specifies the target value for the given metric".into());
                    schema_obj
                }),
            },
            "required": [
                "container",
                "name",
                "target",
            ],
        })
    }
}
