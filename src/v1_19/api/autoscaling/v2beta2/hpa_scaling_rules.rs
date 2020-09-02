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

impl<'de> serde::Deserialize<'de> for HPAScalingRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_policies,
            Key_select_policy,
            Key_stabilization_window_seconds,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HPAScalingRules;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HPAScalingRules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_policies: Option<Vec<crate::api::autoscaling::v2beta2::HPAScalingPolicy>> = None;
                let mut value_select_policy: Option<String> = None;
                let mut value_stabilization_window_seconds: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_policies => value_policies = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_select_policy => value_select_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stabilization_window_seconds => value_stabilization_window_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for HPAScalingRules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingRules",
            self.policies.as_ref().map_or(0, |_| 1) +
            self.select_policy.as_ref().map_or(0, |_| 1) +
            self.stabilization_window_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.policies {
            serde::ser::SerializeStruct::serialize_field(&mut state, "policies", value)?;
        }
        if let Some(value) = &self.select_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "selectPolicy", value)?;
        }
        if let Some(value) = &self.stabilization_window_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "stabilizationWindowSeconds", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
