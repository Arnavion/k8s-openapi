// Generated from definition io.k8s.api.flowcontrol.v1beta2.PolicyRulesWithSubjects

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRulesWithSubjects {
    /// `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.
    pub non_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule>>,

    /// `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
    pub resource_rules: Option<Vec<crate::api::flowcontrol::v1beta2::ResourcePolicyRule>>,

    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.
    pub subjects: Vec<crate::api::flowcontrol::v1beta2::Subject>,

}

#[cfg(feature = "dsl")]
impl PolicyRulesWithSubjects  {
    /// Set [`Self::non_resource_rules`]
    pub  fn non_resource_rules_set(&mut self, non_resource_rules: impl Into<Option<Vec<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule>>>) -> &mut Self {
        self.non_resource_rules = non_resource_rules.into(); self
    }

    pub  fn non_resource_rules(&mut self) -> &mut Vec<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule> {
        if self.non_resource_rules.is_none() { self.non_resource_rules = Some(Default::default()) }
        self.non_resource_rules.as_mut().unwrap()
    }

    /// Modify [`Self::non_resource_rules`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn non_resource_rules_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule>)) -> &mut Self {
        if self.non_resource_rules.is_none() { self.non_resource_rules = Some(Default::default()) };
        func(self.non_resource_rules.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::non_resource_rules`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn non_resource_rules_push_with(&mut self, func: impl FnOnce(&mut crate::api::flowcontrol::v1beta2::NonResourcePolicyRule)) -> &mut Self {
        if self.non_resource_rules.is_none() {
            self.non_resource_rules = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.non_resource_rules.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::non_resource_rules`]
    pub  fn non_resource_rules_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::flowcontrol::v1beta2::NonResourcePolicyRule]>) -> &mut Self {
         if self.non_resource_rules.is_none() { self.non_resource_rules = Some(Vec::new()); }
         let non_resource_rules = &mut self.non_resource_rules.as_mut().unwrap();
         for item in other.borrow() {
             non_resource_rules.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::resource_rules`]
    pub  fn resource_rules_set(&mut self, resource_rules: impl Into<Option<Vec<crate::api::flowcontrol::v1beta2::ResourcePolicyRule>>>) -> &mut Self {
        self.resource_rules = resource_rules.into(); self
    }

    pub  fn resource_rules(&mut self) -> &mut Vec<crate::api::flowcontrol::v1beta2::ResourcePolicyRule> {
        if self.resource_rules.is_none() { self.resource_rules = Some(Default::default()) }
        self.resource_rules.as_mut().unwrap()
    }

    /// Modify [`Self::resource_rules`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_rules_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::flowcontrol::v1beta2::ResourcePolicyRule>)) -> &mut Self {
        if self.resource_rules.is_none() { self.resource_rules = Some(Default::default()) };
        func(self.resource_rules.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::resource_rules`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn resource_rules_push_with(&mut self, func: impl FnOnce(&mut crate::api::flowcontrol::v1beta2::ResourcePolicyRule)) -> &mut Self {
        if self.resource_rules.is_none() {
            self.resource_rules = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.resource_rules.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::resource_rules`]
    pub  fn resource_rules_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::flowcontrol::v1beta2::ResourcePolicyRule]>) -> &mut Self {
         if self.resource_rules.is_none() { self.resource_rules = Some(Vec::new()); }
         let resource_rules = &mut self.resource_rules.as_mut().unwrap();
         for item in other.borrow() {
             resource_rules.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::subjects`]
    pub  fn subjects_set(&mut self, subjects: impl Into<Vec<crate::api::flowcontrol::v1beta2::Subject>>) -> &mut Self {
        self.subjects = subjects.into(); self
    }

    pub  fn subjects(&mut self) -> &mut Vec<crate::api::flowcontrol::v1beta2::Subject> {
        &mut self.subjects
    }

    /// Modify [`Self::subjects`] with a `func`
    pub  fn subjects_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::flowcontrol::v1beta2::Subject>)) -> &mut Self {
        func(&mut self.subjects); self
    }

    /// Push new element to [`Self::subjects`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn subjects_push_with(&mut self, func: impl FnOnce(&mut crate::api::flowcontrol::v1beta2::Subject)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.subjects.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::subjects`]
    pub  fn subjects_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::flowcontrol::v1beta2::Subject]>) -> &mut Self {
         for item in other.borrow() {
             self.subjects.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PolicyRulesWithSubjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_rules,
            Key_resource_rules,
            Key_subjects,
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
                            "nonResourceRules" => Field::Key_non_resource_rules,
                            "resourceRules" => Field::Key_resource_rules,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRulesWithSubjects;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PolicyRulesWithSubjects")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule>> = None;
                let mut value_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta2::ResourcePolicyRule>> = None;
                let mut value_subjects: Option<Vec<crate::api::flowcontrol::v1beta2::Subject>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_rules => value_non_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_rules => value_resource_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subjects => value_subjects = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRulesWithSubjects {
                    non_resource_rules: value_non_resource_rules,
                    resource_rules: value_resource_rules,
                    subjects: value_subjects.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PolicyRulesWithSubjects",
            &[
                "nonResourceRules",
                "resourceRules",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PolicyRulesWithSubjects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRulesWithSubjects",
            1 +
            self.non_resource_rules.as_ref().map_or(0, |_| 1) +
            self.resource_rules.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceRules", value)?;
        }
        if let Some(value) = &self.resource_rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceRules", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PolicyRulesWithSubjects {
    fn schema_name() -> String {
        "io.k8s.api.flowcontrol.v1beta2.PolicyRulesWithSubjects".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nonResourceRules".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::flowcontrol::v1beta2::NonResourcePolicyRule>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceRules".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("`resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::flowcontrol::v1beta2::ResourcePolicyRule>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "subjects".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::flowcontrol::v1beta2::Subject>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "subjects".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
