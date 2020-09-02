// Generated from definition io.k8s.api.networking.v1beta1.HTTPIngressRuleValue

/// HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://\<host\>/\<path\>?\<searchpart\> -\> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressRuleValue {
    /// A collection of paths that map requests to backends.
    pub paths: Vec<crate::api::networking::v1beta1::HTTPIngressPath>,
}

impl<'de> serde::Deserialize<'de> for HTTPIngressRuleValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_paths,
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
                            "paths" => Field::Key_paths,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HTTPIngressRuleValue;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HTTPIngressRuleValue")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_paths: Option<Vec<crate::api::networking::v1beta1::HTTPIngressPath>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_paths => value_paths = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressRuleValue {
                    paths: value_paths.ok_or_else(|| serde::de::Error::missing_field("paths"))?,
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

impl serde::Serialize for HTTPIngressRuleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPIngressRuleValue",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "paths", &self.paths)?;
        serde::ser::SerializeStruct::end(state)
    }
}
