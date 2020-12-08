// Generated from definition io.k8s.api.flowcontrol.v1beta1.ServiceAccountSubject

/// ServiceAccountSubject holds detailed information for service-account-kind subject.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountSubject {
    /// `name` is the name of matching ServiceAccount objects, or "*" to match regardless of name. Required.
    pub name: String,

    /// `namespace` is the namespace of matching ServiceAccount objects. Required.
    pub namespace: String,
}

impl<'de> serde::Deserialize<'de> for ServiceAccountSubject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_namespace,
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
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceAccountSubject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceAccountSubject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccountSubject {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace.ok_or_else(|| serde::de::Error::missing_field("namespace"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceAccountSubject",
            &[
                "name",
                "namespace",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ServiceAccountSubject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceAccountSubject",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", &self.namespace)?;
        serde::ser::SerializeStruct::end(state)
    }
}
