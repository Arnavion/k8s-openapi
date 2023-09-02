// Generated from definition io.k8s.api.core.v1.PodAffinityTerm

/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key \<topologyKey\> matches that of any node on which a pod of the set of pods is running
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    pub label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace".
    pub namespaces: Option<Vec<String>>,

    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topology_key: String,
}

impl crate::DeepMerge for PodAffinityTerm {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.label_selector, other.label_selector);
        crate::DeepMerge::merge_from(&mut self.namespace_selector, other.namespace_selector);
        crate::merge_strategies::list::atomic(&mut self.namespaces, other.namespaces);
        crate::DeepMerge::merge_from(&mut self.topology_key, other.topology_key);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodAffinityTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector,
            Key_namespace_selector,
            Key_namespaces,
            Key_topology_key,
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
                            "labelSelector" => Field::Key_label_selector,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "namespaces" => Field::Key_namespaces,
                            "topologyKey" => Field::Key_topology_key,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodAffinityTerm;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodAffinityTerm")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_label_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_namespaces: Option<Vec<String>> = None;
                let mut value_topology_key: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector => value_label_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace_selector => value_namespace_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespaces => value_namespaces = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_key => value_topology_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodAffinityTerm {
                    label_selector: value_label_selector,
                    namespace_selector: value_namespace_selector,
                    namespaces: value_namespaces,
                    topology_key: value_topology_key.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodAffinityTerm",
            &[
                "labelSelector",
                "namespaceSelector",
                "namespaces",
                "topologyKey",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodAffinityTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodAffinityTerm",
            1 +
            self.label_selector.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.namespaces.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.label_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelector", value)?;
        }
        if let Some(value) = &self.namespace_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.namespaces {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaces", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKey", &self.topology_key)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodAffinityTerm {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PodAffinityTerm".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "labelSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over a set of resources, in this case pods.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "namespaceSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means \"this pod's namespace\". An empty selector ({}) matches all namespaces.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "namespaces".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means \"this pod's namespace\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "topologyKey".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "topologyKey".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
