// Generated from definition io.k8s.api.core.v1.AzureFileVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureFileVolumeSource {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// the name of secret that contains Azure Storage Account Name and Key
    pub secret_name: String,

    /// Share Name
    pub share_name: String,
}

impl<'de> ::serde::Deserialize<'de> for AzureFileVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_read_only,
            Key_secret_name,
            Key_share_name,
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
                            "readOnly" => Field::Key_read_only,
                            "secretName" => Field::Key_secret_name,
                            "shareName" => Field::Key_share_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AzureFileVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct AzureFileVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;
                let mut value_share_name: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_share_name => value_share_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureFileVolumeSource {
                    read_only: value_read_only,
                    secret_name: value_secret_name.ok_or_else(|| ::serde::de::Error::missing_field("secretName"))?,
                    share_name: value_share_name.ok_or_else(|| ::serde::de::Error::missing_field("shareName"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AzureFileVolumeSource",
            &[
                "readOnly",
                "secretName",
                "shareName",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for AzureFileVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureFileVolumeSource",
            0 +
            self.read_only.as_ref().map_or(0, |_| 1) +
            1 +
            1,
        )?;
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", &self.secret_name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "shareName", &self.share_name)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
