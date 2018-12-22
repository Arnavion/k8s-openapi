// Generated from definition io.k8s.kubernetes.pkg.api.v1.ComponentStatus

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentStatus {
    /// List of component conditions observed
    pub conditions: Option<Vec<crate::v1_7::kubernetes::pkg::api::v1::ComponentCondition>>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin /v1/ComponentStatus

// Generated from operation listCoreV1ComponentStatus

impl ComponentStatus {
    /// list objects of kind ComponentStatus
    ///
    /// Use [`ListCoreV1ComponentStatusResponse`](./enum.ListCoreV1ComponentStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_core_v1_component_status(
        optional: ListCoreV1ComponentStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListCoreV1ComponentStatusOptional {
            field_selector,
            include_uninitialized,
            label_selector,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/api/v1/componentstatuses?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ComponentStatus::list_core_v1_component_status`](./struct.ComponentStatus.html#method.list_core_v1_component_status)
#[derive(Debug, Default)]
pub struct ListCoreV1ComponentStatusOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`ComponentStatus::list_core_v1_component_status`](./struct.ComponentStatus.html#method.list_core_v1_component_status)
#[derive(Debug)]
pub enum ListCoreV1ComponentStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::api::v1::ComponentStatusList),
    Unauthorized,
    Other,
}

impl crate::Response for ListCoreV1ComponentStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListCoreV1ComponentStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListCoreV1ComponentStatusResponse::Unauthorized, 0)),
            _ => Ok((ListCoreV1ComponentStatusResponse::Other, 0)),
        }
    }
}

// Generated from operation readCoreV1ComponentStatus

impl ComponentStatus {
    /// read the specified ComponentStatus
    ///
    /// Use [`ReadCoreV1ComponentStatusResponse`](./enum.ReadCoreV1ComponentStatusResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ComponentStatus
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_core_v1_component_status(
        name: &str,
        optional: ReadCoreV1ComponentStatusOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadCoreV1ComponentStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/componentstatuses/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ComponentStatus::read_core_v1_component_status`](./struct.ComponentStatus.html#method.read_core_v1_component_status)
#[derive(Debug, Default)]
pub struct ReadCoreV1ComponentStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`ComponentStatus::read_core_v1_component_status`](./struct.ComponentStatus.html#method.read_core_v1_component_status)
#[derive(Debug)]
pub enum ReadCoreV1ComponentStatusResponse {
    Ok(crate::v1_7::kubernetes::pkg::api::v1::ComponentStatus),
    Unauthorized,
    Other,
}

impl crate::Response for ReadCoreV1ComponentStatusResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadCoreV1ComponentStatusResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadCoreV1ComponentStatusResponse::Unauthorized, 0)),
            _ => Ok((ReadCoreV1ComponentStatusResponse::Other, 0)),
        }
    }
}

// End /v1/ComponentStatus

impl crate::Resource for ComponentStatus {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "ComponentStatus"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for ComponentStatus {
    type Ty = crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ComponentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_conditions,
            Key_metadata,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "conditions" => Field::Key_conditions,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComponentStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ComponentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::v1_7::kubernetes::pkg::api::v1::ComponentCondition>> = None;
                let mut value_metadata: Option<crate::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ComponentStatus {
                    conditions: value_conditions,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "ComponentStatus",
            &[
                "apiVersion",
                "kind",
                "conditions",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ComponentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ComponentStatus",
            0 +
            2 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
