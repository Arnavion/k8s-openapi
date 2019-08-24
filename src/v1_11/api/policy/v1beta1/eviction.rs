// Generated from definition io.k8s.api.policy.v1beta1.Eviction

/// Eviction evicts a pod from its node subject to certain policies and safety constraints. This is a subresource of Pod.  A request to cause such an eviction is created by POSTing to .../pods/\<pod name\>/evictions.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Eviction {
    /// DeleteOptions may be provided
    pub delete_options: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::DeleteOptions>,

    /// ObjectMeta describes the pod that is being evicted.
    pub metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin policy/v1beta1/Eviction

// Generated from operation createCoreV1NamespacedPodEviction

impl Eviction {
    /// create eviction of a Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedPodEvictionResponse`]`>` constructor, or [`CreateNamespacedPodEvictionResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Eviction
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_namespaced_pod_eviction(
        name: &str,
        namespace: &str,
        body: &crate::v1_11::api::policy::v1beta1::Eviction,
        optional: CreateNamespacedPodEvictionOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedPodEvictionResponse>), crate::RequestError> {
        let CreateNamespacedPodEvictionOptional {
            pretty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/eviction?",
            name = crate::url::percent_encoding::percent_encode(name.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::url::percent_encoding::percent_encode(namespace.as_bytes(), crate::url::percent_encoding::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Eviction::create_namespaced_pod_eviction`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedPodEvictionOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateNamespacedPodEvictionResponse as Response>::try_from_parts` to parse the HTTP response body of [`Eviction::create_namespaced_pod_eviction`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedPodEvictionResponse {
    Ok(crate::v1_11::api::policy::v1beta1::Eviction),
    Created(crate::v1_11::api::policy::v1beta1::Eviction),
    Accepted(crate::v1_11::api::policy::v1beta1::Eviction),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedPodEvictionResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodEvictionResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodEvictionResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedPodEvictionResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateNamespacedPodEvictionResponse::Other(result), read))
            },
        }
    }
}

// End policy/v1beta1/Eviction

impl crate::Resource for Eviction {
    fn api_version() -> &'static str {
        "policy/v1beta1"
    }

    fn group() -> &'static str {
        "policy"
    }

    fn kind() -> &'static str {
        "Eviction"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl crate::Metadata for Eviction {
    type Ty = crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Eviction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_delete_options,
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
                            "deleteOptions" => Field::Key_delete_options,
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
            type Value = Eviction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Eviction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_delete_options: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::DeleteOptions> = None;
                let mut value_metadata: Option<crate::v1_11::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

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
                        Field::Key_delete_options => value_delete_options = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Eviction {
                    delete_options: value_delete_options,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "Eviction",
            &[
                "apiVersion",
                "kind",
                "deleteOptions",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Eviction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Eviction",
            2 +
            self.delete_options.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.delete_options {
            serde::ser::SerializeStruct::serialize_field(&mut state, "deleteOptions", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
