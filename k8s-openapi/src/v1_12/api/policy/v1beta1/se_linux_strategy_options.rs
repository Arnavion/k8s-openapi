// Generated from definition io.k8s.api.policy.v1beta1.SELinuxStrategyOptions

/// SELinuxStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SELinuxStrategyOptions {
    /// rule is the strategy that will dictate the allowable labels that may be set.
    pub rule: String,

    /// seLinuxOptions required to run as; required for MustRunAs More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    pub se_linux_options: Option<::v1_12::api::core::v1::SELinuxOptions>,
}

impl<'de> ::serde::Deserialize<'de> for SELinuxStrategyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rule,
            Key_se_linux_options,
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
                            "rule" => Field::Key_rule,
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SELinuxStrategyOptions;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct SELinuxStrategyOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_rule: Option<String> = None;
                let mut value_se_linux_options: Option<::v1_12::api::core::v1::SELinuxOptions> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rule => value_rule = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_se_linux_options => value_se_linux_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SELinuxStrategyOptions {
                    rule: value_rule.ok_or_else(|| ::serde::de::Error::missing_field("rule"))?,
                    se_linux_options: value_se_linux_options,
                })
            }
        }

        deserializer.deserialize_struct(
            "SELinuxStrategyOptions",
            &[
                "rule",
                "seLinuxOptions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for SELinuxStrategyOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SELinuxStrategyOptions",
            0 +
            1 +
            self.se_linux_options.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", &self.rule)?;
        if let Some(value) = &self.se_linux_options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "seLinuxOptions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
