// Generated from definition io.k8s.api.autoscaling.v1.ScaleStatus

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    pub replicas: i32,

    /// label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    pub selector: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ScaleStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_replicas,
            Key_selector,
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
                            "replicas" => Field::Key_replicas,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ScaleStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ScaleStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_replicas: Option<i32> = None;
                let mut value_selector: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_replicas => value_replicas = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScaleStatus {
                    replicas: value_replicas.ok_or_else(|| crate::serde::de::Error::missing_field("replicas"))?,
                    selector: value_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScaleStatus",
            &[
                "replicas",
                "selector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ScaleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScaleStatus",
            1 +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ScaleStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ScaleStatus represents the current status of a scale subresource.",
          "properties": {
            "replicas": {
              "description": "actual number of observed instances of the scaled object.",
              "format": "int32",
              "type": "integer"
            },
            "selector": {
              "description": "label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors",
              "type": "string"
            }
          },
          "required": [
            "replicas"
          ],
          "type": "object"
        })
    }
}
