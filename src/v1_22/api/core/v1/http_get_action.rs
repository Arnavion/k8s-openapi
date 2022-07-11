// Generated from definition io.k8s.api.core.v1.HTTPGetAction

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPGetAction {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    pub host: Option<String>,

    /// Custom headers to set in the request. HTTP allows repeated headers.
    pub http_headers: Option<Vec<crate::api::core::v1::HTTPHeader>>,

    /// Path to access on the HTTP server.
    pub path: Option<String>,

    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: crate::apimachinery::pkg::util::intstr::IntOrString,

    /// Scheme to use for connecting to the host. Defaults to HTTP.
    pub scheme: Option<String>,

}

#[cfg(feature = "dsl")]
impl HTTPGetAction  {
    /// Set [`Self::host`]
    pub  fn host_set(&mut self, host: impl Into<Option<String>>) -> &mut Self {
        self.host = host.into(); self
    }

    pub  fn host(&mut self) -> &mut String {
        if self.host.is_none() { self.host = Some(Default::default()) }
        self.host.as_mut().unwrap()
    }

    /// Modify [`Self::host`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.host.is_none() { self.host = Some(Default::default()) };
        func(self.host.as_mut().unwrap()); self
    }


    /// Set [`Self::http_headers`]
    pub  fn http_headers_set(&mut self, http_headers: impl Into<Option<Vec<crate::api::core::v1::HTTPHeader>>>) -> &mut Self {
        self.http_headers = http_headers.into(); self
    }

    pub  fn http_headers(&mut self) -> &mut Vec<crate::api::core::v1::HTTPHeader> {
        if self.http_headers.is_none() { self.http_headers = Some(Default::default()) }
        self.http_headers.as_mut().unwrap()
    }

    /// Modify [`Self::http_headers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn http_headers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::HTTPHeader>)) -> &mut Self {
        if self.http_headers.is_none() { self.http_headers = Some(Default::default()) };
        func(self.http_headers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::http_headers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn http_headers_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::HTTPHeader)) -> &mut Self {
        if self.http_headers.is_none() {
            self.http_headers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.http_headers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::http_headers`]
    pub  fn http_headers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::HTTPHeader]>) -> &mut Self {
         if self.http_headers.is_none() { self.http_headers = Some(Vec::new()); }
         let http_headers = &mut self.http_headers.as_mut().unwrap();
         for item in other.borrow() {
             http_headers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::path`]
    pub  fn path_set(&mut self, path: impl Into<Option<String>>) -> &mut Self {
        self.path = path.into(); self
    }

    pub  fn path(&mut self) -> &mut String {
        if self.path.is_none() { self.path = Some(Default::default()) }
        self.path.as_mut().unwrap()
    }

    /// Modify [`Self::path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.path.is_none() { self.path = Some(Default::default()) };
        func(self.path.as_mut().unwrap()); self
    }


    /// Set [`Self::port`]
    pub  fn port_set(&mut self, port: impl Into<crate::apimachinery::pkg::util::intstr::IntOrString>) -> &mut Self {
        self.port = port.into(); self
    }

    pub  fn port(&mut self) -> &mut crate::apimachinery::pkg::util::intstr::IntOrString {
        &mut self.port
    }

    /// Modify [`Self::port`] with a `func`
    pub  fn port_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::util::intstr::IntOrString)) -> &mut Self {
        func(&mut self.port); self
    }


    /// Set [`Self::scheme`]
    pub  fn scheme_set(&mut self, scheme: impl Into<Option<String>>) -> &mut Self {
        self.scheme = scheme.into(); self
    }

    pub  fn scheme(&mut self) -> &mut String {
        if self.scheme.is_none() { self.scheme = Some(Default::default()) }
        self.scheme.as_mut().unwrap()
    }

    /// Modify [`Self::scheme`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scheme_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.scheme.is_none() { self.scheme = Some(Default::default()) };
        func(self.scheme.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for HTTPGetAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_host,
            Key_http_headers,
            Key_path,
            Key_port,
            Key_scheme,
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
                            "host" => Field::Key_host,
                            "httpHeaders" => Field::Key_http_headers,
                            "path" => Field::Key_path,
                            "port" => Field::Key_port,
                            "scheme" => Field::Key_scheme,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPGetAction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("HTTPGetAction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_host: Option<String> = None;
                let mut value_http_headers: Option<Vec<crate::api::core::v1::HTTPHeader>> = None;
                let mut value_path: Option<String> = None;
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_scheme: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_host => value_host = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_headers => value_http_headers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheme => value_scheme = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPGetAction {
                    host: value_host,
                    http_headers: value_http_headers,
                    path: value_path,
                    port: value_port.unwrap_or_default(),
                    scheme: value_scheme,
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPGetAction",
            &[
                "host",
                "httpHeaders",
                "path",
                "port",
                "scheme",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPGetAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPGetAction",
            1 +
            self.host.as_ref().map_or(0, |_| 1) +
            self.http_headers.as_ref().map_or(0, |_| 1) +
            self.path.as_ref().map_or(0, |_| 1) +
            self.scheme.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.host {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        if let Some(value) = &self.http_headers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "httpHeaders", value)?;
        }
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        if let Some(value) = &self.scheme {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scheme", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HTTPGetAction {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.HTTPGetAction".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("HTTPGetAction describes an action based on HTTP Get requests.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "host".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "httpHeaders".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Custom headers to set in the request. HTTP allows repeated headers.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::HTTPHeader>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "path".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Path to access on the HTTP server.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scheme".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Scheme to use for connecting to the host. Defaults to HTTP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "port".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
