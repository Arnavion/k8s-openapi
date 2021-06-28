// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec

/// This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestSpec {
    /// Extra information about the requesting user. See user.Info interface for details.
    pub extra: std::collections::BTreeMap<String, Vec<String>>,

    /// Group information about the requesting user. See user.Info interface for details.
    pub groups: Vec<String>,

    /// Base64-encoded PKCS#10 CSR data
    pub request: crate::ByteString,

    /// UID information about the requesting user. See user.Info interface for details.
    pub uid: Option<String>,

    /// allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///      https://tools.ietf.org/html/rfc5280#section-4.2.1.12
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
                let mut value_uid: Option<String> = None;
                let mut value_usages: Option<Vec<String>> = None;
                let mut value_username: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
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

#[cfg(feature = "schema")]
impl crate::Schema for CertificateSigningRequestSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.",
          "properties": {
            "extra": {
              "additionalProperties": {
                "items": {
                  "type": "string"
                },
                "type": "array"
              },
              "description": "Extra information about the requesting user. See user.Info interface for details.",
              "type": "object"
            },
            "groups": {
              "description": "Group information about the requesting user. See user.Info interface for details.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "request": {
              "description": "Base64-encoded PKCS#10 CSR data",
              "format": "byte",
              "type": "string"
            },
            "uid": {
              "description": "UID information about the requesting user. See user.Info interface for details.",
              "type": "string"
            },
            "usages": {
              "description": "allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3\n     https://tools.ietf.org/html/rfc5280#section-4.2.1.12",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "username": {
              "description": "Information about the requesting user. See user.Info interface for details.",
              "type": "string"
            }
          },
          "required": [
            "request"
          ],
          "type": "object"
        })
    }
}
