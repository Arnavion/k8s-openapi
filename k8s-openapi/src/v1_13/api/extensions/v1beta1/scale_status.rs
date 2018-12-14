// Generated from definition io.k8s.api.extensions.v1beta1.ScaleStatus

/// represents the current status of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    pub replicas: i32,

    /// label query over pods that should match the replicas count. More info: http://kubernetes.io/docs/user-guide/labels#label-selectors
    pub selector: Option<::std::collections::BTreeMap<String, String>>,

    /// label selector for pods that should match the replicas count. This is a serializated version of both map-based and more expressive set-based selectors. This is done to avoid introspection in the clients. The string will be in the same format as the query-param syntax. If the target type only supports map-based selectors, both this field and map-based selector field are populated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub target_selector: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for ScaleStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_replicas,
            Key_selector,
            Key_target_selector,
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
                            "replicas" => Field::Key_replicas,
                            "selector" => Field::Key_selector,
                            "targetSelector" => Field::Key_target_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScaleStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ScaleStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_replicas: Option<i32> = None;
                let mut value_selector: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_target_selector: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_replicas => value_replicas = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_selector => value_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_selector => value_target_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleStatus {
                    replicas: value_replicas.ok_or_else(|| ::serde::de::Error::missing_field("replicas"))?,
                    selector: value_selector,
                    target_selector: value_target_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScaleStatus",
            &[
                "replicas",
                "selector",
                "targetSelector",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ScaleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleStatus",
            0 +
            1 +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.target_selector.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        if let Some(value) = &self.selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.target_selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "targetSelector", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
