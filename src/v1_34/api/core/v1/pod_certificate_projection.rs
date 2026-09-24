// Generated from definition io.k8s.api.core.v1.PodCertificateProjection

/// PodCertificateProjection provides a private key and X.509 certificate in the pod filesystem.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodCertificateProjection {
    /// Write the certificate chain at this path in the projected volume.
    ///
    /// Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.
    pub certificate_chain_path: Option<std::string::String>,

    /// Write the credential bundle at this path in the projected volume.
    ///
    /// The credential bundle is a single file that contains multiple PEM blocks. The first PEM block is a PRIVATE KEY block, containing a PKCS#8 private key.
    ///
    /// The remaining blocks are CERTIFICATE blocks, containing the issued certificate chain from the signer (leaf and any intermediates).
    ///
    /// Using credentialBundlePath lets your Pod's application code make a single atomic read that retrieves a consistent key and certificate chain.  If you project them to separate files, your application code will need to additionally check that the leaf certificate was issued to the key.
    pub credential_bundle_path: Option<std::string::String>,

    /// Write the key at this path in the projected volume.
    ///
    /// Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.
    pub key_path: Option<std::string::String>,

    /// The type of keypair Kubelet will generate for the pod.
    ///
    /// Valid values are "RSA3072", "RSA4096", "ECDSAP256", "ECDSAP384", "ECDSAP521", and "ED25519".
    pub key_type: std::string::String,

    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    ///
    /// Kubelet copies this value verbatim into the PodCertificateRequests it generates for this projection.
    ///
    /// If omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).
    ///
    /// The signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.
    pub max_expiration_seconds: Option<i32>,

    /// Kubelet's generated CSRs will be addressed to this signer.
    pub signer_name: std::string::String,
}

impl crate::DeepMerge for PodCertificateProjection {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.certificate_chain_path, other.certificate_chain_path);
        crate::DeepMerge::merge_from(&mut self.credential_bundle_path, other.credential_bundle_path);
        crate::DeepMerge::merge_from(&mut self.key_path, other.key_path);
        crate::DeepMerge::merge_from(&mut self.key_type, other.key_type);
        crate::DeepMerge::merge_from(&mut self.max_expiration_seconds, other.max_expiration_seconds);
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodCertificateProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_certificate_chain_path,
            Key_credential_bundle_path,
            Key_key_path,
            Key_key_type,
            Key_max_expiration_seconds,
            Key_signer_name,
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
                            "certificateChainPath" => Field::Key_certificate_chain_path,
                            "credentialBundlePath" => Field::Key_credential_bundle_path,
                            "keyPath" => Field::Key_key_path,
                            "keyType" => Field::Key_key_type,
                            "maxExpirationSeconds" => Field::Key_max_expiration_seconds,
                            "signerName" => Field::Key_signer_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodCertificateProjection;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodCertificateProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_certificate_chain_path: Option<std::string::String> = None;
                let mut value_credential_bundle_path: Option<std::string::String> = None;
                let mut value_key_path: Option<std::string::String> = None;
                let mut value_key_type: Option<std::string::String> = None;
                let mut value_max_expiration_seconds: Option<i32> = None;
                let mut value_signer_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_certificate_chain_path => value_certificate_chain_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_credential_bundle_path => value_credential_bundle_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key_path => value_key_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key_type => value_key_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_expiration_seconds => value_max_expiration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodCertificateProjection {
                    certificate_chain_path: value_certificate_chain_path,
                    credential_bundle_path: value_credential_bundle_path,
                    key_path: value_key_path,
                    key_type: value_key_type.unwrap_or_default(),
                    max_expiration_seconds: value_max_expiration_seconds,
                    signer_name: value_signer_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PodCertificateProjection",
            &[
                "certificateChainPath",
                "credentialBundlePath",
                "keyPath",
                "keyType",
                "maxExpirationSeconds",
                "signerName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodCertificateProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodCertificateProjection",
            2 +
            self.certificate_chain_path.as_ref().map_or(0, |_| 1) +
            self.credential_bundle_path.as_ref().map_or(0, |_| 1) +
            self.key_path.as_ref().map_or(0, |_| 1) +
            self.max_expiration_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.certificate_chain_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "certificateChainPath", value)?;
        }
        if let Some(value) = &self.credential_bundle_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "credentialBundlePath", value)?;
        }
        if let Some(value) = &self.key_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "keyPath", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "keyType", &self.key_type)?;
        if let Some(value) = &self.max_expiration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxExpirationSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", &self.signer_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodCertificateProjection {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.PodCertificateProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodCertificateProjection provides a private key and X.509 certificate in the pod filesystem.",
            "type": "object",
            "properties": {
                "certificateChainPath": {
                    "description": "Write the certificate chain at this path in the projected volume.\n\nMost applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.",
                    "type": "string",
                },
                "credentialBundlePath": {
                    "description": "Write the credential bundle at this path in the projected volume.\n\nThe credential bundle is a single file that contains multiple PEM blocks. The first PEM block is a PRIVATE KEY block, containing a PKCS#8 private key.\n\nThe remaining blocks are CERTIFICATE blocks, containing the issued certificate chain from the signer (leaf and any intermediates).\n\nUsing credentialBundlePath lets your Pod's application code make a single atomic read that retrieves a consistent key and certificate chain.  If you project them to separate files, your application code will need to additionally check that the leaf certificate was issued to the key.",
                    "type": "string",
                },
                "keyPath": {
                    "description": "Write the key at this path in the projected volume.\n\nMost applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.",
                    "type": "string",
                },
                "keyType": {
                    "description": "The type of keypair Kubelet will generate for the pod.\n\nValid values are \"RSA3072\", \"RSA4096\", \"ECDSAP256\", \"ECDSAP384\", \"ECDSAP521\", and \"ED25519\".",
                    "type": "string",
                },
                "maxExpirationSeconds": {
                    "description": "maxExpirationSeconds is the maximum lifetime permitted for the certificate.\n\nKubelet copies this value verbatim into the PodCertificateRequests it generates for this projection.\n\nIf omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).\n\nThe signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.",
                    "type": "integer",
                    "format": "int32",
                },
                "signerName": {
                    "description": "Kubelet's generated CSRs will be addressed to this signer.",
                    "type": "string",
                },
            },
            "required": [
                "keyType",
                "signerName",
            ],
        })
    }
}
