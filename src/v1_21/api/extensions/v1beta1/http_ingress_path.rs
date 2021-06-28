// Generated from definition io.k8s.api.extensions.v1beta1.HTTPIngressPath

/// HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressPath {
    /// Backend defines the referenced service endpoint to which the traffic will be forwarded to.
    pub backend: crate::api::extensions::v1beta1::IngressBackend,

    /// Path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional "path" part of a URL as defined by RFC 3986. Paths must begin with a '/'. When unspecified, all paths from incoming requests are matched.
    pub path: Option<String>,

    /// PathType determines the interpretation of the Path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types. Defaults to ImplementationSpecific.
    pub path_type: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for HTTPIngressPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_backend,
            Key_path,
            Key_path_type,
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
                            "backend" => Field::Key_backend,
                            "path" => Field::Key_path,
                            "pathType" => Field::Key_path_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPIngressPath;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HTTPIngressPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_backend: Option<crate::api::extensions::v1beta1::IngressBackend> = None;
                let mut value_path: Option<String> = None;
                let mut value_path_type: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_backend => value_backend = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path_type => value_path_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressPath {
                    backend: value_backend.ok_or_else(|| crate::serde::de::Error::missing_field("backend"))?,
                    path: value_path,
                    path_type: value_path_type,
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPIngressPath",
            &[
                "backend",
                "path",
                "pathType",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPIngressPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPIngressPath",
            1 +
            self.path.as_ref().map_or(0, |_| 1) +
            self.path_type.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backend", &self.backend)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.path_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pathType", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for HTTPIngressPath {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.",
          "properties": {
            "backend": crate::schema_ref_with_description(crate::api::extensions::v1beta1::IngressBackend::schema(), "Backend defines the referenced service endpoint to which the traffic will be forwarded to."),
            "path": {
              "description": "Path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional \"path\" part of a URL as defined by RFC 3986. Paths must begin with a '/'. When unspecified, all paths from incoming requests are matched.",
              "type": "string"
            },
            "pathType": {
              "description": "PathType determines the interpretation of the Path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is\n  done on a path element by element basis. A path element refers is the\n  list of labels in the path split by the '/' separator. A request is a\n  match for path p if every p is an element-wise prefix of p of the\n  request path. Note that if the last element of the path is a substring\n  of the last element in request path, it is not a match (e.g. /foo/bar\n  matches /foo/bar/baz, but does not match /foo/barbaz).\n* ImplementationSpecific: Interpretation of the Path matching is up to\n  the IngressClass. Implementations can treat this as a separate PathType\n  or treat it identically to Prefix or Exact path types.\nImplementations are required to support all path types. Defaults to ImplementationSpecific.",
              "type": "string"
            }
          },
          "required": [
            "backend"
          ],
          "type": "object"
        })
    }
}
