// Generated from definition io.k8s.api.certificates.v1.CertificateSigningRequestStatus

/// CertificateSigningRequestStatus contains conditions used to indicate approved/denied/failed status of the request, and the issued certificate.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestStatus {
    /// certificate is populated with an issued certificate by the signer after an Approved condition is present. This field is set via the /status subresource. Once populated, this field is immutable.
    ///
    /// If the certificate signing request is denied, a condition of type "Denied" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type "Failed" is added and this field remains empty.
    ///
    /// Validation requirements:
    ///  1. certificate must contain one or more PEM blocks.
    ///  2. All PEM blocks must have the "CERTIFICATE" label, contain no headers, and the encoded data
    ///   must be a BER-encoded ASN.1 Certificate structure as described in section 4 of RFC5280.
    ///  3. Non-PEM content may appear before or after the "CERTIFICATE" PEM blocks and is unvalidated,
    ///   to allow for explanatory text as described in section 5.2 of RFC7468.
    ///
    /// If more than one PEM block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.
    ///
    /// The certificate is encoded in PEM format.
    ///
    /// When serialized as JSON or YAML, the data is additionally base64-encoded, so it consists of:
    ///
    ///   base64(
    ///     -----BEGIN CERTIFICATE-----
    ///     ...
    ///     -----END CERTIFICATE-----
    ///     )
    pub certificate: Option<crate::ByteString>,

    /// conditions applied to the request. Known conditions are "Approved", "Denied", and "Failed".
    pub conditions: Option<Vec<crate::api::certificates::v1::CertificateSigningRequestCondition>>,
}

impl<'de> serde::Deserialize<'de> for CertificateSigningRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_certificate,
            Key_conditions,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CertificateSigningRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_certificate: Option<crate::ByteString> = None;
                let mut value_conditions: Option<Vec<crate::api::certificates::v1::CertificateSigningRequestCondition>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_certificate => value_certificate = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
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

impl serde::Serialize for CertificateSigningRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestStatus",
            self.certificate.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.certificate {
            serde::ser::SerializeStruct::serialize_field(&mut state, "certificate", value)?;
        }
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
