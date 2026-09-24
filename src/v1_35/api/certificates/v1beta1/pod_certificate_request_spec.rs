// Generated from definition io.k8s.api.certificates.v1beta1.PodCertificateRequestSpec

/// PodCertificateRequestSpec describes the certificate request.  All fields are immutable after creation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodCertificateRequestSpec {
    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    ///
    /// If omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).
    ///
    /// The signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.
    pub max_expiration_seconds: Option<i32>,

    /// nodeName is the name of the node the pod is assigned to.
    pub node_name: std::string::String,

    /// nodeUID is the UID of the node the pod is assigned to.
    pub node_uid: std::string::String,

    /// pkixPublicKey is the PKIX-serialized public key the signer will issue the certificate to.
    ///
    /// The key must be one of RSA3072, RSA4096, ECDSAP256, ECDSAP384, ECDSAP521, or ED25519. Note that this list may be expanded in the future.
    ///
    /// Signer implementations do not need to support all key types supported by kube-apiserver and kubelet.  If a signer does not support the key type used for a given PodCertificateRequest, it must deny the request by setting a status.conditions entry with a type of "Denied" and a reason of "UnsupportedKeyType". It may also suggest a key type that it does support in the message field.
    pub pkix_public_key: crate::ByteString,

    /// podName is the name of the pod into which the certificate will be mounted.
    pub pod_name: std::string::String,

    /// podUID is the UID of the pod into which the certificate will be mounted.
    pub pod_uid: std::string::String,

    /// proofOfPossession proves that the requesting kubelet holds the private key corresponding to pkixPublicKey.
    ///
    /// It is contructed by signing the ASCII bytes of the pod's UID using `pkixPublicKey`.
    ///
    /// kube-apiserver validates the proof of possession during creation of the PodCertificateRequest.
    ///
    /// If the key is an RSA key, then the signature is over the ASCII bytes of the pod UID, using RSASSA-PSS from RFC 8017 (as implemented by the golang function crypto/rsa.SignPSS with nil options).
    ///
    /// If the key is an ECDSA key, then the signature is as described by \[SEC 1, Version 2.0\](https://www.secg.org/sec1-v2.pdf) (as implemented by the golang library function crypto/ecdsa.SignASN1)
    ///
    /// If the key is an ED25519 key, the the signature is as described by the \[ED25519 Specification\](https://ed25519.cr.yp.to/) (as implemented by the golang library crypto/ed25519.Sign).
    pub proof_of_possession: crate::ByteString,

    /// serviceAccountName is the name of the service account the pod is running as.
    pub service_account_name: std::string::String,

    /// serviceAccountUID is the UID of the service account the pod is running as.
    pub service_account_uid: std::string::String,

    /// signerName indicates the requested signer.
    ///
    /// All signer names beginning with `kubernetes.io` are reserved for use by the Kubernetes project.  There is currently one well-known signer documented by the Kubernetes project, `kubernetes.io/kube-apiserver-client-pod`, which will issue client certificates understood by kube-apiserver.  It is currently unimplemented.
    pub signer_name: std::string::String,

    /// unverifiedUserAnnotations allow pod authors to pass additional information to the signer implementation.  Kubernetes does not restrict or validate this metadata in any way.
    ///
    /// Entries are subject to the same validation as object metadata annotations, with the addition that all keys must be domain-prefixed. No restrictions are placed on values, except an overall size limitation on the entire field.
    ///
    /// Signers should document the keys and values they support.  Signers should deny requests that contain keys they do not recognize.
    pub unverified_user_annotations: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,
}

impl crate::DeepMerge for PodCertificateRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.max_expiration_seconds, other.max_expiration_seconds);
        crate::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        crate::DeepMerge::merge_from(&mut self.node_uid, other.node_uid);
        crate::DeepMerge::merge_from(&mut self.pkix_public_key, other.pkix_public_key);
        crate::DeepMerge::merge_from(&mut self.pod_name, other.pod_name);
        crate::DeepMerge::merge_from(&mut self.pod_uid, other.pod_uid);
        crate::DeepMerge::merge_from(&mut self.proof_of_possession, other.proof_of_possession);
        crate::DeepMerge::merge_from(&mut self.service_account_name, other.service_account_name);
        crate::DeepMerge::merge_from(&mut self.service_account_uid, other.service_account_uid);
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        crate::merge_strategies::map::granular(&mut self.unverified_user_annotations, other.unverified_user_annotations, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodCertificateRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max_expiration_seconds,
            Key_node_name,
            Key_node_uid,
            Key_pkix_public_key,
            Key_pod_name,
            Key_pod_uid,
            Key_proof_of_possession,
            Key_service_account_name,
            Key_service_account_uid,
            Key_signer_name,
            Key_unverified_user_annotations,
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
                            "maxExpirationSeconds" => Field::Key_max_expiration_seconds,
                            "nodeName" => Field::Key_node_name,
                            "nodeUID" => Field::Key_node_uid,
                            "pkixPublicKey" => Field::Key_pkix_public_key,
                            "podName" => Field::Key_pod_name,
                            "podUID" => Field::Key_pod_uid,
                            "proofOfPossession" => Field::Key_proof_of_possession,
                            "serviceAccountName" => Field::Key_service_account_name,
                            "serviceAccountUID" => Field::Key_service_account_uid,
                            "signerName" => Field::Key_signer_name,
                            "unverifiedUserAnnotations" => Field::Key_unverified_user_annotations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodCertificateRequestSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodCertificateRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max_expiration_seconds: Option<i32> = None;
                let mut value_node_name: Option<std::string::String> = None;
                let mut value_node_uid: Option<std::string::String> = None;
                let mut value_pkix_public_key: Option<crate::ByteString> = None;
                let mut value_pod_name: Option<std::string::String> = None;
                let mut value_pod_uid: Option<std::string::String> = None;
                let mut value_proof_of_possession: Option<crate::ByteString> = None;
                let mut value_service_account_name: Option<std::string::String> = None;
                let mut value_service_account_uid: Option<std::string::String> = None;
                let mut value_signer_name: Option<std::string::String> = None;
                let mut value_unverified_user_annotations: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max_expiration_seconds => value_max_expiration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_uid => value_node_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pkix_public_key => value_pkix_public_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_name => value_pod_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_uid => value_pod_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_proof_of_possession => value_proof_of_possession = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_name => value_service_account_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_uid => value_service_account_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unverified_user_annotations => value_unverified_user_annotations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodCertificateRequestSpec {
                    max_expiration_seconds: value_max_expiration_seconds,
                    node_name: value_node_name.unwrap_or_default(),
                    node_uid: value_node_uid.unwrap_or_default(),
                    pkix_public_key: value_pkix_public_key.unwrap_or_default(),
                    pod_name: value_pod_name.unwrap_or_default(),
                    pod_uid: value_pod_uid.unwrap_or_default(),
                    proof_of_possession: value_proof_of_possession.unwrap_or_default(),
                    service_account_name: value_service_account_name.unwrap_or_default(),
                    service_account_uid: value_service_account_uid.unwrap_or_default(),
                    signer_name: value_signer_name.unwrap_or_default(),
                    unverified_user_annotations: value_unverified_user_annotations,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodCertificateRequestSpec",
            &[
                "maxExpirationSeconds",
                "nodeName",
                "nodeUID",
                "pkixPublicKey",
                "podName",
                "podUID",
                "proofOfPossession",
                "serviceAccountName",
                "serviceAccountUID",
                "signerName",
                "unverifiedUserAnnotations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodCertificateRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodCertificateRequestSpec",
            9 +
            self.max_expiration_seconds.as_ref().map_or(0, |_| 1) +
            self.unverified_user_annotations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.max_expiration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxExpirationSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", &self.node_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeUID", &self.node_uid)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pkixPublicKey", &self.pkix_public_key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podName", &self.pod_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podUID", &self.pod_uid)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "proofOfPossession", &self.proof_of_possession)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountName", &self.service_account_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountUID", &self.service_account_uid)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", &self.signer_name)?;
        if let Some(value) = &self.unverified_user_annotations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unverifiedUserAnnotations", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodCertificateRequestSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.certificates.v1beta1.PodCertificateRequestSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodCertificateRequestSpec describes the certificate request.  All fields are immutable after creation.",
            "type": "object",
            "properties": {
                "maxExpirationSeconds": {
                    "description": "maxExpirationSeconds is the maximum lifetime permitted for the certificate.\n\nIf omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).\n\nThe signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.",
                    "type": "integer",
                    "format": "int32",
                },
                "nodeName": {
                    "description": "nodeName is the name of the node the pod is assigned to.",
                    "type": "string",
                },
                "nodeUID": {
                    "description": "nodeUID is the UID of the node the pod is assigned to.",
                    "type": "string",
                },
                "pkixPublicKey": {
                    "description": "pkixPublicKey is the PKIX-serialized public key the signer will issue the certificate to.\n\nThe key must be one of RSA3072, RSA4096, ECDSAP256, ECDSAP384, ECDSAP521, or ED25519. Note that this list may be expanded in the future.\n\nSigner implementations do not need to support all key types supported by kube-apiserver and kubelet.  If a signer does not support the key type used for a given PodCertificateRequest, it must deny the request by setting a status.conditions entry with a type of \"Denied\" and a reason of \"UnsupportedKeyType\". It may also suggest a key type that it does support in the message field.",
                    "type": "string",
                    "format": "byte",
                },
                "podName": {
                    "description": "podName is the name of the pod into which the certificate will be mounted.",
                    "type": "string",
                },
                "podUID": {
                    "description": "podUID is the UID of the pod into which the certificate will be mounted.",
                    "type": "string",
                },
                "proofOfPossession": {
                    "description": "proofOfPossession proves that the requesting kubelet holds the private key corresponding to pkixPublicKey.\n\nIt is contructed by signing the ASCII bytes of the pod's UID using `pkixPublicKey`.\n\nkube-apiserver validates the proof of possession during creation of the PodCertificateRequest.\n\nIf the key is an RSA key, then the signature is over the ASCII bytes of the pod UID, using RSASSA-PSS from RFC 8017 (as implemented by the golang function crypto/rsa.SignPSS with nil options).\n\nIf the key is an ECDSA key, then the signature is as described by [SEC 1, Version 2.0](https://www.secg.org/sec1-v2.pdf) (as implemented by the golang library function crypto/ecdsa.SignASN1)\n\nIf the key is an ED25519 key, the the signature is as described by the [ED25519 Specification](https://ed25519.cr.yp.to/) (as implemented by the golang library crypto/ed25519.Sign).",
                    "type": "string",
                    "format": "byte",
                },
                "serviceAccountName": {
                    "description": "serviceAccountName is the name of the service account the pod is running as.",
                    "type": "string",
                },
                "serviceAccountUID": {
                    "description": "serviceAccountUID is the UID of the service account the pod is running as.",
                    "type": "string",
                },
                "signerName": {
                    "description": "signerName indicates the requested signer.\n\nAll signer names beginning with `kubernetes.io` are reserved for use by the Kubernetes project.  There is currently one well-known signer documented by the Kubernetes project, `kubernetes.io/kube-apiserver-client-pod`, which will issue client certificates understood by kube-apiserver.  It is currently unimplemented.",
                    "type": "string",
                },
                "unverifiedUserAnnotations": {
                    "description": "unverifiedUserAnnotations allow pod authors to pass additional information to the signer implementation.  Kubernetes does not restrict or validate this metadata in any way.\n\nEntries are subject to the same validation as object metadata annotations, with the addition that all keys must be domain-prefixed. No restrictions are placed on values, except an overall size limitation on the entire field.\n\nSigners should document the keys and values they support.  Signers should deny requests that contain keys they do not recognize.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "nodeName",
                "nodeUID",
                "pkixPublicKey",
                "podName",
                "podUID",
                "proofOfPossession",
                "serviceAccountName",
                "serviceAccountUID",
                "signerName",
            ],
        })
    }
}
