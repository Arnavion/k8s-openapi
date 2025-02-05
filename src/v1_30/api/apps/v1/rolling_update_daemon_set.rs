// Generated from definition io.k8s.api.apps.v1.RollingUpdateDaemonSet

/// Spec to control the desired behavior of daemon set rolling update.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of nodes with an existing available DaemonSet pod that can have an updated DaemonSet pod during during an update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up to a minimum of 1. Default value is 0. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their a new pod created before the old pod is marked as deleted. The update starts by launching new pods on 30% of nodes. Once an updated pod is available (Ready for at least minReadySeconds) the old DaemonSet pod on that node is marked deleted. If the old pod becomes unavailable for any reason (Ready transitions to false, is evicted, or is drained) an updated pod is immediatedly created on that node without considering surge limits. Allowing surge implies the possibility that the resources consumed by the daemonset on any given node can double if the readiness check fails, and so resource intensive daemonsets should take into account that they may cause evictions during disruption.
    pub max_surge: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,

    /// The maximum number of DaemonSet pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0 if MaxSurge is 0 Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.
    pub max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,
}

impl crate::DeepMerge for RollingUpdateDaemonSet {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.max_surge, other.max_surge);
        crate::DeepMerge::merge_from(&mut self.max_unavailable, other.max_unavailable);
    }
}

impl<'de> crate::serde::Deserialize<'de> for RollingUpdateDaemonSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_surge,
            Key_max_unavailable,
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
                            "maxSurge" => Field::Key_max_surge,
                            "maxUnavailable" => Field::Key_max_unavailable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RollingUpdateDaemonSet;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("RollingUpdateDaemonSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_surge: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_max_unavailable: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_surge => value_max_surge = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_unavailable => value_max_unavailable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RollingUpdateDaemonSet {
                    max_surge: value_max_surge,
                    max_unavailable: value_max_unavailable,
                })
            }
        }

        deserializer.deserialize_struct(
            "RollingUpdateDaemonSet",
            &[
                "maxSurge",
                "maxUnavailable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RollingUpdateDaemonSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RollingUpdateDaemonSet",
            self.max_surge.as_ref().map_or(0, |_| 1) +
            self.max_unavailable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_surge {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxSurge", value)?;
        }
        if let Some(value) = &self.max_unavailable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxUnavailable", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RollingUpdateDaemonSet {
    fn schema_name() -> std::string::String {
        "io.k8s.api.apps.v1.RollingUpdateDaemonSet".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Spec to control the desired behavior of daemon set rolling update.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "maxSurge".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The maximum number of nodes with an existing available DaemonSet pod that can have an updated DaemonSet pod during during an update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up to a minimum of 1. Default value is 0. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their a new pod created before the old pod is marked as deleted. The update starts by launching new pods on 30% of nodes. Once an updated pod is available (Ready for at least minReadySeconds) the old DaemonSet pod on that node is marked deleted. If the old pod becomes unavailable for any reason (Ready transitions to false, is evicted, or is drained) an updated pod is immediatedly created on that node without considering surge limits. Allowing surge implies the possibility that the resources consumed by the daemonset on any given node can double if the readiness check fails, and so resource intensive daemonsets should take into account that they may cause evictions during disruption.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "maxUnavailable".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The maximum number of DaemonSet pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0 if MaxSurge is 0 Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
