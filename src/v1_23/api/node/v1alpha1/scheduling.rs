// Generated from definition io.k8s.api.node.v1alpha1.Scheduling

/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scheduling {
    /// nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.
    pub node_selector: Option<std::collections::BTreeMap<String, String>>,

    /// tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.
    pub tolerations: Option<Vec<crate::api::core::v1::Toleration>>,

}

#[cfg(feature = "dsl")]
impl Scheduling  {
    /// Set [`Self::node_selector`]
    pub  fn node_selector_set(&mut self, node_selector: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.node_selector = node_selector.into(); self
    }

    pub  fn node_selector(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.node_selector.is_none() { self.node_selector = Some(Default::default()) }
        self.node_selector.as_mut().unwrap()
    }

    /// Modify [`Self::node_selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_selector_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.node_selector.is_none() { self.node_selector = Some(Default::default()) };
        func(self.node_selector.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::node_selector`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn node_selector_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.node_selector.is_none() {
            self.node_selector = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.node_selector.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::node_selector`]
    pub  fn node_selector_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.node_selector.is_none() { self.node_selector = Some(std::collections::BTreeMap::new()); }
         let node_selector = &mut self.node_selector.as_mut().unwrap();
         for (name, value) in other.borrow() {
             node_selector.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::tolerations`]
    pub  fn tolerations_set(&mut self, tolerations: impl Into<Option<Vec<crate::api::core::v1::Toleration>>>) -> &mut Self {
        self.tolerations = tolerations.into(); self
    }

    pub  fn tolerations(&mut self) -> &mut Vec<crate::api::core::v1::Toleration> {
        if self.tolerations.is_none() { self.tolerations = Some(Default::default()) }
        self.tolerations.as_mut().unwrap()
    }

    /// Modify [`Self::tolerations`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn tolerations_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Toleration>)) -> &mut Self {
        if self.tolerations.is_none() { self.tolerations = Some(Default::default()) };
        func(self.tolerations.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::tolerations`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn tolerations_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Toleration)) -> &mut Self {
        if self.tolerations.is_none() {
            self.tolerations = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.tolerations.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::tolerations`]
    pub  fn tolerations_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Toleration]>) -> &mut Self {
         if self.tolerations.is_none() { self.tolerations = Some(Vec::new()); }
         let tolerations = &mut self.tolerations.as_mut().unwrap();
         for item in other.borrow() {
             tolerations.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Scheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_selector,
            Key_tolerations,
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
                            "nodeSelector" => Field::Key_node_selector,
                            "tolerations" => Field::Key_tolerations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Scheduling;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Scheduling")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_node_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_tolerations: Option<Vec<crate::api::core::v1::Toleration>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_selector => value_node_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tolerations => value_tolerations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Scheduling {
                    node_selector: value_node_selector,
                    tolerations: value_tolerations,
                })
            }
        }

        deserializer.deserialize_struct(
            "Scheduling",
            &[
                "nodeSelector",
                "tolerations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Scheduling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scheduling",
            self.node_selector.as_ref().map_or(0, |_| 1) +
            self.tolerations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.node_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelector", value)?;
        }
        if let Some(value) = &self.tolerations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerations", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Scheduling {
    fn schema_name() -> String {
        "io.k8s.api.node.v1alpha1.Scheduling".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nodeSelector".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tolerations".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Toleration>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
