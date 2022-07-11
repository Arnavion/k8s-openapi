// Generated from definition io.k8s.api.autoscaling.v2beta2.HPAScalingRules

/// HPAScalingRules configures the scaling behavior for one direction. These Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HPAScalingRules {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    pub policies: Option<Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy>>,

    /// selectPolicy is used to specify which policy should be used. If not set, the default value MaxPolicySelect is used.
    pub select_policy: Option<String>,

    /// StabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    pub stabilization_window_seconds: Option<i32>,

}

#[cfg(feature = "dsl")]
impl HPAScalingRules  {
    /// Set [`Self::policies`]
    pub  fn policies_set(&mut self, policies: impl Into<Option<Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy>>>) -> &mut Self {
        self.policies = policies.into(); self
    }

    pub  fn policies(&mut self) -> &mut Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy> {
        if self.policies.is_none() { self.policies = Some(Default::default()) }
        self.policies.as_mut().unwrap()
    }

    /// Modify [`Self::policies`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn policies_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy>)) -> &mut Self {
        if self.policies.is_none() { self.policies = Some(Default::default()) };
        func(self.policies.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::policies`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn policies_push_with(&mut self, func: impl FnOnce(&mut crate::api::autoscaling::v2beta2::HPAScalingPolicy)) -> &mut Self {
        if self.policies.is_none() {
            self.policies = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.policies.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::policies`]
    pub  fn policies_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::autoscaling::v2beta2::HPAScalingPolicy]>) -> &mut Self {
         if self.policies.is_none() { self.policies = Some(Vec::new()); }
         let policies = &mut self.policies.as_mut().unwrap();
         for item in other.borrow() {
             policies.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::select_policy`]
    pub  fn select_policy_set(&mut self, select_policy: impl Into<Option<String>>) -> &mut Self {
        self.select_policy = select_policy.into(); self
    }

    pub  fn select_policy(&mut self) -> &mut String {
        if self.select_policy.is_none() { self.select_policy = Some(Default::default()) }
        self.select_policy.as_mut().unwrap()
    }

    /// Modify [`Self::select_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn select_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.select_policy.is_none() { self.select_policy = Some(Default::default()) };
        func(self.select_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::stabilization_window_seconds`]
    pub  fn stabilization_window_seconds_set(&mut self, stabilization_window_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.stabilization_window_seconds = stabilization_window_seconds.into(); self
    }

    pub  fn stabilization_window_seconds(&mut self) -> &mut i32 {
        if self.stabilization_window_seconds.is_none() { self.stabilization_window_seconds = Some(Default::default()) }
        self.stabilization_window_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::stabilization_window_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn stabilization_window_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.stabilization_window_seconds.is_none() { self.stabilization_window_seconds = Some(Default::default()) };
        func(self.stabilization_window_seconds.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for HPAScalingRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_policies,
            Key_select_policy,
            Key_stabilization_window_seconds,
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
                            "policies" => Field::Key_policies,
                            "selectPolicy" => Field::Key_select_policy,
                            "stabilizationWindowSeconds" => Field::Key_stabilization_window_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HPAScalingRules;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HPAScalingRules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_policies: Option<Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy>> = None;
                let mut value_select_policy: Option<String> = None;
                let mut value_stabilization_window_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_policies => value_policies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_select_policy => value_select_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stabilization_window_seconds => value_stabilization_window_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HPAScalingRules {
                    policies: value_policies,
                    select_policy: value_select_policy,
                    stabilization_window_seconds: value_stabilization_window_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "HPAScalingRules",
            &[
                "policies",
                "selectPolicy",
                "stabilizationWindowSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HPAScalingRules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingRules",
            self.policies.as_ref().map_or(0, |_| 1) +
            self.select_policy.as_ref().map_or(0, |_| 1) +
            self.stabilization_window_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.policies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "policies", value)?;
        }
        if let Some(value) = &self.select_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectPolicy", value)?;
        }
        if let Some(value) = &self.stabilization_window_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stabilizationWindowSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HPAScalingRules {
    fn schema_name() -> String {
        "io.k8s.api.autoscaling.v2beta2.HPAScalingRules".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("HPAScalingRules configures the scaling behavior for one direction. These Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "policies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::autoscaling::v2beta2::HPAScalingPolicy>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selectPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("selectPolicy is used to specify which policy should be used. If not set, the default value MaxPolicySelect is used.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "stabilizationWindowSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
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
