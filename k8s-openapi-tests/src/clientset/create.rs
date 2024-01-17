use k8s_openapi::serde_json;

pub(crate) fn create_cluster<T>(
    body: &T,
) -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> super::ResponseBody<CreateResponse<T>>)
where
    T: serde::de::DeserializeOwned + serde::Serialize + k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/{url_path_segment}",
        api_version = T::API_VERSION,
        url_path_segment = T::URL_PATH_SEGMENT,
    );

    let request = http::Request::post(url);
    let request = request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
    let body = serde_json::to_vec(body).unwrap();
    let request = request.body(body).unwrap();
    (request, super::ResponseBody::new)
}

pub(crate) fn create_namespaced<T>(
    namespace: &str,
    body: &T,
) -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> super::ResponseBody<CreateResponse<T>>)
where
    T: serde::de::DeserializeOwned + serde::Serialize + k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope>,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/namespaces/{namespace}/{url_path_segment}",
        api_version = T::API_VERSION,
        namespace = percent_encoding::percent_encode(namespace.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
        url_path_segment = T::URL_PATH_SEGMENT,
    );

    let request = http::Request::post(url);
    let request = request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
    let body = serde_json::to_vec(body).unwrap();
    let request = request.body(body).unwrap();
    (request, super::ResponseBody::new)
}

#[derive(Debug)]
pub(crate) enum CreateResponse<T> where T: serde::de::DeserializeOwned {
    Ok(T),
    Created(T),
    Other(#[allow(dead_code)] Result<Option<serde_json::Value>, serde_json::Error>),
}

impl<T> super::Response for CreateResponse<T> where T: serde::de::DeserializeOwned {
    fn try_from_parts(status_code: reqwest::StatusCode, buf: &[u8]) -> Result<(Self, usize), super::ResponseError> {
        match status_code {
            reqwest::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                    Err(err) => return Err(super::ResponseError::Json(err)),
                };
                Ok((Self::Ok(result), buf.len()))
            },
            reqwest::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                    Err(err) => return Err(super::ResponseError::Json(err)),
                };
                Ok((Self::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((Self::Other(result), read))
            },
        }
    }
}
