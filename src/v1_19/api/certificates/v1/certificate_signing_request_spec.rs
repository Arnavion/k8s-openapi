// Generated from definition io.k8s.api.certificates.v1.CertificateSigningRequestSpec

/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestSpec {
    /// extra contains extra attributes of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub extra: Option<std::collections::BTreeMap<String, Vec<String>>>,

    /// groups contains group membership of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub groups: Option<Vec<String>>,

    /// request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block. When serialized as JSON or YAML, the data is additionally base64-encoded.
    pub request: crate::ByteString,

    /// signerName indicates the requested signer, and is a qualified name.
    ///
    /// List/watch requests for CertificateSigningRequests can filter on this field using a "spec.signerName=NAME" fieldSelector.
    ///
    /// Well-known Kubernetes signers are:
    ///  1. "kubernetes.io/kube-apiserver-client": issues client certificates that can be used to authenticate to kube-apiserver.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  2. "kubernetes.io/kube-apiserver-client-kubelet": issues client certificates that kubelets use to authenticate to kube-apiserver.
    ///   Requests for this signer can be auto-approved by the "csrapproving" controller in kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  3. "kubernetes.io/kubelet-serving" issues serving certificates that kubelets use to serve TLS endpoints, which kube-apiserver can connect to securely.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///
    /// More details are available at https://k8s.io/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers
    ///
    /// Custom signerNames can also be specified. The signer defines:
    ///  1. Trust distribution: how trust (CA bundles) are distributed.
    ///  2. Permitted subjects: and behavior when a disallowed subject is requested.
    ///  3. Required, permitted, or forbidden x509 extensions in the request (including whether subjectAltNames are allowed, which types, restrictions on allowed values) and behavior when a disallowed extension is requested.
    ///  4. Required, permitted, or forbidden key usages / extended key usages.
    ///  5. Expiration/certificate lifetime: whether it is fixed by the signer, configurable by the admin.
    ///  6. Whether or not requests for CA certificates are allowed.
    pub signer_name: String,

    /// uid contains the uid of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub uid: Option<String>,

    /// usages specifies a set of key usages requested in the issued certificate.
    ///
    /// Requests for TLS client certificates typically request: "digital signature", "key encipherment", "client auth".
    ///
    /// Requests for TLS serving certificates typically request: "key encipherment", "digital signature", "server auth".
    ///
    /// Valid values are:
    ///  "signing", "digital signature", "content commitment",
    ///  "key encipherment", "key agreement", "data encipherment",
    ///  "cert sign", "crl sign", "encipher only", "decipher only", "any",
    ///  "server auth", "client auth",
    ///  "code signing", "email protection", "s/mime",
    ///  "ipsec end system", "ipsec tunnel", "ipsec user",
    ///  "timestamping", "ocsp signing", "microsoft sgc", "netscape sgc"
    pub usages: Option<Vec<String>>,

    /// username contains the name of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    pub username: Option<String>,
}

impl<'de> serde::Deserialize<'de> for CertificateSigningRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_extra,
            Key_groups,
            Key_request,
            Key_signer_name,
            Key_uid,
            Key_usages,
            Key_username,
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
                            "extra" => Field::Key_extra,
                            "groups" => Field::Key_groups,
                            "request" => Field::Key_request,
                            "signerName" => Field::Key_signer_name,
                            "uid" => Field::Key_uid,
                            "usages" => Field::Key_usages,
                            "username" => Field::Key_username,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CertificateSigningRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_request: Option<crate::ByteString> = None;
                let mut value_signer_name: Option<String> = None;
                let mut value_uid: Option<String> = None;
                let mut value_usages: Option<Vec<String>> = None;
                let mut value_username: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_signer_name => value_signer_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_uid => value_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_usages => value_usages = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestSpec {
                    extra: value_extra,
                    groups: value_groups,
                    request: value_request.ok_or_else(|| serde::de::Error::missing_field("request"))?,
                    signer_name: value_signer_name.ok_or_else(|| serde::de::Error::missing_field("signerName"))?,
                    uid: value_uid,
                    usages: value_usages,
                    username: value_username,
                })
            }
        }

        deserializer.deserialize_struct(
            "CertificateSigningRequestSpec",
            &[
                "extra",
                "groups",
                "request",
                "signerName",
                "uid",
                "usages",
                "username",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CertificateSigningRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestSpec",
            2 +
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.usages.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", &self.signer_name)?;
        if let Some(value) = &self.uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.usages {
            serde::ser::SerializeStruct::serialize_field(&mut state, "usages", value)?;
        }
        if let Some(value) = &self.username {
            serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
