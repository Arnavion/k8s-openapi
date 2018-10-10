// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestCondition

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestCondition {
    /// timestamp for the last update to this condition
    pub last_update_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time>,

    /// human readable message with details about the request state
    pub message: Option<String>,

    /// brief reason for the request state
    pub reason: Option<String>,

    /// request approval state, currently Approved or Denied.
    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for CertificateSigningRequestCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_last_update_time,
            Key_message,
            Key_reason,
            Key_type_,
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
                            "lastUpdateTime" => Field::Key_last_update_time,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestCondition;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CertificateSigningRequestCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_last_update_time: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_last_update_time => value_last_update_time = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestCondition {
                    last_update_time: value_last_update_time,
                    message: value_message,
                    reason: value_reason,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequestCondition",
            &[
                "lastUpdateTime",
                "message",
                "reason",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CertificateSigningRequestCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestCondition",
            0 +
            self.last_update_time.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.last_update_time {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "lastUpdateTime", value)?;
        }
        if let Some(value) = &self.message {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
