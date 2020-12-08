// Generated from definition io.k8s.api.autoscaling.v2beta2.ContainerResourceMetricStatus

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerResourceMetricStatus {
    /// Container is the name of the container in the pods of the scaling target
    pub container: String,

    /// current contains the current value for the given metric
    pub current: crate::api::autoscaling::v2beta2::MetricValueStatus,

    /// Name is the name of the resource in question.
    pub name: String,
}

impl<'de> serde::Deserialize<'de> for ContainerResourceMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container,
            Key_current,
            Key_name,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "container" => Field::Key_container,
                            "current" => Field::Key_current,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerResourceMetricStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerResourceMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_container: Option<String> = None;
                let mut value_current: Option<crate::api::autoscaling::v2beta2::MetricValueStatus> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container => value_container = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_current => value_current = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerResourceMetricStatus {
                    container: value_container.ok_or_else(|| serde::de::Error::missing_field("container"))?,
                    current: value_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerResourceMetricStatus",
            &[
                "container",
                "current",
                "name",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ContainerResourceMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerResourceMetricStatus",
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "container", &self.container)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "current", &self.current)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::end(state)
    }
}
