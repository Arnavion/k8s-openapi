// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec

/// This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct CertificateSigningRequestSpec {
    /// Extra information about the requesting user. See user.Info interface for details.
    pub extra: std::collections::BTreeMap<String, Vec<String>>,

    /// Group information about the requesting user. See user.Info interface for details.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub groups: Vec<String>,

    /// Base64-encoded PKCS#10 CSR data
    pub request: crate::ByteString,

    /// Requested signer for the request. It is a qualified name in the form: `scope-hostname.io/name`. If empty, it will be defaulted:
    ///  1. If it's a kubelet client certificate, it is assigned
    ///     "kubernetes.io/kube-apiserver-client-kubelet".
    ///  2. If it's a kubelet serving certificate, it is assigned
    ///     "kubernetes.io/kubelet-serving".
    ///  3. Otherwise, it is assigned "kubernetes.io/legacy-unknown".
    /// Distribution of trust for signers happens out of band. You can select on this field using `spec.signerName`.
    pub signer_name: Option<String>,

    /// UID information about the requesting user. See user.Info interface for details.
    pub uid: Option<String>,

    /// allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///      https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    /// Valid values are:
    ///  "signing",
    ///  "digital signature",
    ///  "content commitment",
    ///  "key encipherment",
    ///  "key agreement",
    ///  "data encipherment",
    ///  "cert sign",
    ///  "crl sign",
    ///  "encipher only",
    ///  "decipher only",
    ///  "any",
    ///  "server auth",
    ///  "client auth",
    ///  "code signing",
    ///  "email protection",
    ///  "s/mime",
    ///  "ipsec end system",
    ///  "ipsec tunnel",
    ///  "ipsec user",
    ///  "timestamping",
    ///  "ocsp signing",
    ///  "microsoft sgc",
    ///  "netscape sgc"
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub usages: Vec<String>,

    /// Information about the requesting user. See user.Info interface for details.
    pub username: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for CertificateSigningRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CertificateSigningRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_extra: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_request: Option<crate::ByteString> = None;
                let mut value_signer_name: Option<String> = None;
                let mut value_uid: Option<String> = None;
                let mut value_usages: Option<Vec<String>> = None;
                let mut value_username: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_usages => value_usages = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestSpec {
                    extra: value_extra.unwrap_or_default(),
                    groups: value_groups.unwrap_or_default(),
                    request: value_request.ok_or_else(|| crate::serde::de::Error::missing_field("request"))?,
                    signer_name: value_signer_name,
                    uid: value_uid,
                    usages: value_usages.unwrap_or_default(),
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

impl crate::serde::Serialize for CertificateSigningRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestSpec",
            1 +
            usize::from(!self.extra.is_empty()) +
            usize::from(!self.groups.is_empty()) +
            self.signer_name.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            usize::from(!self.usages.is_empty()) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if !self.extra.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", &self.extra)?;
        }
        if !self.groups.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", &self.groups)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        if let Some(value) = &self.signer_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if !self.usages.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "usages", &self.usages)?;
        }
        if let Some(value) = &self.username {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
