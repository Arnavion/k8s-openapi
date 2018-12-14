// Generated from definition io.k8s.api.core.v1.ServiceAccountTokenProjection

/// ServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountTokenProjection {
    /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
    pub audience: Option<String>,

    /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
    pub expiration_seconds: Option<i64>,

    /// Path is the path relative to the mount point of the file to project the token into.
    pub path: String,
}

impl<'de> ::serde::Deserialize<'de> for ServiceAccountTokenProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audience,
            Key_expiration_seconds,
            Key_path,
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
                            "audience" => Field::Key_audience,
                            "expirationSeconds" => Field::Key_expiration_seconds,
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceAccountTokenProjection;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ServiceAccountTokenProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_audience: Option<String> = None;
                let mut value_expiration_seconds: Option<i64> = None;
                let mut value_path: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audience => value_audience = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expiration_seconds => value_expiration_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccountTokenProjection {
                    audience: value_audience,
                    expiration_seconds: value_expiration_seconds,
                    path: value_path.ok_or_else(|| ::serde::de::Error::missing_field("path"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceAccountTokenProjection",
            &[
                "audience",
                "expirationSeconds",
                "path",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ServiceAccountTokenProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceAccountTokenProjection",
            0 +
            self.audience.as_ref().map_or(0, |_| 1) +
            self.expiration_seconds.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.audience {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "audience", value)?;
        }
        if let Some(value) = &self.expiration_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "expirationSeconds", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
