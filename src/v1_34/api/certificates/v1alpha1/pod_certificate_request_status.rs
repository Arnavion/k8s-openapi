// Generated from definition io.k8s.api.certificates.v1alpha1.PodCertificateRequestStatus

/// PodCertificateRequestStatus describes the status of the request, and holds the certificate data if the request is issued.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodCertificateRequestStatus {
    /// beginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.  This field is set via the /status subresource, and must be set at the same time as certificateChain.  Once populated, this field is immutable.
    ///
    /// This field is only a hint.  Kubelet may start refreshing before or after this time if necessary.
    pub begin_refresh_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// certificateChain is populated with an issued certificate by the signer. This field is set via the /status subresource. Once populated, this field is immutable.
    ///
    /// If the certificate signing request is denied, a condition of type "Denied" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type "Failed" is added and this field remains empty.
    ///
    /// Validation requirements:
    ///  1. certificateChain must consist of one or more PEM-formatted certificates.
    ///  2. Each entry must be a valid PEM-wrapped, DER-encoded ASN.1 Certificate as
    ///     described in section 4 of RFC5280.
    ///
    /// If more than one block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.  When projecting the chain into a pod volume, kubelet will drop any data in-between the PEM blocks, as well as any PEM block headers.
    pub certificate_chain: Option<std::string::String>,

    /// conditions applied to the request.
    ///
    /// The types "Issued", "Denied", and "Failed" have special handling.  At most one of these conditions may be present, and they must have status "True".
    ///
    /// If the request is denied with `Reason=UnsupportedKeyType`, the signer may suggest a key type that will work in the message field.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>>,

    /// notAfter is the time at which the certificate expires.  The value must be the same as the notAfter value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable.  The signer must set this field at the same time it sets certificateChain.
    pub not_after: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// notBefore is the time at which the certificate becomes valid.  The value must be the same as the notBefore value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable. The signer must set this field at the same time it sets certificateChain.
    pub not_before: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for PodCertificateRequestStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.begin_refresh_at, other.begin_refresh_at);
        crate::DeepMerge::merge_from(&mut self.certificate_chain, other.certificate_chain);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.not_after, other.not_after);
        crate::DeepMerge::merge_from(&mut self.not_before, other.not_before);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodCertificateRequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_begin_refresh_at,
            Key_certificate_chain,
            Key_conditions,
            Key_not_after,
            Key_not_before,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "beginRefreshAt" => Field::Key_begin_refresh_at,
                            "certificateChain" => Field::Key_certificate_chain,
                            "conditions" => Field::Key_conditions,
                            "notAfter" => Field::Key_not_after,
                            "notBefore" => Field::Key_not_before,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodCertificateRequestStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodCertificateRequestStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_begin_refresh_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_certificate_chain: Option<std::string::String> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::Condition>> = None;
                let mut value_not_after: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_not_before: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_begin_refresh_at => value_begin_refresh_at = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_certificate_chain => value_certificate_chain = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not_after => value_not_after = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not_before => value_not_before = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodCertificateRequestStatus {
                    begin_refresh_at: value_begin_refresh_at,
                    certificate_chain: value_certificate_chain,
                    conditions: value_conditions,
                    not_after: value_not_after,
                    not_before: value_not_before,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodCertificateRequestStatus",
            &[
                "beginRefreshAt",
                "certificateChain",
                "conditions",
                "notAfter",
                "notBefore",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodCertificateRequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodCertificateRequestStatus",
            self.begin_refresh_at.as_ref().map_or(0, |_| 1) +
            self.certificate_chain.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.not_after.as_ref().map_or(0, |_| 1) +
            self.not_before.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.begin_refresh_at {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "beginRefreshAt", value)?;
        }
        if let Some(value) = &self.certificate_chain {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "certificateChain", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.not_after {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "notAfter", value)?;
        }
        if let Some(value) = &self.not_before {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "notBefore", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodCertificateRequestStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.certificates.v1alpha1.PodCertificateRequestStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodCertificateRequestStatus describes the status of the request, and holds the certificate data if the request is issued.",
            "type": "object",
            "properties": {
                "beginRefreshAt": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "beginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.  This field is set via the /status subresource, and must be set at the same time as certificateChain.  Once populated, this field is immutable.\n\nThis field is only a hint.  Kubelet may start refreshing before or after this time if necessary.".into());
                    schema_obj
                }),
                "certificateChain": {
                    "description": "certificateChain is populated with an issued certificate by the signer. This field is set via the /status subresource. Once populated, this field is immutable.\n\nIf the certificate signing request is denied, a condition of type \"Denied\" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type \"Failed\" is added and this field remains empty.\n\nValidation requirements:\n 1. certificateChain must consist of one or more PEM-formatted certificates.\n 2. Each entry must be a valid PEM-wrapped, DER-encoded ASN.1 Certificate as\n    described in section 4 of RFC5280.\n\nIf more than one block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.  When projecting the chain into a pod volume, kubelet will drop any data in-between the PEM blocks, as well as any PEM block headers.",
                    "type": "string",
                },
                "conditions": {
                    "description": "conditions applied to the request.\n\nThe types \"Issued\", \"Denied\", and \"Failed\" have special handling.  At most one of these conditions may be present, and they must have status \"True\".\n\nIf the request is denied with `Reason=UnsupportedKeyType`, the signer may suggest a key type that will work in the message field.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Condition>()),
                },
                "notAfter": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "notAfter is the time at which the certificate expires.  The value must be the same as the notAfter value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable.  The signer must set this field at the same time it sets certificateChain.".into());
                    schema_obj
                }),
                "notBefore": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "notBefore is the time at which the certificate becomes valid.  The value must be the same as the notBefore value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable. The signer must set this field at the same time it sets certificateChain.".into());
                    schema_obj
                }),
            },
        })
    }
}
