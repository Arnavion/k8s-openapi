// Generated from definition io.k8s.api.networking.v1.HTTPIngressPath

/// HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressPath {
    /// backend defines the referenced service endpoint to which the traffic will be forwarded to.
    pub backend: crate::api::networking::v1::IngressBackend,

    /// path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional "path" part of a URL as defined by RFC 3986. Paths must begin with a '/' and must be present when using PathType with value "Exact" or "Prefix".
    pub path: Option<std::string::String>,

    /// pathType determines the interpretation of the path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types.
    pub path_type: std::string::String,
}

impl crate::DeepMerge for HTTPIngressPath {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.backend, other.backend);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.path_type, other.path_type);
    }
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HTTPIngressPath")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_backend: Option<crate::api::networking::v1::IngressBackend> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_path_type: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_backend => value_backend = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path_type => value_path_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressPath {
                    backend: value_backend.unwrap_or_default(),
                    path: value_path,
                    path_type: value_path_type.unwrap_or_default(),
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
            2 +
            self.path.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "backend", &self.backend)?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pathType", &self.path_type)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HTTPIngressPath {
    fn schema_name() -> std::string::String {
        "io.k8s.api.networking.v1.HTTPIngressPath".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "backend".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::IngressBackend>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("backend defines the referenced service endpoint to which the traffic will be forwarded to.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "path".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional \"path\" part of a URL as defined by RFC 3986. Paths must begin with a '/' and must be present when using PathType with value \"Exact\" or \"Prefix\".".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pathType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("pathType determines the interpretation of the path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is\n  done on a path element by element basis. A path element refers is the\n  list of labels in the path split by the '/' separator. A request is a\n  match for path p if every p is an element-wise prefix of p of the\n  request path. Note that if the last element of the path is a substring\n  of the last element in request path, it is not a match (e.g. /foo/bar\n  matches /foo/bar/baz, but does not match /foo/barbaz).\n* ImplementationSpecific: Interpretation of the Path matching is up to\n  the IngressClass. Implementations can treat this as a separate PathType\n  or treat it identically to Prefix or Exact path types.\nImplementations are required to support all path types.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "backend".into(),
                    "pathType".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
