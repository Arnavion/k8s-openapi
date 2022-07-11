// Generated from definition io.k8s.api.rbac.v1beta1.AggregationRule

/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
    pub cluster_role_selectors: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>,

}

#[cfg(feature = "dsl")]
impl AggregationRule  {
    /// Set [`Self::cluster_role_selectors`]
    pub  fn cluster_role_selectors_set(&mut self, cluster_role_selectors: impl Into<Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>>) -> &mut Self {
        self.cluster_role_selectors = cluster_role_selectors.into(); self
    }

    pub  fn cluster_role_selectors(&mut self) -> &mut Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> {
        if self.cluster_role_selectors.is_none() { self.cluster_role_selectors = Some(Default::default()) }
        self.cluster_role_selectors.as_mut().unwrap()
    }

    /// Modify [`Self::cluster_role_selectors`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cluster_role_selectors_with(&mut self, func: impl FnOnce(&mut Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>)) -> &mut Self {
        if self.cluster_role_selectors.is_none() { self.cluster_role_selectors = Some(Default::default()) };
        func(self.cluster_role_selectors.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::cluster_role_selectors`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn cluster_role_selectors_push_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        if self.cluster_role_selectors.is_none() {
            self.cluster_role_selectors = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.cluster_role_selectors.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::cluster_role_selectors`]
    pub  fn cluster_role_selectors_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apimachinery::pkg::apis::meta::v1::LabelSelector]>) -> &mut Self {
         if self.cluster_role_selectors.is_none() { self.cluster_role_selectors = Some(Vec::new()); }
         let cluster_role_selectors = &mut self.cluster_role_selectors.as_mut().unwrap();
         for item in other.borrow() {
             cluster_role_selectors.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for AggregationRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cluster_role_selectors,
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
                            "clusterRoleSelectors" => Field::Key_cluster_role_selectors,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AggregationRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AggregationRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cluster_role_selectors: Option<Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_role_selectors => value_cluster_role_selectors = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AggregationRule {
                    cluster_role_selectors: value_cluster_role_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "AggregationRule",
            &[
                "clusterRoleSelectors",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AggregationRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AggregationRule",
            self.cluster_role_selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cluster_role_selectors {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterRoleSelectors", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AggregationRule {
    fn schema_name() -> String {
        "io.k8s.api.rbac.v1beta1.AggregationRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "clusterRoleSelectors".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>()))),
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
