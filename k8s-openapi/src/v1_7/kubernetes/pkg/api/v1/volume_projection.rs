// Generated from definition io.k8s.kubernetes.pkg.api.v1.VolumeProjection

/// Projection that may be projected along with other supported volume types
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeProjection {
    /// information about the configMap data to project
    pub config_map: Option<::v1_7::kubernetes::pkg::api::v1::ConfigMapProjection>,

    /// information about the downwardAPI data to project
    pub downward_api: Option<::v1_7::kubernetes::pkg::api::v1::DownwardAPIProjection>,

    /// information about the secret data to project
    pub secret: Option<::v1_7::kubernetes::pkg::api::v1::SecretProjection>,
}

impl<'de> ::serde::Deserialize<'de> for VolumeProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_map,
            Key_downward_api,
            Key_secret,
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
                            "configMap" => Field::Key_config_map,
                            "downwardAPI" => Field::Key_downward_api,
                            "secret" => Field::Key_secret,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeProjection;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct VolumeProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_config_map: Option<::v1_7::kubernetes::pkg::api::v1::ConfigMapProjection> = None;
                let mut value_downward_api: Option<::v1_7::kubernetes::pkg::api::v1::DownwardAPIProjection> = None;
                let mut value_secret: Option<::v1_7::kubernetes::pkg::api::v1::SecretProjection> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_map => value_config_map = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_downward_api => value_downward_api = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeProjection {
                    config_map: value_config_map,
                    downward_api: value_downward_api,
                    secret: value_secret,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeProjection",
            &[
                "configMap",
                "downwardAPI",
                "secret",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for VolumeProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeProjection",
            0 +
            (if self.config_map.is_some() { 1 } else { 0 }) +
            (if self.downward_api.is_some() { 1 } else { 0 }) +
            (if self.secret.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.config_map {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", value)?;
        }
        if let Some(value) = &self.downward_api {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "downwardAPI", value)?;
        }
        if let Some(value) = &self.secret {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
