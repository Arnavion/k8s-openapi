// Generated from definition io.k8s.api.policy.v1beta1.RuntimeClassStrategyOptions

/// RuntimeClassStrategyOptions define the strategy that will dictate the allowable RuntimeClasses for a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuntimeClassStrategyOptions {
    /// allowedRuntimeClassNames is an allowlist of RuntimeClass names that may be specified on a pod. A value of "*" means that any RuntimeClass name is allowed, and must be the only item in the list. An empty list requires the RuntimeClassName field to be unset.
    pub allowed_runtime_class_names: Vec<String>,

    /// defaultRuntimeClassName is the default RuntimeClassName to set on the pod. The default MUST be allowed by the allowedRuntimeClassNames list. A value of nil does not mutate the Pod.
    pub default_runtime_class_name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for RuntimeClassStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed_runtime_class_names,
            Key_default_runtime_class_name,
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
                            "allowedRuntimeClassNames" => Field::Key_allowed_runtime_class_names,
                            "defaultRuntimeClassName" => Field::Key_default_runtime_class_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuntimeClassStrategyOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RuntimeClassStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allowed_runtime_class_names: Option<Vec<String>> = None;
                let mut value_default_runtime_class_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed_runtime_class_names => value_allowed_runtime_class_names = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_default_runtime_class_name => value_default_runtime_class_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RuntimeClassStrategyOptions {
                    allowed_runtime_class_names: value_allowed_runtime_class_names.ok_or_else(|| serde::de::Error::missing_field("allowedRuntimeClassNames"))?,
                    default_runtime_class_name: value_default_runtime_class_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "RuntimeClassStrategyOptions",
            &[
                "allowedRuntimeClassNames",
                "defaultRuntimeClassName",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for RuntimeClassStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RuntimeClassStrategyOptions",
            1 +
            self.default_runtime_class_name.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "allowedRuntimeClassNames", &self.allowed_runtime_class_names)?;
        if let Some(value) = &self.default_runtime_class_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultRuntimeClassName", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
