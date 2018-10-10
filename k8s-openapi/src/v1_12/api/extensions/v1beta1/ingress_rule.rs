// Generated from definition io.k8s.api.extensions.v1beta1.IngressRule

/// IngressRule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressRule {
    /// Host is the fully qualified domain name of a network host, as defined by RFC 3986. Note the following deviations from the "host" part of the URI as defined in the RFC: 1. IPs are not allowed. Currently an IngressRuleValue can only apply to the
    /// 	  IP in the Spec of the parent Ingress.
    /// 2. The `:` delimiter is not respected because ports are not allowed.
    /// 	  Currently the port of an Ingress is implicitly :80 for http and
    /// 	  :443 for https.
    /// Both these may change in the future. Incoming requests are matched against the host before the IngressRuleValue. If the host is unspecified, the Ingress routes all traffic based on the specified IngressRuleValue.
    pub host: Option<String>,

    pub http: Option<::v1_12::api::extensions::v1beta1::HTTPIngressRuleValue>,
}

impl<'de> ::serde::Deserialize<'de> for IngressRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_host,
            Key_http,
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
                            "host" => Field::Key_host,
                            "http" => Field::Key_http,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IngressRule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct IngressRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_host: Option<String> = None;
                let mut value_http: Option<::v1_12::api::extensions::v1beta1::HTTPIngressRuleValue> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_host => value_host = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http => value_http = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressRule {
                    host: value_host,
                    http: value_http,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressRule",
            &[
                "host",
                "http",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for IngressRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressRule",
            0 +
            self.host.as_ref().map_or(0, |_| 1) +
            self.http.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.host {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        if let Some(value) = &self.http {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "http", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
