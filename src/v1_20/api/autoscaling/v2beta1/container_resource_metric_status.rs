// Generated from definition io.k8s.api.autoscaling.v2beta1.ContainerResourceMetricStatus

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerResourceMetricStatus {
    /// container is the name of the container in the pods of the scaling target
    pub container: String,

    /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.  It will only be present if `targetAverageValue` was set in the corresponding metric specification.
    pub current_average_utilization: Option<i32>,

    /// currentAverageValue is the current value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the "pods" metric source type. It will always be set, regardless of the corresponding metric specification.
    pub current_average_value: crate::apimachinery::pkg::api::resource::Quantity,

    /// name is the name of the resource in question.
    pub name: String,
}

impl<'de> serde::Deserialize<'de> for ContainerResourceMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container,
            Key_current_average_utilization,
            Key_current_average_value,
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
                            "currentAverageUtilization" => Field::Key_current_average_utilization,
                            "currentAverageValue" => Field::Key_current_average_value,
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
                let mut value_current_average_utilization: Option<i32> = None;
                let mut value_current_average_value: Option<crate::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container => value_container = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_current_average_utilization => value_current_average_utilization = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_average_value => value_current_average_value = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerResourceMetricStatus {
                    container: value_container.ok_or_else(|| serde::de::Error::missing_field("container"))?,
                    current_average_utilization: value_current_average_utilization,
                    current_average_value: value_current_average_value.ok_or_else(|| serde::de::Error::missing_field("currentAverageValue"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerResourceMetricStatus",
            &[
                "container",
                "currentAverageUtilization",
                "currentAverageValue",
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
            3 +
            self.current_average_utilization.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "container", &self.container)?;
        if let Some(value) = &self.current_average_utilization {
            serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageUtilization", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "currentAverageValue", &self.current_average_value)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::end(state)
    }
}
