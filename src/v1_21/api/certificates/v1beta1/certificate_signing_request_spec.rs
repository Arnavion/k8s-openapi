// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec

/// This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestSpec {
    /// Extra information about the requesting user. See user.Info interface for details.
    pub extra: Option<std::collections::BTreeMap<String, Vec<String>>>,

    /// Group information about the requesting user. See user.Info interface for details.
    pub groups: Option<Vec<String>>,

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
    pub usages: Option<Vec<String>>,

    /// Information about the requesting user. See user.Info interface for details.
    pub username: Option<String>,
}

impl crate::DeepMerge for CertificateSigningRequestSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.extra, other.extra, |current_item, other_item| {
            crate::merge_strategies::list::atomic(current_item, other_item);
        });
        crate::merge_strategies::list::atomic(&mut self.groups, other.groups);
        crate::DeepMerge::merge_from(&mut self.request, other.request);
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        crate::DeepMerge::merge_from(&mut self.uid, other.uid);
        crate::merge_strategies::list::atomic(&mut self.usages, other.usages);
        crate::DeepMerge::merge_from(&mut self.username, other.username);
    }
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
                        Field::Key_request => value_request = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_usages => value_usages = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestSpec {
                    extra: value_extra,
                    groups: value_groups,
                    request: value_request.unwrap_or_default(),
                    signer_name: value_signer_name,
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

impl crate::serde::Serialize for CertificateSigningRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestSpec",
            1 +
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            self.signer_name.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.usages.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        if let Some(value) = &self.signer_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", value)?;
        }
        if let Some(value) = &self.uid {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.usages {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "usages", value)?;
        }
        if let Some(value) = &self.username {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CertificateSigningRequestSpec {
    fn schema_name() -> String {
        "io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "extra".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Extra information about the requesting user. See user.Info interface for details.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                                        array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                            items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                                crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                                    instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                                    ..Default::default()
                                                })
                                            ))),
                                            ..Default::default()
                                        })),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "groups".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Group information about the requesting user. See user.Info interface for details.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "request".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Base64-encoded PKCS#10 CSR data".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            format: Some("byte".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "signerName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Requested signer for the request. It is a qualified name in the form: `scope-hostname.io/name`. If empty, it will be defaulted:\n 1. If it's a kubelet client certificate, it is assigned\n    \"kubernetes.io/kube-apiserver-client-kubelet\".\n 2. If it's a kubelet serving certificate, it is assigned\n    \"kubernetes.io/kubelet-serving\".\n 3. Otherwise, it is assigned \"kubernetes.io/legacy-unknown\".\nDistribution of trust for signers happens out of band. You can select on this field using `spec.signerName`.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID information about the requesting user. See user.Info interface for details.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "usages".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3\n     https://tools.ietf.org/html/rfc5280#section-4.2.1.12\nValid values are:\n \"signing\",\n \"digital signature\",\n \"content commitment\",\n \"key encipherment\",\n \"key agreement\",\n \"data encipherment\",\n \"cert sign\",\n \"crl sign\",\n \"encipher only\",\n \"decipher only\",\n \"any\",\n \"server auth\",\n \"client auth\",\n \"code signing\",\n \"email protection\",\n \"s/mime\",\n \"ipsec end system\",\n \"ipsec tunnel\",\n \"ipsec user\",\n \"timestamping\",\n \"ocsp signing\",\n \"microsoft sgc\",\n \"netscape sgc\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "username".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Information about the requesting user. See user.Info interface for details.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "request".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
