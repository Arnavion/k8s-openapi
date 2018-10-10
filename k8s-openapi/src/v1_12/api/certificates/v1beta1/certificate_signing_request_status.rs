// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestStatus

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestStatus {
    /// If request was approved, the controller will place the issued certificate here.
    pub certificate: Option<::ByteString>,

    /// Conditions applied to the request, such as approval or denial.
    pub conditions: Option<Vec<::v1_12::api::certificates::v1beta1::CertificateSigningRequestCondition>>,
}

impl<'de> ::serde::Deserialize<'de> for CertificateSigningRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_certificate,
            Key_conditions,
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
                            "certificate" => Field::Key_certificate,
                            "conditions" => Field::Key_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CertificateSigningRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_certificate: Option<::ByteString> = None;
                let mut value_conditions: Option<Vec<::v1_12::api::certificates::v1beta1::CertificateSigningRequestCondition>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_certificate => value_certificate = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestStatus {
                    certificate: value_certificate,
                    conditions: value_conditions,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequestStatus",
            &[
                "certificate",
                "conditions",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CertificateSigningRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestStatus",
            0 +
            self.certificate.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.certificate {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "certificate", value)?;
        }
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
