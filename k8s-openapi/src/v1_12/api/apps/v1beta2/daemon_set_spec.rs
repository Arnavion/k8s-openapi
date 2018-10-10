// Generated from definition io.k8s.api.apps.v1beta2.DaemonSetSpec

/// DaemonSetSpec is the specification of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetSpec {
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
    pub min_ready_seconds: Option<i32>,

    /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    pub revision_history_limit: Option<i32>,

    /// A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: ::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    pub template: ::v1_12::api::core::v1::PodTemplateSpec,

    /// An update strategy to replace existing DaemonSet pods with new pods.
    pub update_strategy: Option<::v1_12::api::apps::v1beta2::DaemonSetUpdateStrategy>,
}

impl<'de> ::serde::Deserialize<'de> for DaemonSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_revision_history_limit,
            Key_selector,
            Key_template,
            Key_update_strategy,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "selector" => Field::Key_selector,
                            "template" => Field::Key_template,
                            "updateStrategy" => Field::Key_update_strategy,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DaemonSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_template: Option<::v1_12::api::core::v1::PodTemplateSpec> = None;
                let mut value_update_strategy: Option<::v1_12::api::apps::v1beta2::DaemonSetUpdateStrategy> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_template => value_template = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_update_strategy => value_update_strategy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector.ok_or_else(|| ::serde::de::Error::missing_field("selector"))?,
                    template: value_template.ok_or_else(|| ::serde::de::Error::missing_field("template"))?,
                    update_strategy: value_update_strategy,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSetSpec",
            &[
                "minReadySeconds",
                "revisionHistoryLimit",
                "selector",
                "template",
                "updateStrategy",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for DaemonSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetSpec",
            0 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.update_strategy.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.update_strategy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "updateStrategy", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
