// Generated from definition io.k8s.api.extensions.v1beta1.HTTPIngressRuleValue

/// HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://\<host\>/\<path\>?\<searchpart\> -\> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressRuleValue {
    /// A collection of paths that map requests to backends.
    pub paths: Vec<crate::api::extensions::v1beta1::HTTPIngressPath>,
}

impl<'de> crate::serde::Deserialize<'de> for HTTPIngressRuleValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_paths,
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
                            "paths" => Field::Key_paths,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPIngressRuleValue;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HTTPIngressRuleValue")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_paths: Option<Vec<crate::api::extensions::v1beta1::HTTPIngressPath>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_paths => value_paths = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressRuleValue {
                    paths: value_paths.ok_or_else(|| crate::serde::de::Error::missing_field("paths"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPIngressRuleValue",
            &[
                "paths",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPIngressRuleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPIngressRuleValue",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paths", &self.paths)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for HTTPIngressRuleValue {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.",
          "properties": {
            "paths": {
              "description": "A collection of paths that map requests to backends.",
              "items": crate::api::extensions::v1beta1::HTTPIngressPath::schema(),
              "type": "array"
            }
          },
          "required": [
            "paths"
          ],
          "type": "object"
        })
    }
}
