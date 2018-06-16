// Generated from definition io.k8s.api.certificates.v1beta1.CertificateSigningRequestSpec

/// This information is immutable after the request is created. Only the Request and Usages fields can be set on creation, other fields are derived by Kubernetes and cannot be modified by users.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CertificateSigningRequestSpec {
    /// Extra information about the requesting user. See user.Info interface for details.
    pub extra: Option<::std::collections::BTreeMap<String, Vec<String>>>,

    /// Group information about the requesting user. See user.Info interface for details.
    pub groups: Option<Vec<String>>,

    /// Base64-encoded PKCS#10 CSR data
    pub request: ::ByteString,

    /// UID information about the requesting user. See user.Info interface for details.
    pub uid: Option<String>,

    /// allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///      https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    pub usages: Option<Vec<String>>,

    /// Information about the requesting user. See user.Info interface for details.
    pub username: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for CertificateSigningRequestSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSigningRequestSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CertificateSigningRequestSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_extra: Option<::std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut value_groups: Option<Vec<String>> = None;
                let mut value_request: Option<::ByteString> = None;
                let mut value_uid: Option<String> = None;
                let mut value_usages: Option<Vec<String>> = None;
                let mut value_username: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_extra => value_extra = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_groups => value_groups = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_request => value_request = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_uid => value_uid = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_usages => value_usages = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_username => value_username = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CertificateSigningRequestSpec {
                    extra: value_extra,
                    groups: value_groups,
                    request: value_request.ok_or_else(|| ::serde::de::Error::missing_field("request"))?,
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
                "uid",
                "usages",
                "username",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CertificateSigningRequestSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CertificateSigningRequestSpec",
            0 +
            self.extra.as_ref().map_or(0, |_| 1) +
            self.groups.as_ref().map_or(0, |_| 1) +
            1 +
            self.uid.as_ref().map_or(0, |_| 1) +
            self.usages.as_ref().map_or(0, |_| 1) +
            self.username.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.extra {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "extra", value)?;
        }
        if let Some(value) = &self.groups {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "groups", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "request", &self.request)?;
        if let Some(value) = &self.uid {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        if let Some(value) = &self.usages {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "usages", value)?;
        }
        if let Some(value) = &self.username {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "username", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
