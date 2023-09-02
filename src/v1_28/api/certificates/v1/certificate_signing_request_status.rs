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

impl crate::DeepMerge for CertificateSigningRequestStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.certificate, other.certificate);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for CertificateSigningRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_certificate,
            Key_conditions,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CertificateSigningRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_certificate: Option<crate::ByteString> = None;
                let mut value_conditions: Option<Vec<crate::api::certificates::v1::CertificateSigningRequestCondition>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_certificate => value_certificate = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
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

impl crate::serde::Serialize for CertificateSigningRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestStatus",
            self.certificate.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.certificate {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "certificate", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CertificateSigningRequestStatus {
    fn schema_name() -> String {
        "io.k8s.api.certificates.v1.CertificateSigningRequestStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CertificateSigningRequestStatus contains conditions used to indicate approved/denied/failed status of the request, and the issued certificate.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "certificate".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("certificate is populated with an issued certificate by the signer after an Approved condition is present. This field is set via the /status subresource. Once populated, this field is immutable.\n\nIf the certificate signing request is denied, a condition of type \"Denied\" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type \"Failed\" is added and this field remains empty.\n\nValidation requirements:\n 1. certificate must contain one or more PEM blocks.\n 2. All PEM blocks must have the \"CERTIFICATE\" label, contain no headers, and the encoded data\n  must be a BER-encoded ASN.1 Certificate structure as described in section 4 of RFC5280.\n 3. Non-PEM content may appear before or after the \"CERTIFICATE\" PEM blocks and is unvalidated,\n  to allow for explanatory text as described in section 5.2 of RFC7468.\n\nIf more than one PEM block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.\n\nThe certificate is encoded in PEM format.\n\nWhen serialized as JSON or YAML, the data is additionally base64-encoded, so it consists of:\n\n    base64(\n    -----BEGIN CERTIFICATE-----\n    ...\n    -----END CERTIFICATE-----\n    )".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            format: Some("byte".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conditions applied to the request. Known conditions are \"Approved\", \"Denied\", and \"Failed\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::certificates::v1::CertificateSigningRequestCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
