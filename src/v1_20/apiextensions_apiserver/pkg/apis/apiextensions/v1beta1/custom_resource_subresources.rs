// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceSubresources

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceSubresources {
    /// scale indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object.
    pub scale: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale>,

    /// status indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.
    pub status: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceSubresources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_scale,
            Key_status,
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
                            "scale" => Field::Key_scale,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceSubresources;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceSubresources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_scale: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale> = None;
                let mut value_status: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_scale => value_scale = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceSubresources {
                    scale: value_scale,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceSubresources",
            &[
                "scale",
                "status",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceSubresources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceSubresources",
            self.scale.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.scale {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scale", value)?;
        }
        if let Some(value) = &self.status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CustomResourceSubresources {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CustomResourceSubresources defines the status and scale subresources for CustomResources.",
          "properties": {
            "scale": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale::schema(), "scale indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object."),
            "status": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus::schema(), "status indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.")
          },
          "type": "object"
        })
    }
}
