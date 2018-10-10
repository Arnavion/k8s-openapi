// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceSubresources

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceSubresources {
    /// Scale denotes the scale subresource for CustomResources
    pub scale: Option<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale>,

    /// Status denotes the status subresource for CustomResources
    pub status: Option<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus>,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceSubresources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_scale,
            Key_status,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceSubresources;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceSubresources")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_scale: Option<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceScale> = None;
                let mut value_status: Option<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_scale => value_scale = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
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

impl ::serde::Serialize for CustomResourceSubresources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceSubresources",
            0 +
            self.scale.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.scale {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "scale", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
