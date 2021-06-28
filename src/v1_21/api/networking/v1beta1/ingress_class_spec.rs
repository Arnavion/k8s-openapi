// Generated from definition io.k8s.api.networking.v1beta1.IngressClassSpec

/// IngressClassSpec provides information about the class of an Ingress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressClassSpec {
    /// Controller refers to the name of the controller that should handle this class. This allows for different "flavors" that are controlled by the same controller. For example, you may have different Parameters for the same implementing controller. This should be specified as a domain-prefixed path no more than 250 characters in length, e.g. "acme.io/ingress-controller". This field is immutable.
    pub controller: Option<String>,

    /// Parameters is a link to a custom resource containing additional configuration for the controller. This is optional if the controller does not require extra parameters.
    pub parameters: Option<crate::api::networking::v1beta1::IngressClassParametersReference>,
}

impl<'de> crate::serde::Deserialize<'de> for IngressClassSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_controller,
            Key_parameters,
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
                            "controller" => Field::Key_controller,
                            "parameters" => Field::Key_parameters,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressClassSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressClassSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_controller: Option<String> = None;
                let mut value_parameters: Option<crate::api::networking::v1beta1::IngressClassParametersReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_controller => value_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressClassSpec {
                    controller: value_controller,
                    parameters: value_parameters,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressClassSpec",
            &[
                "controller",
                "parameters",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressClassSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressClassSpec",
            self.controller.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        if let Some(value) = &self.parameters {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for IngressClassSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "IngressClassSpec provides information about the class of an Ingress.",
          "properties": {
            "controller": {
              "description": "Controller refers to the name of the controller that should handle this class. This allows for different \"flavors\" that are controlled by the same controller. For example, you may have different Parameters for the same implementing controller. This should be specified as a domain-prefixed path no more than 250 characters in length, e.g. \"acme.io/ingress-controller\". This field is immutable.",
              "type": "string"
            },
            "parameters": crate::schema_ref_with_description(crate::api::networking::v1beta1::IngressClassParametersReference::schema(), "Parameters is a link to a custom resource containing additional configuration for the controller. This is optional if the controller does not require extra parameters.")
          },
          "type": "object"
        })
    }
}
