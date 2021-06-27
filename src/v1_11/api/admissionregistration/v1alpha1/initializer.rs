// Generated from definition io.k8s.api.admissionregistration.v1alpha1.Initializer

/// Initializer describes the name and the failure policy of an initializer, and what resources it applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Initializer {
    /// Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where "alwayspullimages" is the name of the webhook, and kubernetes.io is the name of the organization. Required
    pub name: String,

    /// Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources.
    pub rules: Vec<crate::api::admissionregistration::v1alpha1::Rule>,
}

impl<'de> crate::serde::Deserialize<'de> for Initializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_rules,
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
                            "name" => Field::Key_name,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Initializer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Initializer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_rules: Option<Vec<crate::api::admissionregistration::v1alpha1::Rule>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Initializer {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    rules: value_rules.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Initializer",
            &[
                "name",
                "rules",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Initializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Initializer",
            1 +
            usize::from(!self.rules.is_empty()),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if !self.rules.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl Initializer {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Initializer describes the name and the failure policy of an initializer, and what resources it applies to.",
          "properties": {
            "name": {
              "description": "Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where \"alwayspullimages\" is the name of the webhook, and kubernetes.io is the name of the organization. Required",
              "type": "string"
            },
            "rules": {
              "description": "Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources.",
              "items": crate::api::admissionregistration::v1alpha1::Rule::schema(),
              "type": "array"
            }
          },
          "required": [
            "name"
          ],
          "type": "object"
        })
    }
}
